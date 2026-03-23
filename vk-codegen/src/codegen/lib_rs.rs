use crate::codegen::pretty;
use quote::quote;

pub fn gen_lib_rs() -> String {
    let ts = quote! {
        //! Auto-generated Vulkan FFI (`vk-rs-bindings`).
        //!
        //! Generated from `vk.xml` + `video.xml` by `vk-codegen`.
        //!
        //! Every item is gated by `#[cfg(feature = "...")]` mirroring the Vulkan
        //! API version or extension name exactly.  Enabling a feature via Cargo
        //! automatically pulls in all transitive dependencies.
        //!
        //! ```toml
        //! [dependencies.vk-rs-bindings]
        //! features = ["VK_VERSION_1_3", "VK_KHR_swapchain"]
        //! ```
        #![allow(
            non_snake_case, non_camel_case_types, non_upper_case_globals,
            dead_code, unused_imports, clippy::all, deprecated
        )]

        pub mod commands;
        pub mod consts;
        pub mod device;
        pub mod entry;
        pub mod enums;
        pub mod instance;
        pub mod types;
        pub mod validation;
        pub use commands::*;
        pub use consts::*;
        pub use device::*;
        pub use entry::*;
        pub use enums::*;
        pub use instance::*;
        pub use types::*;
        pub use validation::*;

        pub use core::ptr::null;
    };
    pretty(ts)
}
