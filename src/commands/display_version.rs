// src/commands/display_version.rs

pub fn run() {
    println!("{}", env!("CARGO_PKG_VERSION"));
}
