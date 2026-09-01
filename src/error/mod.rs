// src/error/mod.rs

pub mod aggregate_scan_violations;
pub mod format_json_reports;
pub mod map_system_exit_codes;
pub mod register_panic_handler;
pub mod segregate_domain_diagnostics;
pub use register_panic_handler::setup as setup_panic_handler;
