#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(clippy::all)]
#[allow(clippy::pedantic)]
pub mod sys;

handle!(0 in sys::twiliInitialize(), sys::twiliExit(), {});
