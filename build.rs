fn main() {
    println!("cargo:rustc-link-search=native=libs");
    println!("cargo:rustc-link-lib=dylib=ZRDDSC_VS2019");
}
