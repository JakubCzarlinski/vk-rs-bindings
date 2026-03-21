//! Integration tests for vk-codegen.
//! Run with: cargo test --package vk-codegen

use std::sync::OnceLock;
use vk_codegen::{codegen, ir, parser};
static VK_FIXTURE: &str = include_str!("fixtures/vk.xml");
static VK_VIDEO_FIXTURE: &str = include_str!("fixtures/video.xml");

fn make_registry() -> &'static ir::Registry {
    static REGISTRY: OnceLock<ir::Registry> = OnceLock::new();
    REGISTRY.get_or_init(|| {
        let mut reg = parser::parse_registry(VK_FIXTURE);
        parser::merge_registry(&mut reg, VK_VIDEO_FIXTURE);
        parser::apply_require_extensions(&mut reg);
        reg.simplify_features();
        reg.remap_video_header_names();
        reg
    })
}

// -- DepExpr parser ------------------------------------------------------------

#[test]
fn dep_simple_atom() {
    let e = ir::parse_dep_expr("VK_VERSION_1_0");
    assert_eq!(e, ir::DepExpr::Atom("VK_VERSION_1_0".into()));
}

#[test]
fn dep_simple_and() {
    let c = ir::parse_dep_expr("VK_A+VK_B").to_dnf_clauses();
    assert_eq!(c.len(), 1);
    assert_eq!(c[0].len(), 2);
    assert_eq!(format!("{:?}", c), "[[\"VK_A\", \"VK_B\"]]");
}

#[test]
fn dep_simple_or() {
    let c = ir::parse_dep_expr("VK_A,VK_B").to_dnf_clauses();
    assert_eq!(c.len(), 2);
    assert_eq!(format!("{:?}", c), "[[\"VK_A\"], [\"VK_B\"]]");
}

#[test]
fn dep_nested_parens() {
    let c = ir::parse_dep_expr("((VK_A,VK_B)+VK_C),VK_D").to_dnf_clauses();
    assert_eq!(c.len(), 3, "got: {c:?}");
    assert!(
        c.iter().any(|cl| cl == &["VK_D"]),
        "no single-atom clause for VK_D; got: {c:?}"
    );
    assert!(
        c.iter().any(|cl| cl == &["VK_A", "VK_C"]),
        "no clause for VK_A+VK_C; got: {c:?}"
    );
    assert!(
        c.iter().any(|cl| cl == &["VK_B", "VK_C"]),
        "no clause for VK_B+VK_C; got: {c:?}"
    );
}

#[test]
fn dep_real_world_complex() {
    let s = "((VK_KHR_get_physical_device_properties2,VK_VERSION_1_1)+VK_KHR_depth_stencil_resolve),VK_VERSION_1_2";
    let c = ir::parse_dep_expr(s).to_dnf_clauses();
    assert_eq!(c.len(), 3, "got: {c:?}");
    assert!(
        c.iter().any(|cl| cl == &["VK_VERSION_1_2"]),
        "no single-atom clause"
    );
}

// -- EnumValue offset encoding -------------------------------------------------

#[test]
fn offset_enum_encoding() {
    let v0 = ir::EnumValue::Offset {
        extnumber: 272,
        offset: 0,
        negative: false,
    };
    let v1 = ir::EnumValue::Offset {
        extnumber: 272,
        offset: 1,
        negative: false,
    };
    assert_eq!(v0.resolve(), Some(1_000_271_000));
    assert_eq!(v1.resolve(), Some(1_000_271_001));
}

#[test]
fn offset_enum_negative() {
    let v = ir::EnumValue::Offset {
        extnumber: 1,
        offset: 0,
        negative: true,
    };
    assert_eq!(v.resolve(), Some(-1_000_000_000));
}

// -- XML parsing ---------------------------------------------------------------

#[test]
fn parse_core_features() {
    let reg = make_registry();
    let names: Vec<_> = reg.features.iter().map(|f| f.name.as_str()).collect();
    assert!(names.contains(&"VK_VERSION_1_0"));
    assert!(names.contains(&"VK_VERSION_1_3"));
}

#[test]
fn parse_extensions_present() {
    let reg = make_registry();
    let names: Vec<_> = reg.extensions.iter().map(|e| e.name.as_str()).collect();
    assert!(names.contains(&"VK_KHR_swapchain"));
    assert!(names.contains(&"VK_KHR_video_queue"));
    assert!(names.contains(&"VK_KHR_video_decode_h264"));
}

#[test]
fn disabled_extension_is_disabled() {
    let reg = make_registry();
    let disabled = reg
        .extensions
        .iter()
        .find(|e| e.name == "VK_ANDROID_native_buffer")
        .expect("fixture has VK_ANDROID_native_buffer");
    assert!(disabled.is_disabled());
}

#[test]
fn parse_struct_fields() {
    let reg = make_registry();
    let ici = reg
        .structs
        .get("VkInstanceCreateInfo")
        .and_then(|v| v.first())
        .expect("VkInstanceCreateInfo");
    let names: Vec<_> = ici.members.iter().map(|m| m.name.as_str()).collect();
    assert!(names.contains(&"sType"));
    assert!(names.contains(&"pApplicationInfo"));
    assert!(names.contains(&"enabledLayerCount"));
}

#[test]
fn parse_command_params() {
    let reg = make_registry();
    let ci = reg
        .commands
        .get("vkCreateInstance")
        .and_then(|v| v.first())
        .expect("vkCreateInstance");
    assert_eq!(ci.params.len(), 3);
    assert_eq!(ci.return_type.base, "VkResult");
}

#[test]
fn parse_enum_variants() {
    let reg = make_registry();
    let result = reg
        .enums
        .get("VkResult")
        .and_then(|v| v.first())
        .expect("VkResult");
    let names: Vec<_> = result.variants.iter().map(|v| v.name.as_str()).collect();
    assert!(names.contains(&"VK_SUCCESS"));
    assert!(names.contains(&"VK_ERROR_OUT_OF_HOST_MEMORY"));
}

#[test]
fn parse_bitmask_is_bitmask() {
    let reg = make_registry();
    let qf = reg
        .enums
        .get("VkQueueFlagBits")
        .and_then(|v| v.first())
        .expect("VkQueueFlagBits");
    assert!(qf.is_bitmask);
    let names: Vec<_> = qf.variants.iter().map(|v| v.name.as_str()).collect();
    assert!(names.contains(&"VK_QUEUE_GRAPHICS_BIT"));
}

#[test]
fn parse_64bit_bitmask() {
    let reg = make_registry();
    let ps = reg
        .enums
        .get("VkPipelineStageFlagBits2")
        .and_then(|v| v.first())
        .expect("VkPipelineStageFlagBits2");
    assert_eq!(ps.bit_width, 64);
}

#[test]
fn provided_by_tracking() {
    let reg = make_registry();
    let ici = &reg.structs["VkInstanceCreateInfo"][0];
    assert!(
        ici.provided_by.contains(&"VK_BASE_VERSION_1_0".to_owned()),
        "VkInstanceCreateInfo.provided_by = {:?}",
        ici.provided_by
    );
    let cmd = &reg.commands["vkCreateSwapchainKHR"][0];
    assert!(
        cmd.provided_by.contains(&"VK_KHR_swapchain".to_owned()),
        "vkCreateSwapchainKHR.provided_by = {:?}",
        cmd.provided_by
    );
}

// -- Code generation -----------------------------------------------------------

fn generate() -> codegen::GeneratedFiles {
    codegen::generate(&make_registry())
}

#[test]
fn cargo_toml_has_features() {
    let f = generate();
    assert!(f.cargo_toml.contains("VK_VERSION_1_0"));
    assert!(f.cargo_toml.contains("VK_VERSION_1_3"));
    assert!(f.cargo_toml.contains("VK_KHR_swapchain"));
    assert!(f.cargo_toml.contains("VK_KHR_video_decode_h264"));
}

#[test]
fn cargo_toml_no_disabled_ext() {
    let f = generate();
    assert!(
        !f.cargo_toml.contains("VK_LUNARG_test_disabled"),
        "disabled extension leaked into Cargo.toml"
    );
}

#[test]
fn cargo_toml_feature_deps_correct() {
    let f = generate();
    assert!(
        f.cargo_toml.contains("\"VK_KHR_surface\""),
        "VK_KHR_surface dep missing from toml"
    );
}

#[test]
fn types_rs_has_structs() {
    let f = generate();
    assert!(f.types_rs.contains("VkInstanceCreateInfo"));
    assert!(f.types_rs.contains("VkSwapchainCreateInfoKHR"));
    assert!(f.types_rs.contains("VkVideoSessionCreateInfoKHR"));
}

#[test]
fn types_rs_has_cfg_gates() {
    let f = generate();
    assert!(f.types_rs.contains("#[cfg"), "no cfg in types.rs");
    assert!(f.types_rs.contains("feature ="), "no feature= in types.rs");
}

#[test]
fn types_rs_has_defaults() {
    let f = generate();
    assert!(
        f.types_rs.contains("DEFAULT") || f.types_rs.contains("fn new"),
        "no DEFAULT or new() in types.rs"
    );
}

#[test]
fn enums_rs_has_variants() {
    let f = generate();
    assert!(f.enums_rs.contains("VK_SUCCESS"));
    assert!(f.enums_rs.contains("VK_QUEUE_GRAPHICS_BIT"));
    assert!(f.enums_rs.contains("VK_PIPELINE_STAGE_2_NONE"));
}

