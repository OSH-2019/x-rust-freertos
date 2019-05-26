// build.rs - The build script.
// This file is created by Fan Jinhao.

extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    // NOTE: Running bindgen in every build is quite time-consuming.
    // The solution is to comment out the following line after the first time you've generated the bindings.
    // run_bindgen();

    run_cc();
}

// Run bindgen to genernate C bindings in portable.h to Rust.
fn run_bindgen() {
    let bindings = bindgen::Builder::default()
        .header("portable/portable.h")
        .whitelist_function(".+Port.+")
        .whitelist_type("^[A-Z].+_t")
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from("./src/");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

// Run cc to compile portable files.
fn run_cc() {
    cc::Build::new()
        .file("portable/port.c")
        .file("portable/heap_3.c")
        .compile("libport.a");
}
