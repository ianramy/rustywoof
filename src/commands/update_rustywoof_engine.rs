// src/commands/update_rustywoof_engine.rs

use crate::updater;
use miette::Result;

pub fn run() -> Result<()> {
    updater::execute_update()
}
