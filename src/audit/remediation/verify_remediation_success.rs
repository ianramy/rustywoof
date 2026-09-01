// src/audit/remediation/verify_remediation_success.rs

use miette::Result;

pub trait RemediationValidator {
    fn verify(&self, package: &str, target_version: &str) -> Result<bool>;
}

pub struct LockfileValidator;

impl RemediationValidator for LockfileValidator {
    fn verify(&self, package: &str, target_version: &str) -> Result<bool> {
        let (deps, _) = crate::audit::parsers::extract_dependencies()?;
        let is_fixed = deps
            .iter()
            .any(|(p, v, _)| p == package && v == target_version);
        Ok(is_fixed)
    }
}
