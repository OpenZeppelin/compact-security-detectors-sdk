// Re-export the detectors
pub use array_loop_bound_check::ArrayLoopBoundCheck;
pub use assertion_error_message_verbose::AssertionErrorMessageVerbose;

include!(concat!(env!("OUT_DIR"), "/detector-report-templates.rs"));

// Module declarations
mod array_loop_bound_check;
mod assertion_error_message_verbose;
mod utils;

detectors! {}
