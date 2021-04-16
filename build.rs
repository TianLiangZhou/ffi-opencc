use std::env;

fn main() {
    for var in env::vars() {
        println!("{} = {}", var.0, var.1)
    }
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut config: cbindgen::Config = Default::default();
    config.language = cbindgen::Language::C;
    cbindgen::generate_with_config(&crate_dir, config)
        .unwrap()
        .write_to_file("lib/ffi_opencc.h");
}
