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
    std::fs::remove_file(output);
    let iswin = cfg!(windows);
    let ilibnx = match iswin
    {
        true => "-Ic:\\devkitpro\\libnx\\include",
        false => "-I/opt/devkitpro/libnx/include",
    };
    let igcc1 = match iswin
    {
        true => "-Ic:\\devkitpro\\devkitA64\\aarch64-none-elf\\include",
        false => "-I/opt/devkitpro/devkitA64/aarch64-none-elf/include",
    };
    let igcc2 = match iswin
    {
        true => "-Ic:\\devkitpro\\devkitA64\\lib\\gcc\\aarch64-none-elf\\8.2.0\\include",
        false => "-I/opt/devkitpro/devkitA64/lib/gcc/aarch64-none-elf/8.2.0/include",
    };
    bindgen::Builder::default().trust_clang_mangling(false).use_core().rust_target(bindgen::RustTarget::Nightly).ctypes_prefix("ctypes").generate_inline_functions(true).parse_callbacks(Box::new(CustomCallbacks{})).header(input).clang_arg(ilibnx).clang_arg(igcc1).clang_arg(igcc2).blacklist_type("u8").blacklist_type("u16").blacklist_type("u32").blacklist_type("u64").generate().map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Could not create file!")).and_then(|bnd| bnd.write_to_file(output).map(|_| bnd))
}

pub fn main()
{
    // Where the Rust code will be generated to
    let gen_path = "bindgen/libnx.rs";

    // Input header
    let header_wrapper = "bindgen/libnx.h";

    // Only build if the output doesn't exist
    if !std::path::Path::new(gen_path).exists()
    {
        // Use bindgen crate to process libnx headers
        match regen_libnx_native(header_wrapper, gen_path)
        {
            Err(e) => panic!("Error generating libnx bindings: '{}'", e),
            _ => {},
        };
    }
}