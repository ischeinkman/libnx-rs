#![cfg(not(feature="sysroot"))]
use super::LibnxError;
use super::libnx;
use super::libnx::{lang_items, usbCommsInitializeEx, usbCommsSetErrorHandling, usbCommsExit, usbCommsReadEx, usbCommsWriteEx};

pub const TOTAL_INTERFACES : usize = 4;

pub struct UsbCommsContext {
}

#[cfg(not(no_std))]
use std::mem;

#[cfg(no_std)]
use core::mem;

impl UsbCommsContext {
    pub fn initialize(interfaces : &mut [UsbCommsInterface]) -> Result<UsbCommsContext, LibnxError> {
        let ln = interfaces.len();
        if ln == 0 {
            return Ok(UsbCommsContext{});
        }
        else if ln > TOTAL_INTERFACES {
            return Err(LibnxError::from_msg(format!("Error: too many interfaces! {} > {}.", ln, TOTAL_INTERFACES)));
        }
        let mut base : [libnx::UsbCommsInterfaceInfo ; TOTAL_INTERFACES] = unsafe { mem::zeroed() };
        for idx in 0..ln {
            let cur = &interfaces[idx];
            let raw : libnx::UsbCommsInterfaceInfo = libnx::UsbCommsInterfaceInfo {
                 bInterfaceClass: cur.class,
                 bInterfaceSubClass: cur.subclass,
                 bInterfaceProtocol: cur.protocol,
            };
            base[idx] = raw; 
        }

        unsafe {
            let rval = usbCommsInitializeEx(ln as u32, &base as *const libnx::UsbCommsInterfaceInfo);
            if rval != 0 {
                return Err(LibnxError::from_raw(rval));
            }
        }
        for idx in 0 .. ln {
            let mut cur = &mut interfaces[idx];
            cur.id = idx as u32; 
            cur.is_initialized = true;
        }
        Ok(UsbCommsContext {})
    }

    pub fn set_error_handling(&mut self, flag : bool) {
        unsafe {
            usbCommsSetErrorHandling(flag);
        }
    }
}

impl Drop for UsbCommsContext {
    fn drop(&mut self) {
        unsafe {
            usbCommsExit();
        }
    }
}

#[derive(Debug)]
pub struct UsbCommsInterface {
    pub id : u32, 
    pub class : u8, 
    pub subclass : u8, 
    pub protocol : u8,
    is_initialized : bool,
}

impl Default for UsbCommsInterface {
    fn default() -> Self {
        UsbCommsInterface::new(255, 255, 255)
    }
}
impl UsbCommsInterface {

    pub fn new(class : u8, subclass : u8, protocol : u8) -> UsbCommsInterface {
        UsbCommsInterface {
            id : 0, 
            class, 
            subclass, 
            protocol, 
            is_initialized : false,
        }
    }

    pub fn write_bytes(&mut self, bytes : &[u8]) -> usize {
        if !self.is_initialized {
            return 0;
        }
        let ln = bytes.len();
        let begin = bytes.as_ptr();
        unsafe {
            usbCommsWriteEx(begin as *const lang_items::c_void, ln, self.id)
        }
    }

    pub fn read_bytes(&mut self, output : &mut [u8]) -> usize {
        if !self.is_initialized {
            return 0;
        }
        let ln = output.len();
        let begin = output.as_ptr();
        unsafe {
            usbCommsReadEx(begin as *mut lang_items::c_void, ln, self.id)
        }
    }
}