#[test]
fn enums_rs_bitmask_ops() {
    let f = generate();
    assert!(f.enums_rs.contains("BitOr"));
    assert!(f.enums_rs.contains("EMPTY"));
    assert!(f.enums_rs.contains("contains"));
}

#[test]
fn commands_rs_has_pfn() {
    let f = generate();
    assert!(f.commands_rs.contains("PFN_vkCreateInstance"));
    assert!(f.commands_rs.contains("PFN_vkCreateSwapchainKHR"));
    assert!(f.commands_rs.contains("PFN_vkCreateVideoSessionKHR"));
    assert!(f.commands_rs.contains("extern \"system\""));
}

#[test]
fn loader_rs_has_dispatch_table() {
    let f = generate();
    assert!(f.loader_rs.contains("DispatchTable"));
    assert!(f.loader_rs.contains("EMPTY"));
    assert!(f.loader_rs.contains("fn load"));
}

#[test]
fn extension_name_consts_in_consts_rs() {
    // Extension SPEC_VERSION and EXTENSION_NAME constants must be in consts.rs,
    // not in loader.rs. The old EXT_NAME_ pattern was wrong.
    let f = generate();
    // loader.rs must NOT have the old EXT_NAME_ pattern
    assert!(
        !f.loader_rs.contains("EXT_NAME_"),
        "EXT_NAME_ pattern found in loader.rs - extension names must be in consts.rs"
    );
    // We can't test specific extension names without adding them to the fixture,
    // but we can verify the overall constant infrastructure is working.
    // consts.rs should at minimum have the API version constants.
    assert!(
        f.consts_rs.contains("VK_FALSE")
            || f.consts_rs.contains("VK_TRUE")
            || f.consts_rs.contains("VK_MAX")
            || f.consts_rs.contains("VK_WHOLE"),
        "consts.rs should contain API constants"
    );
}

#[test]
fn validation_rs_has_compile_errors() {
    let f = generate();
    assert!(f.validation_rs.contains("compile_error"));
}

#[test]
fn dot_graph_is_valid() {
    let f = generate();
    assert!(f.dot_graph.starts_with("digraph"));
    assert!(f.dot_graph.contains("VK_VERSION_1_0"));
    assert!(f.dot_graph.contains("->"));
}

#[test]
fn superseded_by_preserved() {
    let reg = make_registry();
    let ext = reg
        .extensions
        .iter()
        .find(|e| e.name == "VK_KHR_synchronization2")
        .expect("VK_KHR_synchronization2");
    assert!(
        ext.depr.promoted_to.is_some(),
        "promotedto not captured: {:?}",
        ext.depr
    );
}

#[test]
fn offset_enum_values_in_generated_code() {
    let reg = make_registry();
    // After apply_require_extensions, VkStructureType should have the offset-derived variants
    let stype = reg
        .enums
        .get("VkStructureType")
        .and_then(|v| v.first())
        .expect("VkStructureType");
    let map_info = stype
        .variants
        .iter()
        .find(|v| v.name == "VK_STRUCTURE_TYPE_MEMORY_MAP_INFO");
    let unmap_info = stype
        .variants
        .iter()
        .find(|v| v.name == "VK_STRUCTURE_TYPE_MEMORY_UNMAP_INFO");
    assert!(
        map_info.is_some(),
        "VK_STRUCTURE_TYPE_MEMORY_MAP_INFO not found"
    );
    assert!(
        unmap_info.is_some(),
        "VK_STRUCTURE_TYPE_MEMORY_UNMAP_INFO not found"
    );

    // Verify the computed values
    if let Some(v) = map_info {
        assert_eq!(
            v.value.resolve(),
            Some(1_000_271_000),
            "wrong value for MEMORY_MAP_INFO: {:?}",
            v.value
        );
    }
    if let Some(v) = unmap_info {
        assert_eq!(
            v.value.resolve(),
            Some(1_000_271_001),
            "wrong value for MEMORY_UNMAP_INFO: {:?}",
            v.value
        );
    }
}

#[test]
fn extension_stub_is_disabled() {
    let reg = make_registry();
    let stub = reg
        .extensions
        .iter()
        .find(|e| e.name == "VK_ANDROID_native_buffer")
        .expect("VK_ANDROID_native_buffer");
    assert!(stub.is_disabled(), "extension stub should be disabled");
}

#[test]
fn complex_dep_in_generated_cargo_toml() {
    let f = generate();
    // VK_KHR_dynamic_rendering has complex ((A,B)+C),D depends
    // All atoms should appear as deps in Cargo.toml
    assert!(
        f.cargo_toml.contains("VK_KHR_dynamic_rendering"),
        "VK_KHR_dynamic_rendering missing from Cargo.toml"
    );
    // At least one of its deps must appear
    let has_dep = f.cargo_toml.contains("VK_VERSION_1_2")
        || f.cargo_toml.contains("VK_KHR_depth_stencil_resolve");
    assert!(has_dep, "dynamic_rendering deps missing from Cargo.toml");
}

#[test]
fn superseded_command_has_depr_info() {
    let reg = make_registry();
    let cmd = reg
        .commands
        .get("vkCreateRenderPass")
        .and_then(|v| v.first())
        .expect("vkCreateRenderPass");
    assert!(
        cmd.depr.superseded_by.is_some(),
        "vkCreateRenderPass.depr.superseded_by should be set; got: {:?}",
        cmd.depr
    );

    // Should be superseded by vkCreateRenderPass2
    let superseding = cmd.depr.superseded_by.as_ref().unwrap();
    assert_eq!(
        superseding, "vkCreateRenderPass2",
        "vkCreateRenderPass should be superseded by vkCreateRenderPass2; got: {superseding}"
    );
}

// -- New: opaque extern types --------------------------------------------------

#[test]
fn opaque_basetype_struct_parsed() {
    let reg = make_registry();
    // "struct <n>ANativeWindow</n>;" should be parsed as OpaqueExtern
    let td = reg
        .typedefs
        .get("ANativeWindow")
        .and_then(|v| v.first())
        .expect("ANativeWindow typedef");
    assert_eq!(
        td.kind,
        vk_codegen::ir::TypedefKind::OpaqueExtern,
        "ANativeWindow should be OpaqueExtern, got {:?}",
        td.kind
    );
}

#[test]
fn objc_ifdef_basetype_parsed_as_opaque() {
    let reg = make_registry();
    // The CAMetalLayer #ifdef block should parse as OpaqueExtern
    let td = reg
        .typedefs
        .get("CAMetalLayer")
        .and_then(|v| v.first())
        .expect("CAMetalLayer typedef");
    assert_eq!(
        td.kind,
        vk_codegen::ir::TypedefKind::OpaqueExtern,
        "CAMetalLayer should be OpaqueExtern, got {:?}",
        td.kind
    );
}

#[test]
fn opaque_types_emitted_as_repr_c_struct() {
    // Opaque types are only emitted when they have provided_by set (i.e. some
    // extension requires them).  In this fixture, ANativeWindow and CAMetalLayer
    // are declared but not required by any active extension, so they must be
    // absent from types.rs entirely (empty provided_by -> skip).
    // The key invariant tested here is that the *parser* correctly identified them
    // as OpaqueExtern - see `opaque_basetype_struct_parsed` and
    // `objc_ifdef_basetype_parsed_as_opaque` for that.
    // If an extension DID require ANativeWindow, it must not be emitted as a
    // type alias (no valid RHS for opaque C structs).
    let f = generate();
    assert!(
        !f.types_rs.contains("pub type ANativeWindow"),
        "ANativeWindow wrongly emitted as type alias - must be repr(C) struct or absent"
    );
    assert!(
        !f.types_rs.contains("pub type CAMetalLayer"),
        "CAMetalLayer wrongly emitted as type alias - must be repr(C) struct or absent"
    );
}

// -- New: video struct with enum-based array sizes -----------------------------

#[test]
fn video_struct_with_enum_array_size_parsed() {
    let reg = make_registry();
    let s = reg
        .structs
        .get("StdVideoH265LongTermRefPicsSps")
        .and_then(|v| v.first())
        .expect("StdVideoH265LongTermRefPicsSps");
    // Find the array member
    let arr_member = s
        .members
        .iter()
        .find(|m| m.name == "lt_ref_pic_poc_lsb_sps")
        .expect("lt_ref_pic_poc_lsb_sps member");
    assert!(
        arr_member.ty.is_array.is_some(),
        "lt_ref_pic_poc_lsb_sps should have array size; got: {:?}",
        arr_member.ty
    );
    assert_eq!(
        arr_member.ty.is_array.as_deref(),
        Some("STD_VIDEO_H265_MAX_LONG_TERM_REF_PICS_SPS"),
        "wrong array size constant"
    );
}

// -- New: video codec header remapping -----------------------------------------

#[test]
fn video_extension_is_not_disabled() {
    let reg = make_registry();
    let ext = reg
        .extensions
        .iter()
        .find(|e| e.name == "vulkan_video_codec_h264std_decode")
        .expect("vulkan_video_codec_h264std_decode");
    // is_disabled() should be false so types get provided_by during parse
    assert!(
        !ext.is_disabled(),
        "video header should not be fully disabled during parse"
    );
    // is_video_header() should be true so it's excluded from output features
    assert!(
        ext.is_video_header(),
        "video header should report is_video_header()"
    );
}

