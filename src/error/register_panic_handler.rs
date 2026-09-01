// src/error/register_panic_handler.rs

use std::panic;

pub fn setup() {
    panic::set_hook(Box::new(|panic_info| {
        let mut message = String::from("\n\x1b[31;1m[CRITICAL] Internal Watchdog Fault\x1b[0m\n");
        message.push_str("Rustywoof encountered an unexpected catastrophic failure.\n\n");

        let reason = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            *s
        } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            s.as_str()
        } else {
            "Unknown Panic Payload"
        };
        message.push_str(&format!("Reason: {}\n", reason));

        if let Some(location) = panic_info.location() {
            message.push_str(&format!(
                "Location: {}:{}\n",
                location.file(),
                location.line()
            ));
        }

        message.push_str(
            "\nAction Required: Please report this fault to the security engineering team at:\n",
        );
        message.push_str("https://github.com/ianramy/rustywoof/issues\n");

        eprintln!("{}", message);
    }));
}
