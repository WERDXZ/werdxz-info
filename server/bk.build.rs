use std::process::Command;

const TAILWINDCSS_CONFIG:&str = "../css/tailwin.config.js";

fn main() {
    tailwindcss_build("../css/global.css", "main.css");
}

fn tailwindcss_build(source: &str, output: &str) {
    Command::new("tailwindcss")
        .args(&["-i", source])
        .args(&["-o", &format!("../static/assets/style/{output}")])
        // .args(&["-c", TAILWINDCSS_CONFIG])
        .arg("--minify")
        .status()
        .expect("no tailwindcss");
}


// const FRONTEND_DIR: &str = "../ui";
//
// fn main() {
//     println!("cargo:rerun-if-changed={}/src", FRONTEND_DIR);
//     println!("cargo:rerun-if-changed={}/static", FRONTEND_DIR);
//     
//     build_frontend(FRONTEND_DIR);
// }
//
// fn build_frontend<P: AsRef<Path>>(source: P) {
//     Command::new("trunk")
//         .args(&["build", "--release"])
//         .current_dir(source.as_ref())
//         .status()
//         .expect("Failed to build Frontend");
// }
