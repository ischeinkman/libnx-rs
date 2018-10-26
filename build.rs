extern crate bindgen;

use std::env;
use std::path::PathBuf;

use bindgen::callbacks::{ParseCallbacks, MacroParsingBehavior, IntKind, EnumVariantCustomBehavior, EnumVariantValue};

pub fn main() {

    create_bindings("wrapper.h", "bindings.rs")
        .expect("Could not create libnx bindings!");
}

fn create_bindings(infile : &str, outfile : &str) -> Result<bindgen::Bindings, std::io::Error> {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out = out_path.join(outfile);
    
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    bindgen::Builder::default()
        .trust_clang_mangling(false)
        .use_core()
        .rust_target(bindgen::RustTarget::Nightly)
        .ctypes_prefix("lang_items")
        .parse_callbacks(Box::new(CustomCallbacks {}))
        .header(infile)
        .clang_arg("-I/opt/devkitpro/libnx/include")
        .clang_arg("-I/opt/devkitpro/devkitA64/aarch64-none-elf/include")
        .clang_arg("-I/opt/devkitpro/devkitA64/lib/gcc/aarch64-none-elf/8.2.0/include")
        .bitfield_enum("HidMouseButton")
        .bitfield_enum("HidKeyboardModifier")
        .rustified_enum("HidKeyboardScancode")
        .bitfield_enum("HidControllerType")
        .rustified_enum("HidControllerLayoutType")
        .bitfield_enum("HidControllerColorDescription")
        .bitfield_enum("HidControllerKeys")
        .rustified_enum("HidControllerJoystick")
        .bitfield_enum("HidControllerConnectionState")
        .rustified_enum("HidControllerID")
        .blacklist_type("u8")
        .blacklist_type("u16")
        .blacklist_type("u32")
        .blacklist_type("u64")
        .generate()
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Could not create file!"))
        .and_then(|bnd| {
            bnd.write_to_file(out).map(|_| bnd)
        })

}

#[derive(Debug)]
struct CustomCallbacks;
impl ParseCallbacks for CustomCallbacks {
    fn will_parse_macro(&self, _name: &str) -> MacroParsingBehavior {
        MacroParsingBehavior::Default
    }
    
    fn int_macro(&self, _name: &str, _value: i64) -> Option<IntKind> {
        if _name.starts_with("POLL") && _value < i16::max_value() as i64  && _value > i16::min_value() as i64 {
            Some(IntKind::I16)
        }
        else if _name.starts_with("DT_") && _value > 0  && _value < u8::max_value() as i64 {
            Some(IntKind::U8)
        }
        else if _name.starts_with("S_IF") && _value > 0 && _value < u32::max_value() as i64 {
            Some(IntKind::U32)
        }
        else if _value < i32::max_value() as i64 && _value > i32::min_value() as i64 {
            Some(IntKind::I32)
        }
        else {
            None
        }
    }

    fn enum_variant_behavior(&self, _enum_name: Option<&str>, _original_variant_name: &str, _variant_value: EnumVariantValue) -> Option<EnumVariantCustomBehavior> {
        None
    }

    fn enum_variant_name(&self, _enum_name : Option<&str>, _original_variant_name : &str, _variant_value : EnumVariantValue) ->Option<String> {
        None
    }
}
