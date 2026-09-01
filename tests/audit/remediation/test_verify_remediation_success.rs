// tests/audit/remediation/test_verify_remediation_success.rs

use miette::Result;
use rustywoof::audit::remediation::verify_remediation_success::RemediationValidator;

pub struct MockValidator {
    pub is_fixed: bool,
}

impl RemediationValidator for MockValidator {
    fn verify(&self, _package: &str, _target_version: &str) -> Result<bool> {
        Ok(self.is_fixed)
    }
}

#[test]
fn test_validator_confirms_fix() {
    let validator = MockValidator { is_fixed: true };
    assert!(
        validator.verify("lodash", "4.17.21").unwrap(),
        "Expected validator to confirm fix"
    );
}

#[test]
fn test_validator_reports_still_vulnerable() {
    let validator = MockValidator { is_fixed: false };
    assert!(
        !validator.verify("lodash", "4.17.21").unwrap(),
        "Expected validator to reject unfixed state"
    );
}
