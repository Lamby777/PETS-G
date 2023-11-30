use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=./dg/src");
    println!("cargo:rerun-if-changed=build.rs");

    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let dir = PathBuf::from(&dir);

    let dg_dir = dir.join("dg");

    Command::new("dg")
        .args(&["src/main.dg", "-o", "packed.dgc"])
        .current_dir(&dg_dir)
        .status()
        .unwrap();
}
