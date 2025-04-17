// 1. Modules and re-exports
include!(concat!(env!("OUT_DIR"), "/mod_includes.rs"));
mod utils;

// 2. DetectorReportTemplate trait impls
include!(concat!(env!("OUT_DIR"), "/detector-report-templates.rs"));

// 3. Registration
include!(concat!(env!("OUT_DIR"), "/register.rs"));
