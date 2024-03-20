use std::env;
use std::process::Command;

const PORTFOLIO:&str = "portfolio";

fn main(){
    build_frontend(PORTFOLIO);
}

fn build_frontend(source: &str) { 
    Command::new("trunk")
        .args(&["build", "--release"])
        .args(&[
            "-d",
            &format!("{}/../generated/{}", env!("CARGO_MANIFEST_DIR"),source),
        ])
        .args(&["--public-url", source])
        .current_dir(&format!("../{}",source))
        .status()
        .expect("Failed to build Frontend");
}
