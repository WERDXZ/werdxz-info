use std::process::Command;
use std::env;

// const TAILWINDCSS_CONFIG:&str = "../css/tailwin.config.js";

fn main() {
    println!("excecuting buildscript");
    env::set_current_dir("./css").expect("Failed to change directory");
    tailwindcss_build("./global.css", "main.generated.css");
}

fn tailwindcss_build(source: &str, output: &str) {
    Command::new("tailwindcss")
        .args(&["-i", source])
        .args(&["-o", &format!("{}/static/assets/style/{output}",env!("CARGO_MANIFEST_DIR"))])
        // .args(&["-c", TAILWINDCSS_CONFIG])
        .arg("--minify")
        .status()
        .expect("no tailwindcss");

}