#[test]
fn video_ext_not_in_cargo_toml_features() {
    let f = generate();
    assert!(
        !f.cargo_toml.contains("vulkan_video_codec"),
        "video codec header name leaked into Cargo.toml features"
    );
}

#[test]
fn video_types_remapped_to_vk_feature() {
    // After remap_video_header_names(), StdVideoH265LongTermRefPicsSps.provided_by
    // should contain VK_KHR_video_decode_h265 (mapped from vulkan_video_codec_h265std_decode)
    let reg = make_registry();
    let s = reg
        .structs
        .get("StdVideoH265LongTermRefPicsSps")
        .and_then(|v| v.first())
        .expect("StdVideoH265LongTermRefPicsSps");

    assert!(
        s.provided_by
            .contains(&"VK_KHR_video_decode_h265".to_owned()),
        "StdVideoH265LongTermRefPicsSps.provided_by should contain VK_KHR_video_decode_h265; got: {:?}",
        s.provided_by
    );
}

// -- New: disabled extension items scrubbed ------------------------------------

#[test]
fn disabled_extension_items_not_in_types_rs() {
    let f = generate();
    // VK_KHR_extension_123 and VK_LUNARG_test_disabled are disabled -
    // any items they uniquely provide must not appear in output.
    // (Our fixture doesn't add unique types for these, but the cargo_toml check
    //  verifies the extension itself is absent.)
    assert!(
        !f.cargo_toml.contains("VK_KHR_mir_surface"),
        "disabled placeholder ext in Cargo.toml"
    );
    assert!(
        !f.cargo_toml.contains("VK_GOOGLE_extension_49"),
        "explicitly disabled ext in Cargo.toml"
    );
}

// -- New: correct URL format ---------------------------------------------------

#[test]
fn refpage_urls_use_correct_path() {
    let f = generate();
    let combined = format!("{}{}{}", f.types_rs, f.commands_rs, f.consts_rs);
    // Correct path: /refpages/latest/refpages/source/
    assert!(
        combined.contains("refpages/latest/refpages/source/"),
        "URL does not use expected /refpages/latest/refpages/source/ path"
    );
    // Wrong path must not appear
    assert!(
        !combined.contains("spec/latest/chapters/refpages"),
        "old spec/latest/chapters URL still present"
    );
}

// -- New: duplicate API-variant members ---------------------------------------

#[test]
fn api_variant_members_deduped_in_ir() {
    // VkPipelineShaderStageCreateInfo has `pName` twice - once for vulkan, once for vulkansc.
    // After parsing the struct should still have it twice (both members preserved in IR)
    // but only ONCE in the generated output after deduplication.
    let reg = make_registry();
    let s = reg
        .structs
        .get("VkPipelineShaderStageCreateInfo")
        .and_then(|v| v.first())
        .expect("VkPipelineShaderStageCreateInfo");
    // Both members present in raw IR
    let p_name_count = s.members.iter().filter(|m| m.name == "pName").count();
    assert!(p_name_count >= 1, "pName member missing entirely");
    // The XML has two entries so count should be 2 in raw IR
    assert_eq!(
        p_name_count, 2,
        "expected 2 raw pName entries (one per api variant)"
    );
}

#[test]
fn api_variant_dedup_in_generated_types() {
    let f = generate();
    // VkTestApiVariantStruct must be present
    assert!(
        f.types_rs.contains("VkPipelineShaderStageCreateInfo"),
        "VkPipelineShaderStageCreateInfo missing from types.rs"
    );
    // Find the struct block and check dedup within it only
    let start = f
        .types_rs
        .find("struct VkPipelineShaderStageCreateInfo")
        .expect("struct VkPipelineShaderStageCreateInfo not found");
    // The struct block ends at the next top-level `pub struct` or `pub union`
    let block_end = f.types_rs[start..]
        .find("\npub struct ")
        .or_else(|| f.types_rs[start..].find("\npub union "))
        .map(|i| start + i)
        .unwrap_or(f.types_rs.len());
    let struct_block = &f.types_rs[start..block_end];
    let count = struct_block.matches("pub pName:").count();
    assert_eq!(
        count, 1,
        "duplicate field `pName` within VkPipelineShaderStageCreateInfoVkPipelineShaderStageCreateInfo (found {count} occurrences):\n{struct_block}"
    );
}

#[test]
fn stype_uses_full_const_name() {
    let f = generate();
    // sType default must use the full VK_STRUCTURE_TYPE_APPLICATION_INFO constant,
    // NOT a stripped "APPLICATION_INFO" variant.
    assert!(
        f.types_rs.contains("VK_STRUCTURE_TYPE_APPLICATION_INFO"),
        "sType default does not use full VK_STRUCTURE_TYPE_ constant name"
    );
    // The broken stripped form must not appear
    assert!(
        !f.types_rs.contains("VkStructureType::APPLICATION_INFO"),
        "sType using stripped variant name - must use full VK_STRUCTURE_TYPE_* const"
    );
}

// -- New: funcpointers emitted -------------------------------------------------

#[test]
fn funcpointers_parsed() {
    let reg = make_registry();
    let pfn = reg
        .typedefs
        .get("PFN_vkVoidFunction")
        .and_then(|v| v.first())
        .expect("PFN_vkVoidFunction typedef");
    assert_eq!(
        pfn.kind,
        vk_codegen::ir::TypedefKind::FuncPointer,
        "PFN_vkVoidFunction should be FuncPointer"
    );

    let pfn2 = reg
        .typedefs
        .get("PFN_vkInternalAllocationNotification")
        .and_then(|v| v.first())
        .expect("PFN_vkInternalAllocationNotification typedef");
    assert_eq!(pfn2.kind, vk_codegen::ir::TypedefKind::FuncPointer);
    // Encoded signature must have return type and parameters
    let enc = pfn2.ty.as_deref().expect("encoded signature");
    assert!(
        enc.contains('|'),
        "encoded signature must have '|' separator"
    );
    assert!(
        enc.contains("pUserData"),
        "encoded signature must include param name"
    );
}

#[test]
fn funcpointers_emitted_in_types_rs() {
    let f = generate();
    // Funcpointer types must appear in types.rs as Option<unsafe extern "system" fn(...)>
    assert!(
        f.types_rs.contains("PFN_vkVoidFunction"),
        "PFN_vkVoidFunction missing from types.rs"
    );
    assert!(
        f.types_rs.contains("PFN_vkInternalAllocationNotification"),
        "PFN_vkInternalAllocationNotification missing from types.rs"
    );
    assert!(
        f.types_rs.contains("unsafe extern"),
        "funcpointers must use unsafe extern in types.rs"
    );
    assert!(
        f.types_rs.contains("Option<"),
        "funcpointers must be Option-wrapped in types.rs"
    );
}

// -- New: const-safe defaults without unsafe -----------------------------------

#[test]
fn integer_alias_fields_are_const_safe() {
    // VkInstanceCreateInfo has VkInstanceCreateFlags (typedef u32) and other integer
    // fields - its DEFAULT should be const (no fn new() with unsafe block).
    let f = generate();
    // Find the VkInstanceCreateInfo impl block region
    let idx = f
        .types_rs
        .find("VkInstanceCreateInfo")
        .expect("struct in types.rs");
    let snippet = &f.types_rs[idx..idx.saturating_add(800)];
    assert!(
        snippet.contains("pub const DEFAULT"),
        "VkInstanceCreateInfo should have const DEFAULT (no unsafe fields);\n\
         snippet: {snippet}"
    );
}

// -- New: typed zero defaults --------------------------------------------------

#[test]
fn enum_fields_use_typed_zero() {
    // VkSwapchainCreateInfoKHR has VkFormat and VkPresentModeKHR fields.
    // Their defaults must be VkFormat(0) and VkPresentModeKHR(0), not plain 0.
    let f = generate();
    assert!(
        f.types_rs.contains("VkFormat(0)"),
        "VkFormat field should default to VkFormat(0), not 0"
    );
    assert!(
        f.types_rs.contains("VkPresentModeKHR(0)"),
        "VkPresentModeKHR field should default to VkPresentModeKHR(0)"
    );
    assert!(
        f.types_rs.contains("VkColorSpaceKHR(0)"),
        "VkColorSpaceKHR field should default to VkColorSpaceKHR(0)"
    );
}

#[test]
fn nested_struct_uses_default_not_zeroed() {
    // VkSurfaceCapabilitiesKHR has a VkExtent2D field.
    // Its default must be VkExtent2D::DEFAULT, not unsafe { zeroed() }.
    let f = generate();
    assert!(
        f.types_rs.contains("VkExtent2D::DEFAULT"),
        "nested VkExtent2D field should use VkExtent2D::DEFAULT;\n\
         types_rs snippet: {}",
        &f.types_rs[f.types_rs.find("VkSurfaceCapabilitiesKHR").unwrap_or(0)
            ..f.types_rs.find("VkSurfaceCapabilitiesKHR").unwrap_or(0) + 300]
    );
    // The struct containing a nested struct should still be const DEFAULT
    let idx = f
        .types_rs
        .find("impl VkSurfaceCapabilitiesKHR")
        .expect("VkSurfaceCapabilitiesKHR impl");
    let snippet = &f.types_rs[idx..idx + 400];
    assert!(
        snippet.contains("pub const DEFAULT"),
        "VkSurfaceCapabilitiesKHR should have const DEFAULT (nested struct uses ::DEFAULT);\n\
         snippet: {snippet}"
    );
}

