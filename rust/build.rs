fn main() {
    let target = std::env::var("TARGET").unwrap_or_default();
    // 16KB page alignment required by Google Play for Android native .so files.
    // Uses -Wl, prefix because the Cargokit linker wrapper invokes clang
    // (not ld directly), so linker flags must be wrapped.
    if target.contains("android") {
        println!("cargo:rustc-link-arg=-Wl,-z,max-page-size=16384");
    }
}
