use std::sync::atomic::{AtomicBool, Ordering};

#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
#[allow(clippy::all)]
#[allow(clippy::pedantic)]
pub mod sys;

static INITIALIZED: AtomicBool = AtomicBool::new(false);

pub struct Handle(());

impl Drop for Handle {
    fn drop(&mut self) {
        unsafe { sys::twiliExit(); };
    }
}

pub fn init() -> Option<Handle> {
    if !INITIALIZED.swap(true, Ordering::SeqCst) {
        let res = unsafe { sys::twiliInitialize() };
        assert_eq!(res, 0);
        Some(Handle(()))
    } else {
        None
    }
}
