use cxx_build::bridge;

fn main() {
    let mut build = bridge("src/lib.rs");

    build
        .file("../libstark/src/languages/Bair/BairWitness.cpp")
        .flag_if_supported("-std=c++14")
        .include("src")
        .include("../libstark/src/languages/Bair/")
        .compile("bair_witness");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/bair_witness_bridge.h");
}
