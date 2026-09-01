// tests/audit/remediation/test_execute_package_manager_upgrade.rs

use miette::Result;
use rustywoof::audit::remediation::execute_package_manager_upgrade::CommandRunner;
use rustywoof::audit::remediation::resolve_remediation_context::RemediationCommand;

struct MockCommandRunner {
    should_succeed: bool,
}

impl CommandRunner for MockCommandRunner {
    fn run(&self, _cmd: &RemediationCommand) -> Result<bool> {
        Ok(self.should_succeed)
    }
}

#[test]
fn test_command_runner_reports_success() {
    let runner = MockCommandRunner {
        should_succeed: true,
    };
    let cmd = RemediationCommand {
        binary: "npm".to_string(),
        args: vec!["install".to_string(), "lodash@4.17.21".to_string()],
    };

    let result = runner.run(&cmd).expect("Mock runner execution failed");
    assert!(result, "Expected runner to report success");
}

#[test]
fn test_command_runner_reports_failure() {
    let runner = MockCommandRunner {
        should_succeed: false,
    };
    let cmd = RemediationCommand {
        binary: "cargo".to_string(),
        args: vec!["add".to_string(), "serde@1.0.190".to_string()],
    };

    let result = runner.run(&cmd).expect("Mock runner execution failed");
    assert!(!result, "Expected runner to report failure");
}
