use std::{cell::RefCell, collections::HashMap};

use midnight_security_rules_sdk::{
    codebase::{Codebase, SealedState},
    Rule,
};

pub struct AssertionErrorMessageConsistency;

#[rustfmt::skip]
impl Rule for AssertionErrorMessageConsistency {
    fn name(&self) -> String {
        "Assertion Error Message Consistency".to_string()
    }

    fn description(&self) -> String {
        "Without a clear error message, debugging failures in this critical admin-setting function becomes difficult.".to_string()
    }

    fn check(&self, codebase: &RefCell<Codebase<SealedState>>) -> Option<HashMap<String, Vec<(usize, usize)>>> {
        let codebase = codebase.borrow();
        todo!("Implement this function")
    }
}

pub fn all_rules() -> Vec<Box<dyn Rule>> {
    vec![Box::new(AssertionErrorMessageConsistency)]
}

#[cfg(test)]
mod tests {
    use midnight_security_rules_sdk::build_codebase;

    use super::*;

    #[test]
    fn test_all_rules() {
        let rule = AssertionErrorMessageConsistency;
        let src = "export circuit set_admin(new_admin: Bytes<32>): [] {
            const current_proof = generate_key_proof(sigCounter as Field as Bytes<32>);
            // Vulnerability: Assertion lacks an error message. If this fails, it's unclear why.
            assert admin == pad(32, \"\") || admin == current_proof;
            admin = new_admin;
            return [];
        }";
        let mut data = HashMap::new();
        data.insert("test.compact".to_string(), src.to_string());
        let codebase = build_codebase(data).unwrap();
        let result = rule.check(&codebase);
        assert!(result.is_some());
    }
}
