use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-env-changed=LIBRIME_INCLUDE_DIR");
    println!("cargo:rerun-if-env-changed=RIME_INCLUDE_DIR");
    println!("cargo:rerun-if-env-changed=LIBRIME_LIB_DIR");
    println!("cargo:rerun-if-env-changed=RIME_LIB_DIR");

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let local_dist = manifest_dir.join("librime/dist");

    let librime_include_dir = env::var("LIBRIME_INCLUDE_DIR")
        .or_else(|_| env::var("RIME_INCLUDE_DIR"))
        .unwrap_or_else(|_| {
            let local_inc = local_dist.join("include");
            if local_inc.exists() {
                local_inc.to_string_lossy().into_owned()
            } else {
                "/usr/include".to_owned()
            }
        });

    let librime_lib_dir = env::var("LIBRIME_LIB_DIR")
        .or_else(|_| env::var("RIME_LIB_DIR"))
        .unwrap_or_else(|_| {
            let local_lib = local_dist.join("lib");
            if local_lib.exists() {
                local_lib.to_string_lossy().into_owned()
            } else {
                "/usr/lib".to_owned()
            }
        });

    println!("cargo:rustc-link-search={librime_lib_dir}");
    println!("cargo:rustc-link-lib=rime");
    if env::var("CARGO_FEATURE_SEPARATE_GEARS_LIB").is_ok() {
        println!("cargo:rustc-link-lib=rime-gears");
    }

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{librime_include_dir}"))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
