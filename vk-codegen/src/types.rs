//! C -> Rust type mapping utilities.

use crate::ir::CType;

/// Map a C type name (including multi-word forms) to the Rust equivalent.
/// Returns `""` if the type should be passed through as-is (e.g. `VkDevice`).
pub fn c_type_to_rust(c: &str) -> &'static str {
    match c.trim() {
        "void" => "core::ffi::c_void",
        "char" => "core::ffi::c_char",
        "int" => "core::ffi::c_int",
        "unsigned int" | "uint32_t" => "u32",
        "unsigned long long" | "uint64_t" => "u64",
        "unsigned char" | "uint8_t" => "u8",
        "unsigned short" | "uint16_t" => "u16",
        "signed char" | "int8_t" => "i8",
        "short" | "int16_t" => "i16",
        "int32_t" => "i32",
        "long long" | "int64_t" => "i64",
        "size_t" => "usize",
        "ptrdiff_t" => "isize",
        "float" => "f32",
        "double" => "f64",
        "uintptr_t" => "usize",
        "intptr_t" => "isize",
        _ => "",
    }
}

/// Convert a `CType` to a Rust type string suitable for inserting into source.
pub fn ctype_to_rust_str(ty: &CType) -> String {
    let base = {
        let mapped = c_type_to_rust(&ty.base);
        if mapped.is_empty() {
            if ty.base.is_empty() {
                "core::ffi::c_void".to_owned()
            } else {
                ty.base.clone()
            }
        } else {
            mapped.to_owned()
        }
    };

    if let Some(ref size) = ty.is_array {
        // If the size is a named constant (not a plain integer literal) we must
        // cast it to `usize` so the array type is valid in Rust.
        let size_expr = if size.parse::<u64>().is_ok() {
            size.clone() // plain numeric literal - no cast needed
        } else {
            format!("{size} as usize") // named constant like VK_UUID_SIZE
        };
        return format!("[{base}; {size_expr}]");
    }

    match ty.pointer_depth {
        0 => base,
        1 => {
            if ty.is_const {
                format!("*const {base}")
            } else {
                format!("*mut {base}")
            }
        }
        2 => {
            if ty.is_const {
                format!("*const *const {base}")
            } else {
                format!("*mut *mut {base}")
            }
        }
        _ => format!("*mut {base}"),
    }
}

/// Determine the Rust primitive type for an API constant.
pub fn const_rust_type(xml_type: &str, value: &str) -> &'static str {
    match xml_type.trim() {
        "uint32_t" | "u32" => "u32",
        "uint64_t" | "u64" => "u64",
        "float" | "f32" => "f32",
        "size_t" => "usize",
        _ => {
            let v = value.trim();
            if v.ends_with('f') || v.ends_with('F') || v.contains('.') {
                "f32"
            } else if v.starts_with("0x") || v.starts_with("0X") {
                if v.len() > 10 { "u64" } else { "u32" }
            } else {
                "u32"
            }
        }
    }
}
