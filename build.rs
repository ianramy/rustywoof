// build.rs
fn main() {
    // Read the TARGET variable provided by Cargo and forward it to rustc
    let target = std::env::var("TARGET").expect("TARGET not found");
    println!("cargo:rustc-env=TARGET={}", target);
}
