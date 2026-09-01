// tests/error/test_segregate_domain_diagnostics.rs

use rustywoof::error::segregate_domain_diagnostics::{
    AuditDiagnostic, ConfigDiagnostic, WatchdogError,
};
use std::io;

#[test]
fn test_domain_errors_wrap_into_unified_watchdog_error() {
    let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "access denied");
    let config_err = ConfigDiagnostic::ConfigWriteFailed(io_err);

    let wrapped_error: WatchdogError = config_err.into();

    let error_string = wrapped_error.to_string();
    assert!(
        error_string.contains("Perimeter Configuration Failure"),
        "Top-level error did not correctly transparently display the domain error: {}",
        error_string
    );

    let audit_err = AuditDiagnostic::NoLockfilesFound;
    let wrapped_audit: WatchdogError = audit_err.into();
    assert!(
        wrapped_audit.to_string().contains("Perimeter Audit Failed"),
        "Top-level error failed to wrap AuditDiagnostic."
    );
}
