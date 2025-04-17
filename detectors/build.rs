use serde_yaml::Value;
use std::path::Path;
use std::{env, fs};

fn main() {
    let metadata_dir = Path::new("metadata");
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let dest_path = Path::new(&out_dir).join("detector-report-templates.rs");

    let mut generated = String::new();

    if metadata_dir.exists() && metadata_dir.is_dir() {
        generated.push_str("use midnight_security_detectors_sdk::DetectorReportTemplate;\n");
        for entry in fs::read_dir(metadata_dir).expect("Failed to read metadata directory") {
            let entry = entry.expect("Failed to read directory entry");
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("yml") {
                println!("cargo:rerun-if-changed={}", path.display());

                let content = fs::read_to_string(&path)
                    .unwrap_or_else(|_| panic!("Failed to read file {}", path.display()));
                let yaml: Value = serde_yaml::from_str(&content)
                    .unwrap_or_else(|_| panic!("Failed to parse YAML in file {}", path.display()));

                let metadata = &yaml["metadata"];
                let id = metadata["id"]
                    .as_str()
                    .expect("metadata.id is missing or not a string");
                let type_name = to_type_name(id);

                let uid = metadata["uid"]
                    .as_str()
                    .expect("metadata.uid is missing or not a string");

                let description = metadata["description"].as_str().unwrap_or("");
                let report = &metadata["report"];
                let severity = report["severity"].as_str().unwrap_or("note");
                let tags = report["tags"]
                    .as_sequence()
                    .unwrap_or(&Vec::new())
                    .iter()
                    .filter_map(|tag| tag.as_str().map(|s| format!("\"{}\".to_string()", s)))
                    .collect::<Vec<_>>()
                    .join(",");

                let template = &metadata["report"]["template"];
                let template_yaml =
                    serde_yaml::to_string(&template).unwrap_or_else(|_| String::from("{}"));
                let title = template["title"].as_str().unwrap_or_default();
                let opening = template["body-list-item-intro"]
                    .as_str()
                    .unwrap_or_default();
                let body_list_item = template["body-list-item"].as_str().unwrap_or_default();
                let closing = template["closing"].as_str().unwrap_or_default();
                let type_def = format!(
                    r#"
impl DetectorReportTemplate for {type_name} {{
    fn id(&self) -> String {{
        "{id}".to_string()
    }}
    fn uid(&self) -> String {{
        "{uid}".to_string()
    }}
    fn description(&self) -> String {{
        "{description}".to_string()
    }}
    fn severity(&self) -> String {{
        "{severity}".to_string()
    }}
    fn tags(&self) -> Vec<String> {{
        vec![{tags}]
    }}
    fn title_single_instance(&self) -> String {{
        "{title}".to_string()
    }}
    fn title_multiple_instance(&self) -> String {{
        "{title}".to_string()
    }}
    fn opening(&self) -> String {{
        "{opening}".to_string()
    }}
    fn body_single_file_single_instance(&self) -> String {{
        String::new()
    }}
    fn body_single_file_multiple_instance(&self) -> String {{
        String::new()
    }}
    fn body_multiple_file_multiple_instance(&self) -> String {{
        String::new()
    }}
    fn body_list_item_single_file(&self) -> String {{
        "{body_list_item}".to_string()
    }}
    fn body_list_item_multiple_file(&self) -> String {{
        "{body_list_item}".to_string()
    }}
    fn closing(&self) -> String {{
        "{closing}".to_string()
    }}
    fn template(&self) -> String {{
        "{template_yaml}".to_string()
    }}
}}
"#,
                    type_name = type_name,
                    id = escape_rust_string(id),
                    uid = escape_rust_string(uid),
                    description = escape_rust_string(description),
                    severity = escape_rust_string(severity),
                    tags = tags,
                    title = escape_rust_string(title),
                    opening = escape_rust_string(opening),
                    body_list_item = escape_rust_string(body_list_item),
                    closing = escape_rust_string(closing),
                    template_yaml = escape_rust_string(&template_yaml),
                );
                generated.push_str(&type_def);
            }
        }
    } else {
        println!("cargo:warning=metadata directory not found.");
    }

    fs::write(&dest_path, generated)
        .unwrap_or_else(|_| panic!("Failed to write to {}", dest_path.display()));

    // --- Generate mod_includes.rs to register mods ---
    let src_dir = Path::new("src");
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let mod_file_path = Path::new(&out_dir).join("mod_includes.rs");

    let mut mods = String::new();
    let mut detector_type_names = Vec::new();

    for entry in fs::read_dir(src_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("rs") {
            let file_name = path.file_name().unwrap().to_str().unwrap();
            if file_name != "lib.rs" && file_name != "utils.rs" {
                let mod_name = file_name.trim_end_matches(".rs");
                let type_name = to_type_name(mod_name);
                mods.push_str(&format!(
                    "pub mod {} {{ include!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/src/{}.rs\")); }}\n",
                    mod_name, mod_name
                ));
                mods.push_str(&format!("pub use {}::{};\n", mod_name, type_name));
                detector_type_names.push(type_name);
            }
        }
    }

    fs::write(&mod_file_path, mods).unwrap();

    // --- Generate register.rs for all_detectors() ---
    let register_path = Path::new(&out_dir).join("register.rs");
    let mut register_code = String::from(
        "pub fn all_detectors() -> Vec<midnight_security_detectors_sdk::MidnightDetector> {\n    vec![\n",
    );
    for type_name in &detector_type_names {
        register_code.push_str(&format!("        Box::new({}),\n", type_name));
    }
    register_code.push_str("    ]\n}\n");
    fs::write(&register_path, register_code).unwrap();
}

fn to_type_name(id: &str) -> String {
    id.split(|c: char| !c.is_alphanumeric())
        .filter(|s| !s.is_empty())
        .map(|s| {
            let mut chars = s.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<String>()
}

fn escape_rust_string(s: &str) -> String {
    s.replace("\\", "\\\\")
        .replace("\"", "\\\"")
        .replace("\n", "\\n")
        .replace("\r", "")
}
