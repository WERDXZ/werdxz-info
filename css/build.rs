use std::env;
use std::process::Command;

fn main() {
    tailwindcss_build("./global.css", "main.generated.css");
}

fn tailwindcss_build(source: &str, output: &str) {
    Command::new("tailwindcss")
        .args(&["-i", source])
        .args(&[
            "-o",
            &format!("{}/../generated/assets/style/{output}", env!("CARGO_MANIFEST_DIR")),
        ])
        .arg("--minify")
        .current_dir(&format!("{}/css", env!("CARGO_MANIFEST_DIR")))
        .status()
        .expect("no tailwindcss");
}
