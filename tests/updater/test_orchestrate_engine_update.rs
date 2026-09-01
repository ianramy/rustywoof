// tests/updater/test_orchestrate_engine_update.rs

use miette::Result;
use rustywoof::updater::execute_package_manager_update::UpdateCommandRunner;
use rustywoof::updater::orchestrate_engine_update::orchestrate_update;
use rustywoof::updater::replace_host_executable::ExecutableReplacer;
use std::path::Path;

struct MockRunner {
    success: bool,
}
impl UpdateCommandRunner for MockRunner {
    fn run(&self, _p: &str, _a: &[&str]) -> Result<bool> {
        Ok(self.success)
    }
}

struct MockReplacer {
    replaced: std::cell::Cell<bool>,
}
impl ExecutableReplacer for MockReplacer {
    fn replace(&self, _p: &Path) -> Result<()> {
        self.replaced.set(true);
        Ok(())
    }
}

#[test]
fn test_orchestrator_routes_to_package_manager() {
    let runner = MockRunner { success: true };
    let replacer = MockReplacer {
        replaced: std::cell::Cell::new(false),
    };

    let result = orchestrate_update(
        "/usr/local/lib/node_modules/woof",
        "http://dummy",
        "1.0",
        "x86_64",
        &runner,
        &replacer,
    );

    assert!(result.is_ok());
    assert!(
        !replacer.replaced.get(),
        "Should not attempt self-replace during package manager update"
    );
}
