//! C -> Rust type mapping utilities.

use crate::ir::CType;

/// Maps a C type name to the Rust equivalent.
///
/// Arguments:
///
/// - `c_type_name`: The C type name string to map.
///
/// Returns:
/// - `&'static str`: The Rust type equivalent, or an empty string if it should be passed through as-is.
#[must_use]
pub fn c_type_to_rust(c_type_name: &str) -> &'static str {
    match c_type_name.trim() {
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
        "size_t" | "uintptr_t" => "usize",
        "ptrdiff_t" | "intptr_t" => "isize",
        "float" => "f32",
        "double" => "f64",
        _ => "",
    }
}

/// Converts a `CType` to a Rust type string suitable for inserting into source.
///
/// Arguments:
///
/// - `type_info`: The `CType` IR representation.
///
/// Returns:
/// - `String`: A string representing the equivalent Rust type.
#[must_use]
pub fn ctype_to_rust_str(type_info: &CType) -> String {
    let base_type = {
        let mapped = c_type_to_rust(&type_info.base);
        if mapped.is_empty() {
            if type_info.base.is_empty() {
                "core::ffi::c_void".to_owned()
            } else {
                type_info.base.clone()
            }
        } else {
            mapped.to_owned()
        }
    };

    if let Some(ref array_size) = type_info.is_array {
        // If the size is a named constant (not a plain integer literal) we must
        // cast it to `usize` so the array type is valid in Rust.
        let size_expression = if array_size.parse::<u64>().is_ok() {
            array_size.clone() // plain numeric literal - no cast needed
        } else {
            format!("{array_size} as usize") // named constant like VK_UUID_SIZE
        };
        return format!("[{base_type}; {size_expression}]");
    }

    match type_info.pointer_depth {
        0 => base_type,
        1 => {
            if type_info.is_const {
                format!("*const {base_type}")
            } else {
                format!("*mut {base_type}")
            }
        }
        2 => {
            if type_info.is_const {
                format!("*const *const {base_type}")
            } else {
                format!("*mut *mut {base_type}")
            }
        }
        _ => format!("*mut {base_type}"),
    }
}

/// Determines the Rust primitive type for an API constant.
///
/// Arguments:
///
/// - `xml_type`: The XML type name.
/// - `value`: The raw constant value string.
///
/// Returns:
/// - `&'static str`: The Rust primitive type for the constant.
#[must_use]
pub fn const_rust_type(xml_type: &str, value: &str) -> &'static str {
    match xml_type.trim() {
        "uint32_t" | "u32" => "u32",
        "uint64_t" | "u64" => "u64",
        "float" | "f32" => "f32",
        "size_t" => "usize",
        _ => {
            let constant_value = value.trim();
            if constant_value.ends_with('f')
                || constant_value.ends_with('F')
                || constant_value.contains('.')
            {
                "f32"
            } else if constant_value.starts_with("0x") || constant_value.starts_with("0X") {
                if constant_value.len() > 10 {
                    "u64"
                } else {
                    "u32"
                }
            } else {
                "u32"
            }
        }
    }
}
