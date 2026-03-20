use crate::codegen::pretty;
use quote::quote;

pub fn gen_lib_rs() -> String {
    let ts = quote! {
        //! Auto-generated Vulkan FFI (`vk-sys`).
        //!
        //! Generated from `vk.xml` + `video.xml` by `vk-codegen`.
        //!
        //! Every item is gated by `#[cfg(feature = "...")]` mirroring the Vulkan
        //! API version or extension name exactly.  Enabling a feature via Cargo
        //! automatically pulls in all transitive dependencies.
        //!
        //! ```toml
        //! [dependencies.vk-sys]
        //! features = ["VK_VERSION_1_3", "VK_KHR_swapchain"]
        //! ```
        #![no_std]
        #![allow(
            non_snake_case, non_camel_case_types, non_upper_case_globals,
            dead_code, unused_imports, clippy::all
        )]

        pub mod consts;
        pub mod enums;
        pub mod types;
        pub mod commands;
        pub mod loader;
        pub mod validation;

        pub use consts::*;
        pub use enums::*;
        pub use types::*;
        pub use commands::*;
        pub use loader::*;
        pub use validation::*;
    };
    pretty(ts)
}
