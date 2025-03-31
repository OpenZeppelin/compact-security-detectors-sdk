use std::collections::HashMap;

use midnight_security_rules::all_rules;
use midnight_security_rules_sdk::{build_codebase, Rule};

fn main() {
    let contract_content = r#"export circuit set_admin(new_admin: Bytes<32>): [] {
            const current_proof = generate_key_proof(sigCounter as Field as Bytes<32>);
            assert admin == pad(32, "") || admin == current_proof;
            admin = new_admin;
            return [];
        }"#;

    let mut data = HashMap::new();
    data.insert("test.compact".to_string(), contract_content.to_string());
    let codebase = build_codebase(data).unwrap();
    let mut rules = all_rules();
    rules.extend(custom_rules());
    for rule in rules {
        let rule_result = rule.check(&codebase);
        if let Some(errors) = rule_result {
            for (container_name, locations) in errors.iter() {
                for (line, col) in locations.iter() {
                    println!(
                        "[{}]: in {container_name} detected an error at [{line}:{col}]",
                        rule.name()
                    );
                }
            }
        }
    }
}

#[allow(clippy::let_and_return, unused_mut)]
fn custom_rules() -> Vec<Box<dyn Rule>> {
    let mut rules: Vec<Box<dyn Rule>> = Vec::new();
    //Import and add your rules here
    rules
}
