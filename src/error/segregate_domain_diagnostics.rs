// src/error/segregate_domain_diagnostics.rs

use miette::Diagnostic;
use std::io;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum ConfigDiagnostic {
    #[error("Perimeter Configuration Failure")]
    #[diagnostic(
        code(woof::config::io_error),
        help("Verify that you have write permissions in the current directory.")
    )]
    ConfigWriteFailed(#[source] io::Error),

    #[error("Invalid Configuration Syntax")]
    #[diagnostic(
        code(woof::config::parse_error),
        help("The .woof.toml file contains invalid formatting. Please check your syntax.")
    )]
    ConfigParseFailed(#[source] toml::de::Error),

    #[error("Configuration Serialization Failed")]
    #[diagnostic(
        code(woof::config::serialize_error),
        help("Failed to write the default configuration to string formatting.")
    )]
    ConfigSerializeFailed(#[source] toml::ser::Error),

    #[error("Configuration Validation Failed: {0}")]
    #[diagnostic(
        code(woof::config::validation_error),
        help("Review your configuration overrides and correct the invalid parameters.")
    )]
    ConfigValidationFailed(String),
}

#[derive(Error, Debug, Diagnostic)]
pub enum GitDiagnostic {
    #[error("Perimeter Guard Hook Failure")]
    #[diagnostic(
        code(woof::git::hook_error),
        help("Failed to install the pre-commit hook. Ensure this is a valid Git repository.")
    )]
    GitHookFailed(#[source] io::Error),
}

#[derive(Error, Debug, Diagnostic)]
pub enum AuditDiagnostic {
    #[error("Perimeter Audit Failed: No lockfiles detected.")]
    #[diagnostic(
        code(woof::audit::no_lockfiles),
        help("Ensure you are running this command in a project root.")
    )]
    NoLockfilesFound,

    #[error("Lockfile Corruption Detected: {file_name}")]
    #[diagnostic(
        code(woof::audit::corrupt_lockfile),
        help("Failed to parse the lockfile. Ensure it is well-formed.")
    )]
    LockfileParseError {
        file_name: String,
        #[source]
        source: anyhow::Error,
    },
}

#[derive(Error, Debug, Diagnostic)]
#[error(transparent)]
#[diagnostic(transparent)]
pub enum WatchdogError {
    Config(#[from] ConfigDiagnostic),
    Git(#[from] GitDiagnostic),
    Audit(#[from] AuditDiagnostic),
}
