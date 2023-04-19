# Rust Project
This repository contains a Rust project that interacts with the libSTARK library. The libSTARK library is used for constructing scalable, transparent, and post-quantum secure zero-knowledge proofs.

The Rust project includes a module named libstark_tests which contains various tests for the libSTARK library. Additionally, the project includes a C++ bridge module bair_witness_bridge which interfaces with the BairWitness class in the libSTARK library.

The BairWitness class represents the witness for a Boolean arithmetic circuit. The Rust project interacts with this class through the C++ bridge module to generate and verify proofs of the circuit's validity.

To build and run the Rust project, follow these steps:

Clone the repository: git clone https://github.com/muaj07/zk_rust.git
Change into the rust_project directory: cd zk_rust/rust_project
Build the project: cargo build
Run the tests: cargo test
Note that building the project requires the libSTARK library to be installed on your system. Refer to the libSTARK repository for installation instructions.

The Rust project can be integrated into other applications by importing the libstark_tests module and using the BairWitness class through the C++ bridge module.