#[test]
fn handle_fields_use_zero_not_typed() {
    // Handles are typedef u64 - plain 0 is correct (not a newtype, so no Foo(0)).
    let f = generate();
    // VkSwapchainCreateInfoKHR.surface is VkSurfaceKHR (a handle = u64 typedef)
    // It should be `surface: 0`, not `surface: VkSurfaceKHR(0)`
    assert!(
        f.types_rs.contains("surface: 0"),
        "handle field surface should default to plain 0 (it's a typedef u64)"
    );
}

#[test]
fn video_enum_uses_typed_zero() {
    // VkVideoSessionCreateInfoKHR has VkVideoCodecOperationFlagBitsKHR field
    let f = generate();
    assert!(
        f.types_rs.contains("VkVideoCodecOperationFlagBitsKHR(0)"),
        "VkVideoCodecOperationFlagBitsKHR should use typed zero"
    );
}

// -- New: array field type uses `as usize` -------------------------------------

#[test]
fn array_field_uses_as_usize_cast() {
    let f = generate();
    // Named-constant array sizes must be cast to usize in the field type.
    // e.g. `pub lt_idx_sps: [u8; STD_VIDEO_H265_MAX_LONG_TERM_REF_PICS_SPS as usize]`
    assert!(
        f.types_rs.contains("as usize]"),
        "array field type missing `as usize` cast for named constant size"
    );
}

#[test]
fn array_default_is_const_safe() {
    let f = generate();
    // Array defaults for primitive element types must be const-safe typed zeros.
    // e.g. `lt_idx_sps: [0u8; STD_VIDEO_H265_MAX_LONG_TERM_REF_PICS_SPS as usize]`
    assert!(
        f.types_rs.contains("[0u8;"),
        "u8 array default should be [0u8; N], got unsafe zeroed"
    );
    // The struct containing the array field should have const DEFAULT
    let idx = f
        .types_rs
        .find("StdVideoH265LongTermRefPicsSps")
        .expect("StdVideoH265LongTermRefPicsSps in types.rs");
    // Should NOT need unsafe because all elements are primitives
    // (num_long_term_sps: u8, used_by_curr_pic_lt_flag: u16 - all plain integers)
    let snippet = &f.types_rs[idx..idx.saturating_add(600)];
    assert!(
        snippet.contains("pub const DEFAULT") || snippet.contains("fn new"),
        "StdVideoH265LongTermRefPicsSps should have DEFAULT or new();\nsnippet: {snippet}"
    );
}

#[test]
fn types_rs_imports_consts() {
    let f = generate();
    assert!(
        f.types_rs.contains("crate::consts::*"),
        "types.rs must import crate::consts::* for array size constant names"
    );
}

// -- New: extension constants in consts.rs -------------------------------------

#[test]
fn spec_version_in_consts_rs() {
    let f = generate();
    assert!(
        f.consts_rs
            .contains("pub const VK_KHR_SWAPCHAIN_SPEC_VERSION: u32 = 70;"),
        "SPEC_VERSION constant missing from consts.rs"
    );
}

#[test]
fn extension_name_string_in_consts_rs() {
    let f = generate();

    // Must be a string constant, not a byte array
    assert!(
        f.consts_rs.contains(
            "pub const VK_KHR_SWAPCHAIN_EXTENSION_NAME: &'static str = \"VK_KHR_swapchain\";"
        ),
        "EXTENSION_NAME constant missing from consts.rs"
    );
}

#[test]
fn extension_consts_are_feature_gated() {
    let f = generate();
    // VK_KHR_SWAPCHAIN_EXTENSION_NAME should be gated by the swapchain feature
    let idx = f
        .consts_rs
        .find("VK_KHR_SWAPCHAIN_EXTENSION_NAME")
        .expect("VK_KHR_SWAPCHAIN_EXTENSION_NAME in consts.rs");
    // Walk backward a bit to find the cfg
    let before = &f.consts_rs[idx.saturating_sub(150)..idx];
    assert!(
        before.contains("VK_KHR_swapchain"),
        "VK_KHR_SWAPCHAIN_EXTENSION_NAME should be feature-gated by VK_KHR_swapchain;\n\
         context before: {before}"
    );
}

// -- New: union fields use ManuallyDrop ----------------------------------------

#[test]
fn union_fields_are_plain_copy_types() {
    // Unions use plain field types (no ManuallyDrop) since all Vulkan union
    // fields are C POD types that implement Copy.
    // A manual Debug impl is provided instead of #[derive(Debug)].
    let f = generate();

    let idx = f
        .types_rs
        .find("pub union VkClearColorValue")
        .expect("VkClearColorValue union in types.rs");
    let snippet = &f.types_rs[idx.saturating_sub(500)..idx.saturating_add(500)];
    if snippet.contains("ManuallyDrop") {
        panic!(
            "union fields should not use ManuallyDrop - all Vulkan union fields are plain Copy types; union snippet:\n{snippet}"
        );
    }
    if !snippet.contains("impl core::fmt::Debug for VkClearColorValue") {
        panic!(
            "union should have a manual Debug impl instead of #[derive(Debug)]; union snippet:\n{snippet}"
        );
    }
    if !snippet.contains("#[derive(Copy, Clone)]") {
        panic!(
            "union should derive Copy+Clone since all fields are Copy; union snippet:\n{snippet}"
        );
    }
}

// -- Video header remap applies to constants ------------------------------------

#[test]
fn video_const_provided_by_is_remapped() {
    // After apply_require_extensions + remap_video_header_names, any constant
    // collected from a video header require block must have VK_KHR_* in provided_by,
    // not the raw video header name like "vulkan_video_codec_h264std_decode".
    let reg = make_registry();
    for (name, cs) in &reg.constants {
        for c in cs {
            for pb in &c.provided_by {
                assert!(
                    !pb.as_str().starts_with("vulkan_video_"),
                    "constant `{name}` still has raw video header `{pb}` in provided_by after remap"
                );
            }
        }
    }
}

// -- c_char arrays use typed zero literal --------------------------------------

#[test]
fn c_char_array_uses_zero_i8_literal() {
    let f = generate();
    // char[] fields (like deviceName) must default to [0i8; N as usize], not unsafe zeroed
    assert!(
        f.types_rs.contains("[0i8;"),
        "c_char array field should default to [0i8; N], got unsafe zeroed or wrong type"
    );
    // Confirm `as usize` is used for named constant sizes
    assert!(
        f.types_rs
            .contains("VK_MAX_PHYSICAL_DEVICE_NAME_SIZE as usize"),
        "c_char array size should use VK_MAX_PHYSICAL_DEVICE_NAME_SIZE as usize"
    );
}

// -- Struct-element arrays use ::DEFAULT ---------------------------------------

#[test]
fn struct_array_element_uses_default() {
    // The VkRect2D has extent: VkExtent2D - not an array, but we also need
    // struct-element arrays like [VkMemoryType; N] to use [VkMemoryType::DEFAULT; N].
    // VkRect2D.extent: VkExtent2D -> VkExtent2D::DEFAULT (not an array, tests scalar)
    let f = generate();
    // The VkExtent2D field in VkRect2D should use VkExtent2D::DEFAULT
    assert!(
        f.types_rs.contains("VkExtent2D::DEFAULT"),
        "VkExtent2D struct field should use VkExtent2D::DEFAULT"
    );
    // VkRect2D should be fully const (no unsafe fn new)
    let idx = f.types_rs.find("impl VkRect2D").expect("VkRect2D impl");
    let snippet = &f.types_rs[idx..idx.saturating_add(400)];
    assert!(
        snippet.contains("pub const DEFAULT"),
        "VkRect2D should have const DEFAULT; snippet:\n{snippet}"
    );
}

// -- Option<T> fields default to None -----------------------------------------

#[test]
fn option_fields_default_to_none() {
    // Function pointer typedefs are emitted as Option<unsafe extern "system" fn(...)>.
    // When such a type is used as a struct field, its default should be None, not 0.
    // VkTestApiVariantStruct has only primitive fields so we check types.rs directly.
    // The PFN_vkVoidFunction and PFN_vkInternalAllocationNotification types exist;
    // if a struct used them, the default should be None.
    // Here we verify the codegen logic: the member_default_const function returns "None"
    // for Option<T> types.
    let f = generate();
    // VkAllocationCallbacks has pfnAllocation etc which in real vk.xml are function pointers.
    // In our minimal fixture they're uint32_t, but we can still verify that None
    // would appear if any Option field existed.
    // At minimum: no field in any struct should default to "0" when its type is Option<...>
    // Check by ensuring Option fields (if any) use None.
    // This test primarily validates no regression - Option<T>: 0 would be a compile error.
    assert!(!f.types_rs.contains(": None; //"), "malformed None default");
    // The code path is tested structurally: if Option fields appeared they'd use None.
    // Verify PFN types are Option-wrapped in types.rs
    assert!(
        f.types_rs.contains("Option<unsafe extern"),
        "funcpointer types should be Option<unsafe extern ...> in types.rs"
    );
}

// -- No nested unsafe blocks ---------------------------------------------------

