# librime-sys

Rust FFI bindings for [librime](https://github.com/rime/librime).

## Building

Building this crate requires `librime` headers (`rime_api.h`, `rime_levers_api.h`) and shared/static library to be available, along with `libclang` for [`bindgen`](https://github.com/rust-lang/rust-bindgen).

### Environment Variables

When building via `cargo build` (or as a dependency in your project), `build.rs` locates `librime` using the following environment variables:

- `LIBRIME_INCLUDE_DIR` (or `RIME_INCLUDE_DIR`): Path to the directory containing `librime` header files.
- `LIBRIME_LIB_DIR` (or `RIME_LIB_DIR`): Path to the directory containing the compiled `librime` library.

If these variables are not set, the build script checks `./librime/dist/{include,lib}` in the crate directory and falls back to `/usr/include` and `/usr/lib`.

### Development with `cargo-make`

When working directly in this repository, you can use [`cargo-make`](https://github.com/sagiegurari/cargo-make):

```sh
cargo make
```

The included `Makefile.toml` automatically fetches and builds `librime` from source into `./librime/dist` and configures the environment variables above for building and testing (note that `Makefile.toml` is not invoked by plain `cargo build`).

