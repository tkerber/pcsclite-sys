extern crate pkg_config;

fn main() {
    use std::env::var;
    let target = var("TARGET").unwrap();
    let target: Vec<_> = target.split('-').collect();
    if target.get(2) == Some(&"windows") {
        println!("cargo:rustc-link-lib=dylib=winscard");
    } else {
        println!("cargo:rustc-link-lib=dylib=pcsclite");
        pkg_config::find_library("libpcsclite").unwrap();
    }
}
