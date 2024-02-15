fn main() {
    println!("cargo:rerun-if-changed=./dg/src");
    println!("cargo:rerun-if-changed=build.rs");

    dialogical::compile("dg/src/main.dg", "dg/packed.dgc").unwrap();
}
