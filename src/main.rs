#[cfg(not(target_os = "android"))]
include!("main2.rs");

#[cfg(target_os = "android")]
pub fn main() {
    println!("Hello from Android!");
}