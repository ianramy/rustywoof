// tests/ui/test_orchestrate_progress_bars.rs

use rustywoof::ui::orchestrate_progress_bars::ProgressOrchestrator;

#[test]
fn test_orchestrator_handles_headless_mode() {
    let orchestrator = ProgressOrchestrator::new(true);
    let pb = orchestrator.add_byte_tracker(1024);

    assert!(
        pb.is_hidden(),
        "Progress bar should be hidden in headless mode"
    );
}

#[test]
fn test_orchestrator_handles_interactive_mode() {
    let orchestrator = ProgressOrchestrator::new(false);
    let pb = orchestrator.add_byte_tracker(1024);

    assert!(
        !pb.is_hidden(),
        "Progress bar should be visible in interactive mode"
    );
}
