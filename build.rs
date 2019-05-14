extern crate cfg_if;

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "bindgen")] {
        extern crate bindgen;
        use bindgen::callbacks::{ EnumVariantCustomBehavior, EnumVariantValue, IntKind, MacroParsingBehavior, ParseCallbacks };

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

        pub fn regen_libnx_native(input: &str, output: &str) -> Result<bindgen::Bindings, std::io::Error>
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
            bindgen::Builder::default().trust_clang_mangling(false).use_core().rust_target(bindgen::RustTarget::Nightly).ctypes_prefix("ctypes").generate_inline_functions(true).parse_callbacks(Box::new(CustomCallbacks{})).header(input).clang_arg(ilibnx).clang_arg(igcc1).clang_arg(igcc2).blacklist_type("u8").blacklist_type("u16").blacklist_type("u32").blacklist_type("u64").generate().map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Could not create file!")).and_then(|bnd| bnd.write_to_file(output).map(|_| bnd))
        }

        pub fn bindgen()
        {
            // Where the Rust code will be generated to
            let gen_path = "bindgen/libnx.rs";

            // Input header
            let header_wrapper = "bindgen/libnx.h";

            // Use bindgen crate to process libnx headers
            regen_libnx_native(header_wrapper, gen_path).expect("Error generating libnx bindings!");
        }
    } else {
        pub fn bindgen() {
            if !std::path::Path::new("bindgen/libnx.rs").exists() {
                panic!("Bindgen disabled but output missing!");
            }
        }
    }
}

pub fn main() {
    bindgen();
}
