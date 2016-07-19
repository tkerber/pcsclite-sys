extern crate pkg_config;

fn main() {
    use std::env::var;
    let target = var("TARGET").unwrap();
    if target.contains("windows") {
        println!("cargo:rustc-link-lib=dylib=winscard");
    } else {
        println!("cargo:rustc-link-lib=dylib=pcsclite");
        pkg_config::find_library("libpcsclite").unwrap();
    }
}
