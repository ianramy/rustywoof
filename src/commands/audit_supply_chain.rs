// src/commands/audit_supply_chain.rs

use crate::audit;
use miette::Result;

pub fn run(dev: bool, prod: bool, audit_level: Option<String>, interactive: bool) -> Result<bool> {
    audit::audit_dependencies(dev, prod, audit_level, interactive)
}
