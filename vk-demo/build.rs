use std::env;
use std::path::PathBuf;
use std::process::Command;

fn compile_shader(src: &str, stage: &str, out_name: &str) {
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR missing"));
    let out_path = out_dir.join(out_name);

    let status = Command::new("glslangValidator")
        .arg("-V")
        .arg("-S")
        .arg(stage)
        .arg(src)
        .arg("-o")
        .arg(&out_path)
        .status()
        .expect("failed to execute glslangValidator");

    if !status.success() {
        panic!("glslangValidator failed for {src}");
    }

    println!("cargo:rerun-if-changed={src}");
}

fn main() {
    compile_shader("src/spinning_triangle.vert", "vert", "spinning_triangle.vert.spv");
    compile_shader("src/spinning_triangle.frag", "frag", "spinning_triangle.frag.spv");
}
