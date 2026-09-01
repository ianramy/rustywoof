// tests/terminal/test_detect_tty_capabilities.rs

use rustywoof::terminal::detect_tty_capabilities;

#[test]
fn test_tty_detection_is_safe_to_call() {
    let _stdout_is_tty = detect_tty_capabilities::is_stdout_interactive();
    let _stdin_is_tty = detect_tty_capabilities::is_stdin_interactive();
}