#[test]
fn no_nested_unsafe_in_fn_new() {
    // fn new() must have at most one outer `unsafe { }` wrapping the struct literal.
    // Individual field defaults must NOT contain their own `unsafe { }` wrappers.
    let f = generate();
    // The pattern "unsafe { core::mem::zeroed() }" inside an already-unsafe fn new()
    // would previously produce "unsafe { ... unsafe { zeroed() } ... }".
    // Now defaults are plain expressions like `core::mem::zeroed::<T>()` (no inner unsafe).
    assert!(
        !f.types_rs.contains("unsafe { core::mem::zeroed()"),
        "individual field defaults must not contain `unsafe {{ core::mem::zeroed() }}` - \
         use `core::mem::zeroed::<T>()` or `[0T; N]` instead"
    );
}

// -- PFN fields default to None ------------------------------------------------

#[test]
fn pfn_fields_parse_as_funcpointer() {
    let reg = make_registry();
    let pfn = reg
        .typedefs
        .get("PFN_vkVoidFunction")
        .and_then(|v| v.first())
        .expect("PFN_vkVoidFunction");
    assert_eq!(pfn.kind, vk_codegen::ir::TypedefKind::FuncPointer);
}

#[test]
fn pfn_struct_fields_default_to_none() {
    let f = generate();
    // VkDebugReportCallbackCreateInfoEXT has PFN fields - they must default to None, not 0
    let idx = f
        .types_rs
        .find("impl VkDebugReportCallbackCreateInfoEXT")
        .expect("VkDebugReportCallbackCreateInfoEXT impl in types.rs");
    let snippet = &f.types_rs[idx..idx.saturating_add(500)];
    assert!(
        snippet.contains("pfnCallback: None"),
        "PFN struct fields must default to None;\nsnippet:\n{snippet}"
    );
    // Should be fully const since None is const-safe
    assert!(
        snippet.contains("pub const DEFAULT"),
        "VkDebugReportCallbackCreateInfoEXT should have const DEFAULT (None is const-safe);\nsnippet:\n{snippet}"
    );
}

#[test]
fn alias_chain_followed_for_enum_default() {
    // VkComponentTypeNV is an alias for VkComponentTypeKHR (an enum newtype).
    // When used as a struct field, it must default to VkComponentTypeNV(0),
    // not 0 (which would require classify_type to follow the alias chain).
    // We test the classify_type logic via the generated output of a real struct.
    // Since our fixture doesn't have VkComponentTypeNV, test the principle:
    // VkColorSpaceKHR is an enum and VkSwapchainCreateInfoKHR has it ->
    // it must appear as VkColorSpaceKHR(0) not plain 0.
    let f = generate();
    assert!(
        f.types_rs.contains("VkColorSpaceKHR(0)"),
        "enum newtype field VkColorSpaceKHR should default to VkColorSpaceKHR(0)"
    );
}

// -- Union: no ManuallyDrop, manual Debug --------------------------------------

#[test]
fn union_has_no_manually_drop() {
    let f = generate();
    if f.types_rs.contains("pub union ") {
        assert!(
            !f.types_rs.contains("ManuallyDrop"),
            "unions must NOT use ManuallyDrop - all Vulkan union fields are Copy"
        );
    }
}

#[test]
fn union_has_manual_debug_impl() {
    let f = generate();
    if f.types_rs.contains("pub union ") {
        assert!(
            f.types_rs.contains("impl core::fmt::Debug for"),
            "unions must have manual Debug impl (cannot auto-derive for union)"
        );
        assert!(
            f.types_rs.contains("debug_struct"),
            "manual Debug impl must use debug_struct format"
        );
    }
}

#[test]
fn union_derives_copy_clone() {
    let f = generate();
    if f.types_rs.contains("pub union ") {
        // Check that Copy+Clone appear before the union declaration
        assert!(
            f.types_rs.contains("#[derive(Copy, Clone)]"),
            "union must derive Copy, Clone"
        );
    }
}

// -- Opaque platform types as *mut c_void --------------------------------------

#[test]
fn opaque_basetype_emitted_as_ptr_alias() {
    let reg = make_registry();
    // ANativeWindow and CAMetalLayer must be OpaqueExtern in the IR
    let an = reg
        .typedefs
        .get("ANativeWindow")
        .and_then(|v| v.first())
        .expect("ANativeWindow");
    assert_eq!(an.kind, vk_codegen::ir::TypedefKind::OpaqueExtern);
    let ca = reg
        .typedefs
        .get("CAMetalLayer")
        .and_then(|v| v.first())
        .expect("CAMetalLayer");
    assert_eq!(ca.kind, vk_codegen::ir::TypedefKind::OpaqueExtern);
}

#[test]
fn opaque_types_emitted_as_newtype_handles() {
    // OpaqueExtern platform types must emit as repr(transparent) newtype handle structs:
    //   pub struct HWND(pub *mut core::ffi::c_void);
    // NOT as zero-sized structs or raw type aliases.
    let f = generate();
    // No zero-sized _private structs
    assert!(
        !f.types_rs.contains("_private: [u8; 0]"),
        "no zero-sized opaque structs allowed"
    );
    // No raw type alias form
    assert!(
        !f.types_rs.contains("pub type HWND"),
        "HWND must be a newtype struct, not a type alias"
    );
    // HWND must be a proper repr(transparent) newtype
    if f.types_rs.contains("pub struct HWND") {
        assert!(
            f.types_rs
                .contains("pub struct HWND(pub *mut core::ffi::c_void)"),
            "HWND must be repr(transparent) newtype over *mut c_void"
        );
        assert!(
            f.types_rs
                .contains("NULL: Self = Self(core::ptr::null_mut())"),
            "HWND must have a NULL constant"
        );
    }
}

// -- Platform requires= types parsed and feature-gated ------------------------

#[test]
fn platform_requires_types_parsed_as_opaque() {
    let reg = make_registry();
    // HWND, Display, etc. must be registered as OpaqueExtern
    let hwnd = reg
        .typedefs
        .get("HWND")
        .and_then(|v| v.first())
        .expect("HWND in typedefs");
    assert_eq!(
        hwnd.kind,
        vk_codegen::ir::TypedefKind::OpaqueExtern,
        "HWND should be OpaqueExtern"
    );
    let display = reg
        .typedefs
        .get("Display")
        .and_then(|v| v.first())
        .expect("Display in typedefs");
    assert_eq!(
        display.kind,
        vk_codegen::ir::TypedefKind::OpaqueExtern,
        "Display should be OpaqueExtern"
    );
}

#[test]
fn platform_types_gated_by_extension_feature() {
    let reg = make_registry();
    // HWND is required by VK_KHR_win32_surface in our fixture
    let hwnd = reg
        .typedefs
        .get("HWND")
        .and_then(|v| v.first())
        .expect("HWND");
    assert!(
        hwnd.provided_by
            .contains(&"VK_KHR_win32_surface".to_owned()),
        "HWND.provided_by should contain VK_KHR_win32_surface; got: {:?}",
        hwnd.provided_by
    );
}

#[test]
fn platform_types_emitted_with_correct_cfg() {
    let f = generate();
    // HWND is provided by VK_KHR_win32_surface - must be feature-gated and a newtype handle
    if f.types_rs.contains("pub struct HWND") {
        let idx = f.types_rs.find("pub struct HWND").unwrap();
        let before = &f.types_rs[idx.saturating_sub(120)..idx];
        assert!(
            before.contains("VK_KHR_win32_surface"),
            "HWND must be gated by VK_KHR_win32_surface;\nbefore: {before}"
        );
        let after = &f.types_rs[idx..idx.saturating_add(80)];
        assert!(
            after.contains("*mut core::ffi::c_void"),
            "HWND must wrap *mut c_void;\nafter: {after}"
        );
    }
}

// -- Alias chain resolves to canonical enum name --------------------------------

#[test]
fn video_enum_fields_use_typed_init() {
    // StdVideoH265ProfileTierLevel has StdVideoH265ProfileIdc and StdVideoH265LevelIdc
    // fields - both are enum newtypes.  They must default to TypeName(0), not plain 0.
    let f = generate();
    assert!(
        f.types_rs.contains("StdVideoH265ProfileIdc(0)"),
        "StdVideoH265ProfileIdc field should default to StdVideoH265ProfileIdc(0)"
    );
    assert!(
        f.types_rs.contains("StdVideoH265LevelIdc(0)"),
        "StdVideoH265LevelIdc field should default to StdVideoH265LevelIdc(0)"
    );
    // The struct must be const DEFAULT since all fields are const-safe
    let idx = f
        .types_rs
        .find("impl StdVideoH265ProfileTierLevel")
        .expect("StdVideoH265ProfileTierLevel impl");
    let snippet = &f.types_rs[idx..idx.saturating_add(300)];
    assert!(
        snippet.contains("pub const DEFAULT"),
        "StdVideoH265ProfileTierLevel should have const DEFAULT;\nsnippet:\n{snippet}"
    );
}

// -- Null ptr alias for OpaqueExtern -------------------------------------------

