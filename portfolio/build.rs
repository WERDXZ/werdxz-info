use std::env;
use std::process::Command;

const SOURCE:&str = "portfolio";

fn main(){
    Command::new("trunk")
        .args(&["build", "--release"])
        .args(&[
            "-d",
            &format!("{}/../generated/{}", env!("CARGO_MANIFEST_DIR"),SOURCE),
        ])
        .args(&["--public-url", SOURCE])
        .status()
        .expect("Failed to build Frontend");
}
