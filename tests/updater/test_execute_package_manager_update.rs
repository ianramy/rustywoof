// tests/updater/test_execute_package_manager_update.rs

use miette::Result;
use rustywoof::updater::detect_package_manager_context::PackageManager;
use rustywoof::updater::execute_package_manager_update::{
    UpdateCommandRunner, run_automatic_update,
};

struct MockRunner {
    should_succeed: bool,
}

impl UpdateCommandRunner for MockRunner {
    fn run(&self, _program: &str, _args: &[&str]) -> Result<bool> {
        Ok(self.should_succeed)
    }
}

#[test]
fn test_executor_cascades_on_failure() {
    let runner = MockRunner {
        should_succeed: false,
    };
    let result = run_automatic_update(PackageManager::Npm, &runner);
    assert!(
        result.is_err(),
        "Expected cascading failure across candidate commands"
    );
}

#[test]
fn test_executor_succeeds_when_runner_returns_true() {
    let runner = MockRunner {
        should_succeed: true,
    };
    let result = run_automatic_update(PackageManager::Cargo, &runner);
    assert!(result.is_ok(), "Expected success");
}