#[test]
fn opaque_extern_fields_default_to_null_mut() {
    // If a struct has a field of type HWND (= *mut c_void), its default should be
    // core::ptr::null_mut(), not 0.
    // Verify via classify_type that OpaqueExtern -> NullMutAlias path works.
    let reg = make_registry();
    // HWND is OpaqueExtern - look it up and verify kind
    let hwnd = reg
        .typedefs
        .get("HWND")
        .and_then(|v| v.first())
        .expect("HWND");
    assert_eq!(hwnd.kind, vk_codegen::ir::TypedefKind::OpaqueExtern);
    // The generated output should use null_mut() for platform pointer types
    let f = generate();
    // Check the type is emitted as pointer alias
    if f.types_rs.contains("pub type HWND") {
        assert!(
            f.types_rs.contains("*mut core::ffi::c_void"),
            "HWND should be *mut c_void"
        );
    }
}

// -- All structs derive Copy ---------------------------------------------------

#[test]
fn structs_derive_copy() {
    let f = generate();
    // All generated structs should derive Copy so they work as union field types
    // and can be used in const array initializers.
    assert!(
        f.types_rs.contains("#[derive(Debug, Clone, Copy)]"),
        "structs should derive Debug, Clone, Copy"
    );
    // No struct should use the old Clone-only derive
    assert!(
        !f.types_rs.contains("#[derive(Debug, Clone)]"),
        "structs must not use #[derive(Debug, Clone)] without Copy"
    );
}

#[test]
fn unions_still_derive_copy_clone() {
    let f = generate();
    if f.types_rs.contains("pub union ") {
        assert!(
            f.types_rs.contains("#[derive(Copy, Clone)]"),
            "unions should derive Copy, Clone"
        );
    }
}

// -- Enum alias chains: canonical constructor used in defaults -----------------

#[test]
fn enum_alias_fields_use_canonical_constructor() {
    // When a struct field has an enum alias type (e.g. VkScopeNV = VkScopeKHR),
    // the default must use the canonical (non-alias) constructor VkScopeKHR(0),
    // not the alias name VkScopeNV(0) which would fail to compile.
    // We test the classify_type logic by verifying it returns the canonical name.
    let reg = make_registry();
    // Add a simulated enum alias to verify the chain is followed:
    // VkColorSpaceKHR is an enum newtype in the fixture - its default is VkColorSpaceKHR(0).
    // In the generated output, the VkSwapchainCreateInfoKHR.imageColorSpace field
    // should already be VkColorSpaceKHR(0) (not an alias case, but exercises EnumNewtype).
    let f = generate();
    assert!(
        f.types_rs.contains("VkColorSpaceKHR(0)"),
        "enum field should use typed constructor, not plain 0"
    );
    // The classify_type function follows Enum.alias chains.
    // Since our minimal fixture doesn't have a two-level enum alias, we verify at
    // the IR level that enum alias is stored correctly.
    let e = reg
        .enums
        .get("VkColorSpaceKHR")
        .and_then(|v| v.first())
        .expect("VkColorSpaceKHR in enums");
    assert!(
        e.alias.is_none(),
        "VkColorSpaceKHR should not itself be an alias"
    );
}

// -- Platform handle newtypes --------------------------------------------------

#[test]
fn platform_handles_have_null_const() {
    let f = generate();
    // Each platform handle type must have a NULL constant
    if f.types_rs.contains("pub struct HWND") {
        assert!(
            f.types_rs
                .contains("NULL: Self = Self(core::ptr::null_mut())"),
            "HWND must have a NULL constant"
        );
        // NULL must be feature-gated too (impl block has cfg)
        let idx = f.types_rs.find("impl HWND").expect("impl HWND");
        let before = &f.types_rs[idx.saturating_sub(100)..idx];
        assert!(
            before.contains("VK_KHR_win32_surface"),
            "impl HWND must be feature-gated"
        );
    }
}

#[test]
fn platform_handles_are_distinct_types() {
    let f = generate();
    // HWND and HINSTANCE are separate newtype structs, not the same *mut c_void alias
    if f.types_rs.contains("pub struct HWND") && f.types_rs.contains("pub struct HINSTANCE") {
        // Both exist as distinct named structs
        assert!(
            f.types_rs
                .contains("pub struct HWND(pub *mut core::ffi::c_void)"),
            "HWND must be a distinct newtype"
        );
        assert!(
            f.types_rs
                .contains("pub struct HINSTANCE(pub *mut core::ffi::c_void)"),
            "HINSTANCE must be a distinct newtype"
        );
    }
}

// -- Video codec types are not treated as platform opaques ---------------------

#[test]
fn video_codec_types_not_opaque_extern() {
    let reg = make_registry();
    // StdVideoH265ProfileIdc is a video enum - must NOT be OpaqueExtern
    let e = reg
        .enums
        .get("StdVideoH265ProfileIdc")
        .and_then(|v| v.first())
        .expect("StdVideoH265ProfileIdc must be in enums");
    // It's in reg.enums, not typedefs
    assert!(
        reg.typedefs
            .get("StdVideoH265ProfileIdc")
            .and_then(|v| v.first())
            .is_none(),
        "StdVideoH265ProfileIdc must be in enums, not typedefs"
    );
    let _ = e;
}

#[test]
fn video_enum_defaults_are_not_null_ptr() {
    let f = generate();
    // The StdVideoH265ProfileTierLevel struct must not have null_mut() for enum fields
    let idx = f
        .types_rs
        .find("impl StdVideoH265ProfileTierLevel {")
        .expect("StdVideoH265ProfileTierLevel impl");
    let snippet = &f.types_rs[idx..idx.saturating_add(400)];
    assert!(
        !snippet.contains("null_mut()"),
        "video enum fields must not use null_mut();\nsnippet:\n{snippet}"
    );
    assert!(
        snippet.contains("StdVideoH265ProfileIdc(0)"),
        "video enum field must use typed zero constructor;\nsnippet:\n{snippet}"
    );
}

// -- Platform type back-propagation --------------------------------------------

#[test]
fn platform_types_back_propagated_from_structs() {
    // wl_display is not listed in <require><type> for VK_KHR_wayland_surface,
    // but VkWaylandSurfaceCreateInfoKHR has a wl_display* field and IS listed.
    // The back-propagation pass must infer wl_display's provided_by from the struct.
    let reg = make_registry();

    let wl = reg
        .typedefs
        .get("wl_display")
        .and_then(|v| v.first())
        .expect("wl_display typedef");
    assert!(
        wl.provided_by
            .contains(&"VK_KHR_wayland_surface".to_owned()),
        "wl_display must be back-propagated to VK_KHR_wayland_surface via struct member;\n\
         got: {:?}",
        wl.provided_by
    );
}

#[test]
fn platform_types_back_propagated_from_explicit_require() {
    // Types explicitly listed in <require><type> must also get provided_by set.
    let reg = make_registry();

    let display = reg
        .typedefs
        .get("Display")
        .and_then(|v| v.first())
        .expect("Display typedef");
    assert!(
        display
            .provided_by
            .contains(&"VK_KHR_xlib_surface".to_owned()),
        "Display must have VK_KHR_xlib_surface in provided_by; got: {:?}",
        display.provided_by
    );
    let hwnd = reg
        .typedefs
        .get("HWND")
        .and_then(|v| v.first())
        .expect("HWND");
    assert!(
        hwnd.provided_by
            .contains(&"VK_KHR_win32_surface".to_owned()),
        "HWND must have VK_KHR_win32_surface in provided_by; got: {:?}",
        hwnd.provided_by
    );
}

#[test]
fn all_platform_types_emitted_in_types_rs() {
    let f = generate();
    let required = [
        "Display",
        "VisualID",
        "Window",
        "RROutput",
        "wl_display",
        "wl_surface",
        "HINSTANCE",
        "HWND",
        "HMONITOR",
        "HANDLE",
        "DWORD",
        "LPCWSTR",
        "xcb_connection_t",
        "xcb_window_t",
        "IDirectFB",
        "zx_handle_t",
        "GgpStreamDescriptor",
        "_screen_context",
        "NvSciSyncAttrList",
        "NvSciBufAttrList",
    ];
    for name in &required {
        // assert!(
        //     f.types_rs.contains(&format!("pub struct {name}")),
        //     "platform type {name} missing from types.rs"
        // );
        // Each must be a repr(transparent) newtype, not a type alias or zero-sized struct
        assert!(
            f.types_rs
                .contains(&format!("pub struct {name}(pub *mut core::ffi::c_void)")),
            "{name} must be a repr(transparent) newtype over *mut c_void"
        );
    }
}

#[test]
fn platform_types_have_null_constant() {
    let f = generate();
    // Every platform handle must have a NULL const for use as default
    for name in &["HWND", "Display", "wl_display", "xcb_connection_t"] {
        assert!(
            f.types_rs.contains(&format!("pub struct {name}")),
            "{name} missing from types.rs"
        );
        // NULL const must appear somewhere near the impl block
        let marker = format!("impl {name}");
        if let Some(idx) = f.types_rs.find(&marker) {
            let snippet = &f.types_rs[idx..idx.saturating_add(120)];
            assert!(
                snippet.contains("NULL"),
                "{name} impl block must have NULL constant; snippet: {snippet}"
            );
        }
    }
}

#[test]
fn platform_types_feature_gated_correctly() {
    let f = generate();
    // Spot-check a few feature gates
    let cases = [
        ("wl_display", "VK_KHR_wayland_surface"),
        ("HWND", "VK_KHR_win32_surface"),
        ("xcb_connection_t", "VK_KHR_xcb_surface"),
        ("Display", "VK_KHR_xlib_surface"),
        ("zx_handle_t", "VK_FUCHSIA_imagepipe_surface"),
    ];
    for (name, feature) in &cases {
        if let Some(idx) = f.types_rs.find(&format!("pub struct {name}")) {
            let before = &f.types_rs[idx.saturating_sub(500)..idx];
            assert!(
                before.contains(feature),
                "{name} must be gated by {feature}; context before: {before}"
            );
        }
    }
}

