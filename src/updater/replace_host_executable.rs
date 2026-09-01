// src/updater/replace_host_executable.rs

use miette::{Result, miette};
use std::path::Path;

pub trait ExecutableReplacer {
    fn replace(&self, new_executable: &Path) -> Result<()>;
}

pub struct SystemReplacer;

impl ExecutableReplacer for SystemReplacer {
    fn replace(&self, new_executable: &Path) -> Result<()> {
        #[cfg(unix)]
        {
            use std::fs;
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(new_executable)
                .map_err(|e| miette!("Failed to read metadata: {}", e))?
                .permissions();
            perms.set_mode(0o755);
            fs::set_permissions(new_executable, perms)
                .map_err(|e| miette!("Failed to set permissions: {}", e))?;
        }

        self_replace::self_replace(new_executable)
            .map_err(|e| miette!("Failed to safely replace executable: {}", e))?;

        Ok(())
    }
}
