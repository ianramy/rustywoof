// src/error/map_system_exit_codes.rs

use crate::error::segregate_domain_diagnostics::{
    AuditDiagnostic, ConfigDiagnostic, GitDiagnostic, WatchdogError,
};

pub trait SystemExitCode {
    fn exit_code(&self) -> i32;
}

impl SystemExitCode for ConfigDiagnostic {
    fn exit_code(&self) -> i32 {
        2
    }
}

impl SystemExitCode for GitDiagnostic {
    fn exit_code(&self) -> i32 {
        3
    }
}

impl SystemExitCode for AuditDiagnostic {
    fn exit_code(&self) -> i32 {
        4
    }
}

impl SystemExitCode for WatchdogError {
    fn exit_code(&self) -> i32 {
        match self {
            WatchdogError::Config(e) => e.exit_code(),
            WatchdogError::Git(e) => e.exit_code(),
            WatchdogError::Audit(e) => e.exit_code(),
        }
    }
}