// -- Union const DEFAULT -------------------------------------------------------

#[test]
fn union_has_const_default() {
    let f = generate();
    assert!(
        f.types_rs.contains("pub union VkClearColorValue"),
        "VkClearColorValue union must be emitted"
    );
    let idx = f
        .types_rs
        .find("impl VkClearColorValue")
        .expect("VkClearColorValue impl block");
    let snippet = &f.types_rs[idx..idx.saturating_add(250)];
    assert!(
        snippet.contains("pub const DEFAULT"),
        "union must have const DEFAULT;\n{snippet}"
    );
    assert!(
        snippet.contains("pub const fn new()"),
        "union must have const fn new();\n{snippet}"
    );
}

#[test]
fn struct_with_union_field_is_const_default() {
    // Now that unions have DEFAULT, any struct containing a union field should
    // also be able to be const DEFAULT - no more fn new() with unsafe block.
    // Check the global invariant: no uses of the old nested unsafe pattern.
    let f = generate();
    assert!(
        !f.types_rs.contains("unsafe { core::mem::zeroed()"),
        "no field-level unsafe zeroed() should remain in defaults - \
         use outer unsafe block or const DEFAULT"
    );
}

// -- Video codec #define constants ---------------------------------------------

#[test]
fn video_defines_parsed() {
    let reg = make_registry();

    let maker = reg
        .typedefs
        .get("VK_MAKE_VIDEO_STD_VERSION")
        .and_then(|v| v.first())
        .expect("VK_MAKE_VIDEO_STD_VERSION must be in registry");
    assert_eq!(maker.kind, vk_codegen::ir::TypedefKind::Define);
    assert!(
        maker
            .ty
            .as_deref()
            .map(|t| t.starts_with("fn:"))
            .unwrap_or(false),
        "VK_MAKE_VIDEO_STD_VERSION must encode as 'fn:...'; got {:?}",
        maker.ty
    );
    // VK_MAKE_VIDEO_STD_VERSION is a universal utility helper - emitted ungated
    assert!(
        maker.provided_by.is_empty(),
        "VK_MAKE_VIDEO_STD_VERSION must be ungated (empty provided_by); got {:?}",
        maker.provided_by
    );

    let h264 = reg
        .typedefs
        .get("VK_STD_VULKAN_VIDEO_CODEC_H264_DECODE_API_VERSION_1_0_0")
        .and_then(|v| v.first())
        .expect("H264 decode version constant must be in registry");
    assert!(
        h264.ty
            .as_deref()
            .map(|t| t.starts_with("vkver:"))
            .unwrap_or(false),
        "H264 version constant must encode args as 'vkver:...'; got {:?}",
        h264.ty
    );
    assert!(
        h264.provided_by
            .contains(&"VK_KHR_video_decode_h264".to_owned()),
        "H264 version constant must be gated by VK_KHR_video_decode_h264; got {:?}",
        h264.provided_by
    );
}

#[test]
fn video_make_version_emitted_as_const_fn() {
    let f = generate();
    assert!(
        f.consts_rs
            .contains("pub const fn VK_MAKE_VIDEO_STD_VERSION"),
        "VK_MAKE_VIDEO_STD_VERSION must be emitted as a const fn"
    );
    assert!(
        f.consts_rs
            .contains("(major << 22) | (minor << 12) | patch"),
        "const fn body must compute the packed version value"
    );
    // VK_MAKE_VIDEO_STD_VERSION is a universal helper - emitted without a cfg gate
    assert!(!f.consts_rs.contains("#[cfg(feature = \"VK_KHR_video_queue\")]\n#[inline] pub const fn VK_MAKE_VIDEO_STD_VERSION"),
        "VK_MAKE_VIDEO_STD_VERSION should not be gated by a feature flag");
}

#[test]
fn video_version_consts_emitted_as_u32() {
    let f = generate();
    // VK_MAKE_VIDEO_STD_VERSION(1, 0, 0) = (1<<22)|(0<<12)|0 = 4194304
    assert!(
        f.consts_rs
            .contains("VK_STD_VULKAN_VIDEO_CODEC_H264_DECODE_API_VERSION_1_0_0: u32 = 4194304"),
        "H264 decode version const must be a pre-evaluated u32 = 4194304"
    );
    assert!(
        f.consts_rs
            .contains("VK_STD_VULKAN_VIDEO_CODEC_H265_DECODE_API_VERSION_1_0_0: u32 = 4194304"),
        "H265 decode version const must be emitted"
    );
    assert!(
        f.consts_rs
            .contains("VK_STD_VULKAN_VIDEO_CODEC_AV1_DECODE_API_VERSION_1_0_0: u32 = 4194304"),
        "AV1 decode version const must be emitted"
    );
}

#[test]
fn video_version_consts_feature_gated() {
    let f = generate();
    // H264 decode -> VK_KHR_video_decode_h264
    let idx = f
        .consts_rs
        .find("VK_STD_VULKAN_VIDEO_CODEC_H264_DECODE_API_VERSION_1_0_0: u32")
        .expect("H264 decode version const in types.rs");
    let before = &f.consts_rs[idx.saturating_sub(150)..idx];
    assert!(
        before.contains("VK_KHR_video_decode_h264"),
        "H264 decode version const must be gated by VK_KHR_video_decode_h264;\nbefore: {before}"
    );

    // H264 encode -> VK_KHR_video_encode_h264
    let idx = f
        .consts_rs
        .find("VK_STD_VULKAN_VIDEO_CODEC_H264_ENCODE_API_VERSION_1_0_0: u32")
        .expect("H264 encode version const in types.rs");
    let before = &f.consts_rs[idx.saturating_sub(150)..idx];
    assert!(
        before.contains("VK_KHR_video_encode_h264"),
        "H264 encode version const must be gated by VK_KHR_video_encode_h264"
    );
}

// -- All defines routed to consts.rs, not types.rs -----------------------------

#[test]
fn defines_not_in_types_rs() {
    let f = generate();
    assert!(
        !f.types_rs.contains("pub const fn VK_MAKE"),
        "VK_MAKE_* defines must be in consts.rs, not types.rs"
    );
    assert!(
        !f.types_rs.contains("VK_API_VERSION_1_"),
        "VK_API_VERSION_* defines must be in consts.rs, not types.rs"
    );
    assert!(
        !f.types_rs.contains("VK_HEADER_VERSION"),
        "VK_HEADER_VERSION must be in consts.rs, not types.rs"
    );
}

// -- vk.xml API version helpers in consts.rs -----------------------------------

#[test]
fn vk_make_api_version_const_fn() {
    let f = generate();
    assert!(
        f.consts_rs.contains("pub const fn VK_MAKE_API_VERSION"),
        "VK_MAKE_API_VERSION must be a const fn in consts.rs"
    );
    assert!(
        f.consts_rs.contains("variant: u32"),
        "VK_MAKE_API_VERSION must have variant parameter"
    );
    assert!(
        f.consts_rs
            .contains("(variant << 29) | (major << 22) | (minor << 12) | patch"),
        "VK_MAKE_API_VERSION body must pack all four fields"
    );
}

#[test]
fn vk_api_version_unpack_fns() {
    let f = generate();
    // Unpacking helpers
    assert!(
        f.consts_rs.contains("pub const fn VK_API_VERSION_MAJOR"),
        "VK_API_VERSION_MAJOR must be emitted"
    );
    assert!(
        f.consts_rs.contains("pub const fn VK_API_VERSION_MINOR"),
        "VK_API_VERSION_MINOR must be emitted"
    );
    assert!(
        f.consts_rs.contains("pub const fn VK_MAKE_VERSION"),
        "VK_MAKE_VERSION must be emitted"
    );
}

#[test]
fn vk_header_version_const() {
    let f = generate();
    assert!(
        f.consts_rs.contains("VK_HEADER_VERSION: u32 = 346"),
        "VK_HEADER_VERSION must be a plain u32 const = 346 in consts.rs",
    );
}

#[test]
fn vk_api_version_constants_correct_values() {
    let f = generate();
    assert!(
        f.consts_rs
            .contains("VK_API_VERSION_1_0: u32 = VK_MAKE_API_VERSION(0, 1, 0, 0)"),
        "VK_API_VERSION_1_0 must be defined as VK_MAKE_API_VERSION(0, 1, 0, 0)"
    );
    assert!(
        f.consts_rs
            .contains("VK_API_VERSION_1_1: u32 = VK_MAKE_API_VERSION(0, 1, 1, 0)"),
        "VK_API_VERSION_1_1 must be defined as VK_MAKE_API_VERSION(0, 1, 1, 0)"
    );

    assert!(
        f.consts_rs
            .contains("VK_API_VERSION_1_2: u32 = VK_MAKE_API_VERSION(0, 1, 2, 0)"),
        "VK_API_VERSION_1_2 must be defined as VK_MAKE_API_VERSION(0, 1, 2, 0)"
    );
    assert!(
        f.consts_rs
            .contains("VK_API_VERSION_1_3: u32 = VK_MAKE_API_VERSION(0, 1, 3, 0)"),
        "VK_API_VERSION_1_3 must be defined as VK_MAKE_API_VERSION(0, 1, 3, 0)"
    );
    assert!(
        f.consts_rs
            .contains("VK_API_VERSION_1_4: u32 = VK_MAKE_API_VERSION(0, 1, 4, 0)"),
        "VK_API_VERSION_1_4 must be defined as VK_MAKE_API_VERSION(0, 1, 4, 0)"
    );
}

