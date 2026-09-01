// src/commands/enforce_strict_check.rs

use crate::scanner;

pub fn run(path: &str) -> bool {
    scanner::execute_sweep(path, true)
}
