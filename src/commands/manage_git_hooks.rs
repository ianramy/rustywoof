// src/commands/manage_git_hooks.rs

use crate::commands::HookAction;
use crate::git::{manage_lifecycle_hooks, resolve_repository_context};
use miette::{IntoDiagnostic, Result};
use std::env;

pub fn run(action: &HookAction) -> Result<()> {
    let current_dir = env::current_dir().into_diagnostic()?;
    let repo_root_result = resolve_repository_context::find_git_root(&current_dir);
    let hook_type = manage_lifecycle_hooks::HookType::PreCommit;

    match action {
        HookAction::Install => {
            let repo_root = repo_root_result?;
            manage_lifecycle_hooks::deploy_guard(&repo_root, hook_type)?;
        }
        HookAction::Remove => {
            if let Ok(repo_root) = repo_root_result {
                manage_lifecycle_hooks::remove_guard(&repo_root, hook_type)?;
            } else {
                println!("[INFO] No repository found, skipping guard removal.");
            }
        }
    }

    Ok(())
}
