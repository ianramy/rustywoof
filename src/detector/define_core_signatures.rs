// src/detector/define_core_signatures.rs

use regex::Regex;
use std::sync::LazyLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
}

pub struct SecretRule {
    pub name: &'static str,
    pub prefix: &'static str,
    pub pattern: Regex,
    pub error_code: &'static str,
    pub remediation: &'static str,
    pub severity: Severity,
}

pub static CORE_RULES: LazyLock<Vec<SecretRule>> = LazyLock::new(|| {
    vec![
        SecretRule {
            name: "AWS Access Key",
            prefix: "AKIA",
            pattern: Regex::new(r"^AKIA[0-9A-Z]{16}").unwrap(),
            error_code: "woof::aws::access_key",
            remediation: "Invalidate this key in AWS IAM immediately. Rotate credentials.",
            severity: Severity::Critical,
        },
        SecretRule {
            name: "Google Cloud API Key",
            prefix: "AIza",
            pattern: Regex::new(r"^AIza[0-9A-Za-z\-_]{35}").unwrap(),
            error_code: "woof::gcp::api_key",
            remediation: "Restrict or regenerate this API key in the Google Cloud Console.",
            severity: Severity::High,
        },
        SecretRule {
            name: "GitHub Personal Access Token",
            prefix: "ghp_",
            pattern: Regex::new(r"^gh[p|u|s|o|r]_[A-Za-z0-9_]{36}").unwrap(),
            error_code: "woof::github::pat",
            remediation: "Revoke this token via GitHub Developer Settings.",
            severity: Severity::Critical,
        },
        SecretRule {
            name: "JSON Web Token (JWT)",
            prefix: "eyJ",
            pattern: Regex::new(r"^eyJ[A-Za-z0-9_-]{10,}\.[A-Za-z0-9_-]{10,}\.[A-Za-z0-9_-]{10,}")
                .unwrap(),
            error_code: "woof::auth::jwt",
            remediation: "Do not hardcode JWTs. Ensure they do not grant administrative access.",
            severity: Severity::Medium,
        },
    ]
});
