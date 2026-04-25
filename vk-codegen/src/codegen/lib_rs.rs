use crate::codegen::pretty;
use quote::quote;

pub fn gen_lib_rs(handles: &[String]) -> String {
    let mut ts = quote! {
        #![no_std]
        #![allow(
            non_snake_case,
            non_camel_case_types,
            non_upper_case_globals,
            dead_code,
            unused_imports,
            clippy::all,
            deprecated,
        )]

        extern crate alloc;

        mod commands;
        mod consts;
        mod entry;
        mod enums;
        mod device;
        mod physical_device;
        mod instance;
        mod types;
        mod validation;
    };

    for h in handles {
        let ident = quote::format_ident!("{}", h);
        ts.extend(quote! { mod #ident; });
    }

    ts.extend(quote! {
        pub use commands::*;
        pub use consts::*;
        pub use core::ptr::null;
        pub use core::ptr::null_mut;
        pub use device::*;
        pub use entry::*;
        pub use enums::*;
        pub use physical_device::*;
        pub use instance::*;
        pub use types::*;
        pub use validation::*;
    });

    for h in handles {
        let ident = quote::format_ident!("{}", h);
        ts.extend(quote! { pub use #ident::*; });
    }
    pretty(&ts)
}
