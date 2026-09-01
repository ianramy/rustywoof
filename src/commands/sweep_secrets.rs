// src/commands/sweep_secrets.rs

use crate::scanner;

pub fn run(path: &str) {
    scanner::execute_sweep(path, false);
}
