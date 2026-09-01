// src/commands/execute_full_patrol.rs

use crate::audit;
use crate::scanner;
use crate::ui::style_terminal_output::{BLUE, BOLD, DIM, GREEN, RED, RESET, colorize};
use std::io::Write;

pub fn run() -> bool {
    println!(
        "{} Deploying Watchdog for full perimeter patrol...\n",
        colorize(BLUE, "[INFO]")
    );

    print!(" [ ] 1/2 Auditing supply chain dependencies...");
    let _ = std::io::stdout().flush();

    let clean_deps = audit::audit_dependencies(false, false, None, false).unwrap_or_default();

    print!("\r\x1B[2K");
    if clean_deps {
        println!(" [{}] 1/2 Supply chain audit passed.", colorize(GREEN, "✓"));
    } else {
        println!(
            " [{}] 1/2 Supply chain audit failed.\n     {} Run {} for more information.",
            colorize(RED, "✗"),
            colorize(DIM, "↳"),
            colorize(BOLD, "woof audit")
        );
    }

    print!(" [ ] 2/2 Sweeping perimeter for exposed secrets...");
    let _ = std::io::stdout().flush();

    let clean_secrets = scanner::execute_sweep(".", false);

    print!("\r\x1B[2K");
    if clean_secrets {
        println!(" [{}] 2/2 Secret sweep passed.", colorize(GREEN, "✓"));
    } else {
        println!(
            " [{}] 2/2 Secret sweep failed.\n     {} Run {} for more information.",
            colorize(RED, "✗"),
            colorize(DIM, "↳"),
            colorize(BOLD, "woof scan")
        );
    }

    println!("\n{DIM}--------------------------------------------------{RESET}");

    if clean_secrets && clean_deps {
        println!(
            "{} Patrol Complete: Perimeter is completely secure.",
            colorize(GREEN, "[INFO]")
        );
        println!(
            "{} You are cleared to commit and push.",
            colorize(GREEN, "[INFO]")
        );
        true
    } else {
        println!(
            "{} Patrol Complete: Threats remain inside the perimeter.",
            colorize(RED, "[CRITICAL]")
        );
        println!(
            "{} Action Required: Review diagnostics above and remediate before pushing.",
            colorize(RED, "[INFO]")
        );
        false
    }
}
