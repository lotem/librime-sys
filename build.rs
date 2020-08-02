extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=rime");
    if let Ok(_) = env::var("CARGO_FEATURE_SEPARATE_GEARS_LIB") {
        println!("cargo:rustc-link-lib=rime-gears");
    }

    let bindings = bindgen::Builder::default()
        .blacklist_type("max_align_t")  // https://github.com/rust-lang/rust-bindgen/issues/550
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
