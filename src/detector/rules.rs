// src/detector/rules.rs

use aho_corasick::AhoCorasick;
use regex::Regex;
use std::sync::LazyLock;

pub struct SecretRule {
    pub name: &'static str,
    pub prefix: &'static str,
    pub pattern: Regex,
    pub error_code: &'static str,
    pub remediation: &'static str,
}

pub static CORE_RULES: LazyLock<Vec<SecretRule>> = LazyLock::new(|| {
    vec![
        // --- Cloud Providers ---
        SecretRule {
            name: "AWS Access Key",
            prefix: "AKIA",
            pattern: Regex::new(r"AKIA[0-9A-Z]{16}").unwrap(),
            error_code: "woof::aws::access_key",
            remediation: "Invalidate this key in AWS IAM immediately. Rotate credentials.",
        },
        SecretRule {
            name: "Google Cloud API Key",
            prefix: "AIza",
            pattern: Regex::new(r"AIza[0-9A-Za-z\\-_]{35}").unwrap(),
            error_code: "woof::gcp::api_key",
            remediation: "Restrict or regenerate this API key in the Google Cloud Console.",
        },
        // --- Infrastructure & CI/CD ---
        SecretRule {
            name: "GitHub Personal Access Token",
            prefix: "ghp_",
            pattern: Regex::new(r"gh[p|u|s|o|r]_[A-Za-z0-9_]{36}").unwrap(),
            error_code: "woof::github::pat",
            remediation: "Revoke this token via GitHub Developer Settings to prevent repository compromise.",
        },
        SecretRule {
            name: "Slack Webhook / Bot Token",
            prefix: "xox",
            pattern: Regex::new(r"xox[baprs]-[0-9a-zA-Z]{10,48}").unwrap(),
            error_code: "woof::slack::token",
            remediation: "Revoke token in Slack API dashboard. Exposed webhooks can lead to internal phishing.",
        },
        // --- Databases & Auth ---
        SecretRule {
            name: "Supabase API Key",
            prefix: "sbp_",
            pattern: Regex::new(r"sbp_[a-zA-Z0-9]{40}").unwrap(),
            error_code: "woof::supabase::api_key",
            remediation: "Rotate the Supabase Service Role or Anon key in your project settings.",
        },
        SecretRule {
            name: "JSON Web Token (JWT)",
            prefix: "eyJ",
            // Matches standard JWT structure: Header.Payload.Signature
            pattern: Regex::new(r"eyJ[A-Za-z0-9_-]{10,}\.[A-Za-z0-9_-]{10,}\.[A-Za-z0-9_-]{10,}")
                .unwrap(),
            error_code: "woof::auth::jwt",
            remediation: "Evaluate if this JWT contains sensitive PII or grants administrative access. Do not hardcode JWTs.",
        },
        // --- Modern Frameworks (Vite / Next.js / React) ---
        SecretRule {
            name: "Framework Environment Variable",
            prefix: "VITE_",
            pattern: Regex::new(
                r"(VITE_|NEXT_PUBLIC_|REACT_APP_)[A-Z_]+\s*=\s*['\x22]?[^'\x22\n]+['\x22]?",
            )
            .unwrap(),
            error_code: "woof::framework::env_leak",
            remediation: "Ensure this variable does not contain private cryptographic keys. Move to a .env file and ensure it is heavily guarded.",
        },
    ]
});

pub static RULE_MATCHER: LazyLock<AhoCorasick> = LazyLock::new(|| {
    let prefixes: Vec<&str> = CORE_RULES.iter().map(|rule| rule.prefix).collect();
    AhoCorasick::builder()
        .build(prefixes)
        .expect("Failed to compile Aho-Corasick automaton")
});
