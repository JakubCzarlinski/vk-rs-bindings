//! vk-codegen: Vulkan Rust FFI generator
//!
//! Usage:
//!   vk-codegen [--vk <vk.xml>] [--video <video.xml>] [--out <dir>]
//!
//! Reads the Khronos Vulkan Registry XML files and produces a complete
//! `vk-sys` crate with feature-gated FFI bindings.

mod cfggen;
mod codegen;
mod ir;
mod parser;
mod types;

use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let vk_path = arg(&args, "--vk")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("vk.xml"));
    let video_path = arg(&args, "--video").map(PathBuf::from);
    let out_dir = arg(&args, "--out")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("vk-sys-generated"));

    eprintln!("vk-codegen: reading {}", vk_path.display());
    let vk_xml = fs::read_to_string(&vk_path)
        .unwrap_or_else(|e| panic!("Cannot read {}: {}", vk_path.display(), e));

    let mut reg = parser::parse_registry(&vk_xml);

    if let Some(ref vp) = video_path {
        eprintln!("vk-codegen: merging {}", vp.display());
        let video_xml = fs::read_to_string(vp)
            .unwrap_or_else(|e| panic!("Cannot read {}: {}", vp.display(), e));
        parser::merge_registry(&mut reg, &video_xml);
        // NOTE: remap_video_header_names is called AFTER apply_require_extensions
        // so that constants collected from video header require blocks also get remapped.
    }

    eprintln!("vk-codegen: resolving enum extensions …");
    parser::apply_require_extensions(&mut reg);

    // Remap after apply_require_extensions so SPEC_VERSION/EXTENSION_NAME constants
    // collected from video header <require> blocks also get their provided_by remapped.
    if video_path.is_some() {
        reg.remap_video_header_names();
    }

    eprintln!(
        "vk-codegen: {} typedefs, {} structs, {} enums, {} commands, {} constants",
        reg.typedefs.len(),
        reg.structs.len(),
        reg.enums.len(),
        reg.commands.len(),
        reg.constants.len(),
    );
    eprintln!(
        "vk-codegen: {} core features, {} extensions ({} enabled)",
        reg.features.len(),
        reg.extensions.len(),
        reg.extensions.iter().filter(|e| !e.is_disabled()).count(),
    );

    eprintln!("vk-codegen: generating …");
    let files = codegen::generate(&reg);

    let src_dir = out_dir.join("src");
    fs::create_dir_all(&src_dir).expect("create output dir");

    write_file(&out_dir, "Cargo.toml", &files.cargo_toml);
    write_file(&src_dir, "lib.rs", &files.lib_rs);
    write_file(&src_dir, "types.rs", &files.types_rs);
    write_file(&src_dir, "enums.rs", &files.enums_rs);
    write_file(&src_dir, "consts.rs", &files.consts_rs);
    write_file(&src_dir, "commands.rs", &files.commands_rs);
    write_file(&src_dir, "loader.rs", &files.loader_rs);
    write_file(&src_dir, "validation.rs", &files.validation_rs);
    write_file(&out_dir, "vk-features.dot", &files.dot_graph);

    eprintln!("vk-codegen: done -> {}", out_dir.display());
    eprintln!("vk-codegen: render graph: dot -Tsvg vk-features.dot -o vk-features.svg");
}

fn arg<'a>(args: &'a [String], flag: &str) -> Option<&'a str> {
    args.windows(2)
        .find(|w| w[0] == flag)
        .map(|w| w[1].as_str())
}

fn write_file(dir: &Path, name: &str, content: &str) {
    let path = dir.join(name);
    fs::write(&path, content).unwrap_or_else(|e| panic!("Cannot write {}: {}", path.display(), e));
    eprintln!("  wrote {} ({} bytes)", path.display(), content.len());
}
