use std::process::Command;

fn main() {
    Command::new("tar")
        .args(["cf", "assets.tar", "assets"])
        .output()
        .expect("Failed to package assets");

    println!("cargo:rerun-if-changed=assets");
    println!("cargo:rerun-if-changed=build.rs");

    println!("cargo:rustc-env=ASSETS_PATH=assets.tar");
}
