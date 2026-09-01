// src/ui/detect_headless_environment.rs

use std::env;

pub fn is_headless() -> bool {
    is_headless_env(env::vars())
}

pub fn is_headless_env<I, K, V>(vars: I) -> bool
where
    I: IntoIterator<Item = (K, V)>,
    K: AsRef<str>,
{
    for (key, _) in vars {
        let k = key.as_ref();
        if k == "CI" || k == "GITHUB_ACTIONS" || k == "GITLAB_CI" || k == "NO_COLOR" {
            return true;
        }
    }
    false
}
