// tests/error/test_register_panic_handler.rs

use rustywoof::error::register_panic_handler;
use std::thread;

#[test]
fn test_panic_hook_registration_and_execution() {
    register_panic_handler::setup();

    let handle = thread::spawn(|| {
        panic!("Controlled Watchdog Test Panic");
    });

    let result = handle.join();
    assert!(
        result.is_err(),
        "The thread should have panicked and been caught by the custom hook."
    );
}
