// tests/terminal/test_generate_shell_completions.rs

use clap::Command;
use clap_complete::Shell;
use rustywoof::terminal::generate_shell_completions;

#[test]
fn test_generates_completions_to_memory_buffer() {
    let mut cmd = Command::new("woof");
    let mut buffer = Vec::new();

    generate_shell_completions::generate_to_writer(Shell::Bash, &mut cmd, "woof", &mut buffer);

    let output = String::from_utf8(buffer).expect("Generated invalid UTF-8 completion script");

    assert!(
        output.contains("_woof()"),
        "Bash completion script is missing the primary hook function."
    );
}
