#![cfg_attr(feature="sysroot", no_std)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]

#[cfg(not(feature="sysroot"))]
extern crate core;

pub struct LibnxError {
    pub error_code : u32
}

impl LibnxError {
    pub fn from_raw(error : u32) -> LibnxError {
        LibnxError {
            error_code : error
        }
    }
}


pub mod libnx {
    #[cfg(features="make_bindings")]
    pub mod lang_items {
        pub enum c_void {}
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
    }

    #[cfg(features="make_bindings")]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
    #[cfg(not(features="make_bindings"))]
    pub use libnx_bindings::*;
    
}
#[cfg(not(features="make_bindings"))]
mod libnx_bindings;

#[cfg(not(feature="sysroot"))]
pub mod usbcomms;

#[cfg(not(feature="sysroot"))]
pub mod console;