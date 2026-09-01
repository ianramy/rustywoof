// src/commands/remediate_package.rs

use crate::audit;
use miette::Result;

pub fn run(package: &str, version: &str) -> Result<()> {
    audit::remediate_vulnerability(package, version)
}
