// src/audit/remediation/execute_package_manager_upgrade.rs

use crate::audit::remediation::resolve_remediation_context::RemediationCommand;
use miette::{IntoDiagnostic, Result};
use std::process::{Command, Stdio};

pub trait CommandRunner {
    fn run(&self, cmd: &RemediationCommand) -> Result<bool>;
}

pub struct SystemCommandRunner;

impl CommandRunner for SystemCommandRunner {
    fn run(&self, cmd: &RemediationCommand) -> Result<bool> {
        let status = Command::new(&cmd.binary)
            .args(&cmd.args)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .into_diagnostic()?;

        Ok(status.success())
    }
}
