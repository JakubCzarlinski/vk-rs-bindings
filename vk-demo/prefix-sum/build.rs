use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR missing"));
    let out_path = out_dir.join("prefix_sum.spv");

    let status = Command::new("glslangValidator")
        .arg("-V")
        .arg("-S")
        .arg("comp")
        .arg("src/prefix_sum.glsl")
        .arg("-o")
        .arg(&out_path)
        .arg("--lto")
        .arg("--target-env")
        .arg("vulkan1.4")
        .arg("-g0")
        .status()
        .expect("failed to execute glslangValidator");

    if !status.success() {
        panic!("glslangValidator failed for src/prefix_sum.glsl");
    }

    println!("cargo:rerun-if-changed=src/prefix_sum.glsl");
}