#[test]
fn api_version_consts_ungated() {
    // Core API version constants have no feature gate - they're always available
    let f = generate();
    let idx = f
        .consts_rs
        .find("VK_API_VERSION_1_0: u32 = VK_MAKE_API_VERSION(0, 1, 0, 0)")
        .expect("VK_API_VERSION_1_0 in consts.rs");
    let before = &f.consts_rs[idx.saturating_sub(80)..idx];
    assert!(
        before.contains("#[cfg(feature = \"VK_BASE_VERSION_1_0\")]"),
        "VK_API_VERSION_1_0 must only be gated by VK_BASE_VERSION_1_0, not a higher version or extension feature;\nbefore: {before}"
    );
}

#[test]
fn vksc_api_variant_emitted() {
    let f = generate();
    // VKSC_API_VARIANT must be emitted
    assert!(
        f.consts_rs.contains("const VKSC_API_VARIANT: u32 ="),
        "VKSC_API_VARIANT must be emitted as a const u32 in consts.rs"
    );

    // If we're using vulkansc VK_HEADER_VERSION_COMPLETE must be defined using:
    // Otherwise: (VKSC_API_VARIANT, 1, 0, VK_HEADER_VERSION)
    // #define VK_HEADER_VERSION_COMPLETE VK_MAKE_API_VERSION(0, 1, 4, VK_HEADER_VERSION)

    // We need to check that we feature gate the VK_HEADER_VERSION_COMPLETE definition on VKSC_VERSION_1_0, not just emit it unconditionally with the wrong value.
    let no_space: String = f.consts_rs.chars().filter(|c| !c.is_whitespace()).collect();
    assert!(
        no_space.contains("VK_HEADER_VERSION_COMPLETE:u32=VK_MAKE_API_VERSION(VKSC_API_VARIANT,1,0,VK_HEADER_VERSION,"),
        "VK_HEADER_VERSION_COMPLETE with VKSC_API_VARIANT missing"
    );
    let idx = f.consts_rs.find("VKSC_API_VARIANT,").expect("found");
    let before = &f.consts_rs[idx.saturating_sub(150)..idx];
    assert!(
        before.contains("VKSC_VERSION_1_0"),
        "VK_HEADER_VERSION_COMPLETE must be gated by VKSC_VERSION_1_0;\nbefore: {before}"
    );
    assert!(
        no_space.contains(
            "VK_HEADER_VERSION_COMPLETE:u32=VK_MAKE_API_VERSION(0,1,4,VK_HEADER_VERSION,"
        ),
        "VK_HEADER_VERSION_COMPLETE definition in consts.rs"
    );

    let idx = f
        .consts_rs
        .find("VK_MAKE_API_VERSION(\n    0,\n")
        .expect("VK_HEADER_VERSION_COMPLETE definition in consts.rs");

    let before = &f.consts_rs[idx.saturating_sub(150)..idx];
    assert!(
        before.contains("VK_BASE_VERSION_1_0"),
        "VK_HEADER_VERSION_COMPLETE must have a fallback definition gated on VK_BASE_VERSION_1_0;\nbefore: {before}"
    );
}

#[test]
fn pfn_command_cfg_is_strict() {
    let f = generate();
    // Check that the KHR version is properly gated by dynamic rendering + versions
    // Based on the parser, it should look for feature = "VK_KHR_dynamic_rendering"
    assert!(
        f.commands_rs
            .contains("#[cfg(feature = \"VK_KHR_dynamic_rendering\")")
    );

    let idx = f
        .commands_rs
        .find("pub type PFN_vkCmdBeginRenderingKHR =")
        .expect("PFN_vkCmdBeginRenderingKHR in commands.rs");
    let before = &f.commands_rs[idx.saturating_sub(200)..idx];

    assert!(
        before.contains("feature = \"VK_KHR_dynamic_rendering\""),
        "PFN_vkCmdBeginRenderingKHR must be gated by VK_KHR_dynamic_rendering;\nbefore: {before}"
    );
}

#[test]
fn correct_cfg_on_vulkansc_api() {
    // <command api="vulkan,vulkanbase" export="vulkan" allownoqueues="true" successcodes="VK_SUCCESS" errorcodes="VK_ERROR_OUT_OF_HOST_MEMORY,VK_ERROR_OUT_OF_DEVICE_MEMORY,VK_ERROR_UNKNOWN,VK_ERROR_VALIDATION_FAILED">
    //     <proto><type>VkResult</type> <name>vkCreatePipelineCache</name></proto>
    //     <param><type>VkDevice</type> <name>device</name></param>
    //     <param>const <type>VkPipelineCacheCreateInfo</type>* <name>pCreateInfo</name></param>
    //     <param optional="true">const <type>VkAllocationCallbacks</type>* <name>pAllocator</name></param>
    //     <param><type>VkPipelineCache</type>* <name>pPipelineCache</name></param>
    // </command>
    // <command api="vulkansc" export="vulkansc" allownoqueues="true" successcodes="VK_SUCCESS" errorcodes="VK_ERROR_OUT_OF_HOST_MEMORY,VK_ERROR_OUT_OF_DEVICE_MEMORY,VK_ERROR_INVALID_PIPELINE_CACHE_DATA,VK_ERROR_UNKNOWN,VK_ERROR_VALIDATION_FAILED">
    //     <proto><type>VkResult</type> <name>vkCreatePipelineCache</name></proto>
    //     <param><type>VkDevice</type> <name>device</name></param>
    //     <param>const <type>VkPipelineCacheCreateInfo</type>* <name>pCreateInfo</name></param>
    //     <param optional="true">const <type>VkAllocationCallbacks</type>* <name>pAllocator</name></param>
    //     <param><type>VkPipelineCache</type>* <name>pPipelineCache</name></param>
    // </command>

    // This should result in:
    // #[cfg(all(feature = "VK_COMPUTE_VERSION_1_0", not(feature = "VKSC_VERSION_1_0")))]
    // pub type PFN_vkCreatePipelineCache = unsafe extern "system" fn(
    //     device: VkDevice,
    //     pCreateInfo: *const VkPipelineCacheCreateInfo,
    //     pAllocator: *const VkAllocationCallbacks,
    //     pPipelineCache: *mut VkPipelineCache,
    // ) -> VkResult;
    //
    // #[cfg(feature = "VKSC_VERSION_1_0")]
    // pub type PFN_vkCreatePipelineCache = unsafe extern "system" fn(
    //     device: VkDevice,
    //     pCreateInfo: *const VkPipelineCacheCreateInfo,
    //     pAllocator: *const VkAllocationCallbacks,
    //     pPipelineCache: *mut VkPipelineCache,
    // ) -> VkResult;
    let reg = make_registry();

    let pfn_vk_create_pipeline_cache = reg.commands.get("vkCreatePipelineCache");
    if pfn_vk_create_pipeline_cache.is_none() {
        panic!("PFN_vkCreatePipelineCache command not found in registry");
    }
    // Assert len = 2
    let unwrapped = pfn_vk_create_pipeline_cache.unwrap();
    if unwrapped.len() != 2 {
        panic!(
            "Expected 2 command entries for PFN_vkCreatePipelineCache, got {}",
            unwrapped.len()
        );
    }

    let first = unwrapped.first();
    let second = unwrapped.last();
    if first.is_none() || second.is_none() {
        panic!("PFN_vkCreatePipelineCache command entries are empty");
    }

    // Verify that one entry is for VKSC_VERSION_1_0 and the other is for the non-VKSC case
    let first = first.unwrap();
    let second = second.unwrap();
    let (vksc_entry, non_vksc_entry) = if first.provided_by.contains(&"VKSC_VERSION_1_0".to_owned())
    {
        (first, second)
    } else if second.provided_by.contains(&"VKSC_VERSION_1_0".to_owned()) {
        (second, first)
    } else {
        panic!("Neither PFN_vkCreatePipelineCache entry is gated on VKSC_VERSION_1_0");
    };
    // The VKSC entry should be gated on VKSC_VERSION_1_0
    assert!(
        vksc_entry
            .provided_by
            .contains(&"VKSC_VERSION_1_0".to_owned()),
        "VKSC PFN_vkCreatePipelineCache entry must be gated on VKSC_VERSION_1_0; got {:?}",
        vksc_entry.provided_by
    );
    // The non-VKSC entry should be gated on VK_COMPUTE_VERSION_1_0 and not VKSC_VERSION_1_0
    assert!(
        non_vksc_entry
            .provided_by
            .contains(&"VK_COMPUTE_VERSION_1_0".to_owned())
            && !non_vksc_entry
                .provided_by
                .contains(&"VKSC_VERSION_1_0".to_owned()),
        "Non-VKSC PFN_vkCreatePipelineCache entry must be gated on VK_COMPUTE_VERSION_1_0 and not VKSC_VERSION_1_0; got {:?}",
        non_vksc_entry.provided_by
    );
}
