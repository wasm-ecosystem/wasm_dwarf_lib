use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=wit/dwarf.wit");
    println!("cargo:rerun-if-changed=build.rs");

    if !Command::new("wit-bindgen")
        .args([
            "rust",
            "wit",
            "--out-dir",
            "src",
            "--runtime-path",
            "wit_bindgen_rt",
        ])
        .status()
        .expect("Failed to run wit-bindgen")
        .success()
    {
        panic!("wit-bindgen command failed");
    }
}
