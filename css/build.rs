use std::process::Command;
use std::env;

// const TAILWINDCSS_CONFIG:&str = "../css/tailwin.config.js";

fn main() {
    println!("excecuting buildscript");
    env::set_current_dir("./css").expect("Failed to change directory");
    tailwindcss_build("./global.css", "main.css");
}

fn tailwindcss_build(source: &str, output: &str) {
    Command::new("tailwindcss")
        .args(&["-i", source])
        .args(&["-o", &format!("../../static/assets/style/{output}")])
        // .args(&["-c", TAILWINDCSS_CONFIG])
        .arg("--minify")
        .status()
        .expect("no tailwindcss");

}
