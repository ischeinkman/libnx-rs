#![feature(asm)]
#![macro_use]
extern crate core;

pub mod macros;

#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
#[allow(clippy::all)]
#[allow(clippy::pedantic)]
pub mod sys;

pub mod sm;

pub mod console;

pub mod hid;

pub mod applet;

pub mod os;

#[cfg(feature = "twili")]
pub mod twili;
