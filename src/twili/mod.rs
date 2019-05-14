use std::sync::Once;

#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
#[allow(clippy::all)]
#[allow(clippy::pedantic)]
pub mod sys;

static INIT: Once = Once::new();
static DROP: Once = Once::new();

pub struct Handle(());

impl Drop for Handle {
    fn drop(&mut self) {
        DROP.call_once(|| unsafe { sys::twiliExit(); });
    }
}

pub fn init() -> Handle {
    INIT.call_once(|| {
        let res = unsafe { sys::twiliInitialize() };
        assert_eq!(res, 0);
    });
    Handle(())
}
