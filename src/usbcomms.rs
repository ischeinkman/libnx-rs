use sys;

pub use sys::UsbCommsInterfaceInfo;

use std::sync::atomic::{AtomicBool, Ordering};

use os;
#[derive(Debug)]
pub struct Handle(());

static INITIALIZED: AtomicBool = AtomicBool::new(false);

impl Drop for Handle {
    fn drop(&mut self) {
        if INITIALIZED.swap(false, Ordering::SeqCst) {
            unsafe { sys::usbCommsExit() };
        }
    }
}

impl Handle {
    pub fn new() -> Option<os::Result<Handle>> {
        if !INITIALIZED.swap(true, Ordering::SeqCst) {
            let res = unsafe { sys::usbCommsInitialize() };
            match res {
                0 => Some(Ok(Handle(()))),
                err => Some(Err(err)),
            }
        } else {
            None
        }
    }

    pub fn new_ex(infos: &[UsbCommsInterfaceInfo]) -> Option<os::Result<Handle>> {
        let num_interfaces = infos.len() as u32;
        let infos_ptr = infos as *const _ as *const _;
        if !INITIALIZED.swap(true, Ordering::SeqCst) {
            let res = unsafe { sys::usbCommsInitializeEx(num_interfaces, infos_ptr) };
            match res {
                0 => Some(Ok(Handle(()))),
                err => Some(Err(err)),
            }
        } else {
            None
        }
    }

    pub fn read_ex(&mut self, buffer: &mut [u8], interface: u32) -> usize {
        let buff_ptr = buffer as *mut _ as *mut _;
        let size = buffer.len();
        unsafe { sys::usbCommsReadEx(buff_ptr, size, interface) }
    }

    pub fn read(&mut self, buffer: &mut [u8]) -> usize {
        let buff_ptr = buffer as *mut _ as *mut _;
        let size = buffer.len();
        unsafe { sys::usbCommsRead(buff_ptr, size) }
    }

    pub fn write_ex(&mut self, buffer: &[u8], interface: u32) -> usize {
        let buff_ptr = buffer as *const _ as *const _;
        let size = buffer.len();
        unsafe { sys::usbCommsWriteEx(buff_ptr, size, interface) }
    }

    pub fn write(&mut self, buffer: &[u8]) -> usize {
        let buff_ptr = buffer as *const _ as *const _;
        let size = buffer.len();
        unsafe { sys::usbCommsWrite(buff_ptr, size) }
    }
}
