#[cfg(target_os = "linux")]
pub fn main() {
    println!("Hello from Linux!");
}

#[cfg(not(target_os = "linux"))]
pub fn main() {
    println!("Hello from everything not Linux!");
}