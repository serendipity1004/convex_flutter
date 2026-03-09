fn main() {
    let target = std::env::var("TARGET").unwrap_or_default();
    // 16KB page alignment required by Google Play for Android native .so files.
    if target.contains("android") {
        println!("cargo:warning=Setting max-page-size=16384 for target: {}", target);
        println!("cargo:rustc-link-arg=-Wl,-z,max-page-size=16384");
    }
}
