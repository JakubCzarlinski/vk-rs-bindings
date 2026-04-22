use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR missing"));
    let out_path = out_dir.join("shader.spv");

    let status = Command::new("glslangValidator")
        .arg("-V")
        .arg("-S")
        .arg("comp")
        .arg("src/shader.glsl")
        .arg("-o")
        .arg(&out_path)
        .status()
        .expect("failed to execute glslangValidator");

    if !status.success() {
        panic!("glslangValidator failed for src/shader.glsl");
    }

    println!("cargo:rerun-if-changed=src/shader.glsl");
}
