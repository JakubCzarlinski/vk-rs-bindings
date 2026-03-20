//! vk-codegen: Vulkan Rust FFI generator
//!
//! Usage:
//!   vk-codegen [--vk <vk.xml>] [--video <video.xml>] [--out <dir>]
//!
//! Reads the Khronos Vulkan Registry XML files and produces a complete
//! `vk-rs-bindings` crate with feature-gated FFI bindings.

mod cfggen;
mod codegen;
mod ir;
mod parser;
mod types;

use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the vk.xml registry file
    #[arg(long, default_value = "tests/fixtures/vk.xml")]
    vk: PathBuf,

    /// Path to the video.xml registry file
    #[arg(long, default_value = "tests/fixtures/video.xml")]
    video: PathBuf,

    /// Output directory for generated code
    #[arg(long, default_value = "vk-rs-bindings")]
    out: PathBuf,
}

fn main() {
    let args = Args::parse();

    eprintln!("vk-codegen: reading {}", args.vk.display());
    let vk_xml = fs::read_to_string(&args.vk).unwrap_or_else(|e| {
        eprintln!("Error: cannot read {}: {}", args.vk.display(), e);
        std::process::exit(1);
    });

    let mut registry = parser::parse_registry(&vk_xml);

    eprintln!("vk-codegen: merging {}", args.video.display());
    let video_xml = fs::read_to_string(&args.video).unwrap_or_else(|e| {
        eprintln!("Error: cannot read {}: {}", args.video.display(), e);
        std::process::exit(1);
    });
    parser::merge_registry(&mut registry, &video_xml);

    eprintln!("vk-codegen: resolving enum extensions …");
    parser::apply_require_extensions(&mut registry);

    // Remap after apply_require_extensions so SPEC_VERSION/EXTENSION_NAME constants
    // collected from video header <require> blocks also get their provided_by remapped.
    registry.remap_video_header_names();

    eprintln!(
        "vk-codegen: {} typedefs, {} structs, {} enums, {} commands, {} constants",
        registry.typedefs.len(),
        registry.structs.len(),
        registry.enums.len(),
        registry.commands.len(),
        registry.constants.len(),
    );
    eprintln!(
        "vk-codegen: {} core features, {} extensions ({} enabled)",
        registry.features.len(),
        registry.extensions.len(),
        registry
            .extensions
            .iter()
            .filter(|e| !e.is_disabled())
            .count(),
    );

    eprintln!("vk-codegen: generating …");

    let src_dir = args.out.join("src");
    if let Err(e) = fs::create_dir_all(&src_dir) {
        eprintln!(
            "Error: cannot create output directory {}: {}",
            src_dir.display(),
            e
        );
        std::process::exit(1);
    }

    let files = codegen::generate(&registry);
    write_file(&args.out, "Cargo.toml", &files.cargo_toml);
    write_file(&src_dir, "lib.rs", &files.lib_rs);
    write_file(&src_dir, "types.rs", &files.types_rs);
    write_file(&src_dir, "enums.rs", &files.enums_rs);
    write_file(&src_dir, "consts.rs", &files.consts_rs);
    write_file(&src_dir, "commands.rs", &files.commands_rs);
    write_file(&src_dir, "loader.rs", &files.loader_rs);
    write_file(&src_dir, "validation.rs", &files.validation_rs);
    write_file(&args.out, "vk-features.dot", &files.dot_graph);

    eprintln!("vk-codegen: done -> {}", args.out.display());
    eprintln!("vk-codegen: render graph: dot -Tsvg vk-features.dot -o vk-features.svg");
}

/// Writes content to a file at the specified directory.
///
/// Arguments:
///
/// - `dir`: The target directory path.
/// - `name`: The filename to create.
/// - `content`: The string content to write.
fn write_file(dir: &Path, name: &str, content: &str) {
    let path = dir.join(name);
    if let Err(e) = fs::write(&path, content) {
        eprintln!("Error: cannot write {}: {}", path.display(), e);
        std::process::exit(1);
    }
    eprintln!("  wrote {} ({} bytes)", path.display(), content.len());
}
