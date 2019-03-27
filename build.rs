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

pub fn main()
{
    // Where the Rust code will be generated to
    let gen_path = "bindgen/libnx.bindgen.rs";

    // Input header
    let header_wrapper = "bindgen/libnx.h";
    
    std::fs::remove_file(gen_path);

    // Use bindgen crate to process libnx headers
    let res_str = match bindgen::Builder::default().trust_clang_mangling(false).use_core().rust_target(bindgen::RustTarget::Nightly).ctypes_prefix("c").parse_callbacks(Box::new(CustomCallbacks {})).header(header_wrapper).clang_arg("-Ic:\\devkitpro\\libnx\\include").clang_arg("-Ic:\\devkitpro\\devkitA64\\aarch64-none-elf\\include").clang_arg("-Ic:\\devkitpro\\devkitA64\\lib\\gcc\\aarch64-none-elf\\8.2.0\\include").blacklist_type("u8").blacklist_type("u16").blacklist_type("u32").blacklist_type("u64").generate().map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Could not create file!")).and_then(|bnd| bnd.write_to_file(gen_path).map(|_| bnd))
    {
        Ok(_b) =>"Worked fine!",
        Err(_e) => "No fine..."
    };
    println!("{}", res_str);
}