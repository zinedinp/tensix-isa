//! Reports this crate's source directory as `DEP_TENSIX_ISA_SRC` to the build scripts of the crates
//! that depend on it (tensix-firmware-images builds the firmware from it).

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!(
        "cargo::metadata=src={}",
        std::env::var("CARGO_MANIFEST_DIR").unwrap()
    );
}
