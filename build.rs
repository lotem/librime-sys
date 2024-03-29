extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let librime_include_dir = env::var("LIBRIME_INCLUDE_DIR").unwrap_or("include".to_owned());
    let librime_lib_dir = env::var("LIBRIME_LIB_DIR").unwrap_or("lib".to_owned());

    println!("cargo:rustc-link-search={librime_lib_dir}");
    println!("cargo:rustc-link-lib=rime");
    if env::var("CARGO_FEATURE_SEPARATE_GEARS_LIB").is_ok() {
        println!("cargo:rustc-link-lib=rime-gears");
    }

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{librime_include_dir}"))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
