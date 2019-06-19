#![cfg_attr(feature = "sysroot", no_std)]
#![cfg_attr(not(feature = "sysroot"), feature(asm))]
#![macro_use]

#[cfg(not(feature = "sysroot"))]
extern crate core;

extern crate cfg_if;

use cfg_if::cfg_if;

#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(clippy::all)]
#[allow(clippy::pedantic)]
pub mod sys;

cfg_if! {
    if #[cfg(not(feature = "sysroot"))] {
        pub mod macros;

        pub mod sm;

        pub mod console;

        pub mod hid;

        pub mod applet;

        pub mod os;

        pub mod usbcomms;

        mod util;
        pub use util::*;
    }
}

#[cfg(feature = "twili")]
pub mod twili;
