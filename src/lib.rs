#![cfg_attr(feature = "sysroot", no_std)]

#![cfg_attr(not(feature = "sysroot"), feature(asm))]
#![macro_use]

#[cfg(not(feature = "sysroot"))]
extern crate core;

#[cfg(not(feature = "sysroot"))]
pub mod macros;

#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
#[allow(clippy::all)]
#[allow(clippy::pedantic)]
pub mod sys;

#[cfg(not(feature = "sysroot"))]
pub mod sm;

#[cfg(not(feature = "sysroot"))]
pub mod console;

#[cfg(not(feature = "sysroot"))]
pub mod hid;

#[cfg(not(feature = "sysroot"))]
pub mod applet;

#[cfg(not(feature = "sysroot"))]
pub mod os;

#[cfg(feature = "twili")]
pub mod twili;
