// tests/audit/remediation/test_resolve_remediation_context.rs

use rustywoof::audit::remediation::resolve_remediation_context::resolve_context;

#[test]
fn test_resolve_context_auto_detects_existing_package() {
    let cmd = resolve_context("miette", "5.10.0")
        .expect("Failed to resolve cargo remediation context for known package");

    assert_eq!(cmd.binary, "cargo");
    assert_eq!(
        cmd.args,
        vec!["add".to_string(), "miette@5.10.0".to_string()]
    );
}

#[test]
fn test_resolve_context_fails_for_missing_package() {
    let result = resolve_context("nonexistent-package-xyz", "1.0.0");
    assert!(
        result.is_err(),
        "Should fail cleanly when the package is not found in the local workspace lockfiles"
    );
}
