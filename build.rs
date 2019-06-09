extern crate cfg_if;

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "bindgen")] {
        extern crate bindgen;
        use bindgen::callbacks::{ EnumVariantCustomBehavior, EnumVariantValue, IntKind, MacroParsingBehavior, ParseCallbacks };
        use std::fs::OpenOptions;
        use std::io::prelude::*;

        #[derive(Debug)]
        struct CustomCallbacks;

        impl ParseCallbacks for CustomCallbacks
        {
            fn will_parse_macro(&self, _name: &str) -> MacroParsingBehavior
            {
                MacroParsingBehavior::Default
            }

            fn int_macro(&self, _name: &str, _value: i64) -> Option<IntKind>
            {
                if _name.starts_with("POLL") && _value < i16::max_value() as i64 && _value > i16::min_value() as i64
                {
                    Some(IntKind::I16)
                }
                else if _name.starts_with("DT_") && _value > 0 && _value < u8::max_value() as i64
                {
                    Some(IntKind::U8)
                }
                else if _name.starts_with("S_IF") && _value > 0 && _value < u32::max_value() as i64
                {
                    Some(IntKind::U32)
                }
                else if _value < i32::max_value() as i64 && _value > i32::min_value() as i64
                {
                    Some(IntKind::I32)
                }
                else
                {
                    None
                }
            }

            fn enum_variant_behavior(&self, _enum_name: Option<&str>, _original_variant_name: &str, _variant_value: EnumVariantValue,) -> Option<EnumVariantCustomBehavior>
            {
                None
            }

            fn enum_variant_name(&self, _enum_name: Option<&str>, _original_variant_name: &str, _variant_value: EnumVariantValue,) -> Option<String>
            {
                None
            }
        }

        pub fn regen_bindings(input: &str, output: &str, whitelist: Option<Vec<String>>) -> Result<bindgen::Bindings, std::io::Error>
        {
            // we don't care if deletion succeeds, as long as the file is gone
            let _ = std::fs::remove_file(output);
            assert!(!std::path::Path::new(output).exists());

            let iswin = cfg!(windows);
            let ilibnx = if iswin {
                "-Ic:\\devkitpro\\libnx\\include"
            } else {
                "-I/opt/devkitpro/libnx/include"
            };
            let igcc1 = if iswin {
                "-Ic:\\devkitpro\\devkitA64\\aarch64-none-elf\\include"
            } else {
                "-I/opt/devkitpro/devkitA64/aarch64-none-elf/include"
            };
            let igcc2 = if iswin {
                "-Ic:\\devkitpro\\devkitA64\\lib\\gcc\\aarch64-none-elf\\8.3.0\\include"
            } else {
                "-I/opt/devkitpro/devkitA64/lib/gcc/aarch64-none-elf/8.3.0/include/"
            };
            let mut builder = bindgen::Builder::default().clang_arg("-mcrc").trust_clang_mangling(false).use_core().rust_target(bindgen::RustTarget::Nightly).ctypes_prefix("ctypes").generate_inline_functions(true).parse_callbacks(Box::new(CustomCallbacks{})).header(input).clang_arg(ilibnx).clang_arg(igcc1).clang_arg(igcc2).blacklist_type("u8").blacklist_type("u16").blacklist_type("u32").blacklist_type("u64");
            if let Some(whitelist) = whitelist {
                for func in whitelist {
                    builder = builder.whitelist_function(func);
                }
            }

            builder.generate().map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Could not create file!")).and_then(|bnd| {
                let mut file = OpenOptions::new().write(true).create(true).open(output)?;
                file.write_all(br#"mod ctypes {
    pub type c_void = core::ffi::c_void;
    pub type c_char = u8;
    pub type c_int = i32;
    pub type c_long = i64;
    pub type c_longlong = i64;
    pub type c_schar = i8;
    pub type c_short = i16;
    pub type c_uchar = u8;
    pub type c_uint = u32;
    pub type c_ulong = u64;
    pub type c_ulonglong = u64;
    pub type c_ushort = u16;
    pub type size_t = u64;
    pub type ssize_t = i64;
    pub type c_float = f32;
    pub type c_double = f64;
}"#)?;
                bnd.write(Box::new(file)).map(|_| bnd)
            })
        }

        pub fn bindgen()
        {
            // Where the Rust code will be generated to
            let gen_path = "bindgen/libnx.rs";

            // Input header
            let header_wrapper = "bindgen/libnx.h";

            // Use bindgen crate to process libnx headers
            regen_bindings(header_wrapper, gen_path, None).expect("Error generating libnx bindings!");
        }
    } else {
        pub fn bindgen() {
            if !std::path::Path::new("bindgen/libnx.rs").exists() {
                panic!("Bindgen disabled but output missing!");
            }
        }
    }
}

cfg_if! {
    if #[cfg(feature = "twili")] {
        pub fn compile_twili() {
            let mut build = cc::Build::new();
            build.warnings(false);
            build.include("/opt/devkitpro/devkitA64/lib/gcc/aarch64-none-elf/8.3.0/include/");
            build.include("/opt/devkitpro/devkitA64/aarch64-none-elf/include");
            build.include("/opt/devkitpro/libnx/include");
            build.include("twili-libnx/include");
            build.file("twili-libnx/src/twili.c");
            build.target("aarch64-none-elf");
            build.compile("libtwili.a");
        }
    } else {
        pub fn compile_twili() {}
    }
}

cfg_if! {
    if #[cfg(all(feature = "twili", feature = "bindgen"))] {
        pub fn twili_bindgen() {
            regen_bindings("bindgen/twili.h", "bindgen/twili.rs", 
                Some(vec!["twiliWriteNamedPipe".to_string(), "twiliCreateNamedOutputPipe".to_string(), "twiliCreateNamedOutputPipe".to_string(), "twiliInitialize".to_string(), "twiliExit".to_string()])
            ).expect("Error generating twili bindings!");
        }
    } else if #[cfg(feature = "twili")] {
        pub fn twili_bindgen() {
            if !std::path::Path::new("bindgen/twili.rs").exists() {
                panic!("Bindgen disabled but twili output missing!");
            }
        }
    } else {
        pub fn twili_bindgen() {}
    }
}

pub fn main() {
    bindgen();
    compile_twili();
    twili_bindgen();
}
