#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![feature(asm)]
#![macro_use]
extern crate core;

pub mod macros;

pub mod native;

pub mod sm;

pub mod console;

pub mod hid;

pub mod applet;

pub mod os;