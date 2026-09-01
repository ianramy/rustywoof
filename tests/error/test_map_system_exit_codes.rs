// tests/error/test_map_system_exit_codes.rs

use rustywoof::error::map_system_exit_codes::SystemExitCode;
use rustywoof::error::segregate_domain_diagnostics::{
    AuditDiagnostic, ConfigDiagnostic, GitDiagnostic,
};
use std::io;

#[test]
fn test_exit_codes_map_correctly_to_domains() {
    let config_err = ConfigDiagnostic::ConfigWriteFailed(io::Error::new(
        io::ErrorKind::PermissionDenied,
        "denied",
    ));

    let git_err =
        GitDiagnostic::GitHookFailed(io::Error::new(io::ErrorKind::NotFound, "not found"));

    let audit_err = AuditDiagnostic::NoLockfilesFound;

    assert_eq!(
        config_err.exit_code(),
        2,
        "ConfigDiagnostic should yield exit code 2"
    );

    assert_eq!(
        git_err.exit_code(),
        3,
        "GitDiagnostic should yield exit code 3"
    );

    assert_eq!(
        audit_err.exit_code(),
        4,
        "AuditDiagnostic should yield exit code 4"
    );
}
