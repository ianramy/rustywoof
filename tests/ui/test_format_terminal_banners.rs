// tests/ui/test_format_terminal_banners.rs

use rustywoof::ui::format_terminal_banners;

#[test]
fn test_formats_banners_based_on_environment() {
    let msg = "System online";

    let headless_info = format_terminal_banners::format_info(msg, true);
    assert_eq!(headless_info, "[INFO] System online");

    let headless_crit = format_terminal_banners::format_critical(msg, true);
    assert_eq!(headless_crit, "[CRITICAL] System online");

    let interactive_info = format_terminal_banners::format_info(msg, false);
    assert_eq!(interactive_info, "\x1b[90m[INFO]\x1b[0m System online");

    let interactive_crit = format_terminal_banners::format_critical(msg, false);

    assert_eq!(
        interactive_crit,
        "\x1b[1;31m[CRITICAL]\x1b[0m System online"
    );
}
