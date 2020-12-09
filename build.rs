use autotools;
use bindgen;
use std::path::PathBuf;
use std::env::var;

fn main() {
    let dst = autotools::Config::new("libigraph")
        .reconf("-ivf")
        .build();

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=igraph");

    let bindings = bindgen::Builder::default()
        .header("somanyheaders")
        .generate()
        .expect("unable to generate bindings");

    let out = PathBuf::from(var("OUT_DIR").unwrap());
    bindings.
        write_to_file(out.join("bindings.rs"))
        .expect("Could not write out bindings");
}
