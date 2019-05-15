pub mod ctypes {
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
}

include!("../bindgen/libnx.rs");
