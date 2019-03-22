#![cfg_attr(feature="prebindgen", no_std)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![cfg_attr(not(feature="prebindgen"), feature(asm))]
#[cfg(not(feature="prebindgen"))]
extern crate core;

#[cfg(not(feature="prebindgen"))]
pub mod nx;