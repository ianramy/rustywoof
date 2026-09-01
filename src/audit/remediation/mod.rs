// src/audit/remediation/mod.rs

pub mod execute_package_manager_upgrade;
pub mod orchestrate_vulnerability_remediation;
pub mod resolve_remediation_context;
pub mod verify_remediation_success;

use execute_package_manager_upgrade::SystemCommandRunner;
use miette::Result;
use orchestrate_vulnerability_remediation::orchestrate_remediation;
use verify_remediation_success::LockfileValidator;

pub fn remediate_vulnerability(package: &str, target_version: &str) -> Result<()> {
    let runner = SystemCommandRunner;
    let validator = LockfileValidator;
    orchestrate_remediation(package, target_version, &runner, &validator)
}
