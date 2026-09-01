// tests/ui/test_prompt_user_remediation.rs

use rustywoof::ui::prompt_user_remediation;
use std::io::Cursor;

#[test]
fn test_prompt_returns_true_on_yes() {
    let mut input = Cursor::new(b"y\n");
    let mut output = Vec::new();

    let result =
        prompt_user_remediation::ask_confirm("Patch package?", false, &mut input, &mut output)
            .expect("Failed to execute prompt");

    assert!(result, "Prompt should return true when user inputs 'y'");
}

#[test]
fn test_prompt_auto_declines_in_headless_mode() {
    let mut input = Cursor::new(b"y\n"); // Even if 'y' is buffered...
    let mut output = Vec::new();

    let result =
        prompt_user_remediation::ask_confirm("Patch package?", true, &mut input, &mut output)
            .expect("Failed to execute prompt");

    assert!(
        !result,
        "Prompt must safely auto-decline in headless environments to prevent CI hangs"
    );
}
