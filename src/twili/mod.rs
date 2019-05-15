use std::sync::atomic::{AtomicBool, Ordering};

#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
#[allow(clippy::all)]
#[allow(clippy::pedantic)]
pub mod sys;

service!(Handle, sys::twiliInitialize, sys::twiliExit, {});
