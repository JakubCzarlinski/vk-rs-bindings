use quote::quote;

pub fn gen_lib_rs() -> String {
    let ts = quote! {
        #![allow(
            non_snake_case,
            non_camel_case_types,
            non_upper_case_globals,
            dead_code,
            unused_imports,
            clippy::all,
            deprecated,
        )]

        pub mod commands;
        pub mod consts;
        pub mod entry;
        pub mod enums;
        pub mod device;
        pub mod physical_device;
        pub mod instance;
        pub mod queue;
        pub mod command_pool;
        pub mod command_buffer;
        pub mod types;
        pub mod validation;

        pub use commands::*;
        pub use consts::*;
        pub use core::ptr::null;
        pub use device::*;
        pub use entry::*;
        pub use enums::*;
        pub use physical_device::*;
        pub use instance::*;
        pub use queue::*;
        pub use command_pool::*;
        pub use command_buffer::*;
        pub use types::*;
        pub use validation::*;
    };
    crate::codegen::pretty(ts)
}
