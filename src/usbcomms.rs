use super::LibnxError;
use super::libnx::{lang_items, usbCommsInitializeEx, usbCommsExitEx, usbCommsReadEx, usbCommsWriteEx};

#[derive(Debug)]
pub struct UsbCommsInterface {
    pub id : u32, 
    pub class : u8, 
    pub subclass : u8, 
    pub protocol : u8,
}

impl UsbCommsInterface {
    pub fn new(class : u8, subclass : u8, protocol : u8) -> Result<UsbCommsInterface, LibnxError> {
        let mut idx = 0u32;
        let rval = unsafe { usbCommsInitializeEx(&mut idx as *mut _, class, subclass, protocol) };
        if rval == 0 {
            Ok(UsbCommsInterface {
                id : idx, 
                class, 
                subclass, 
                protocol
            })
        }
        else {
            Err(LibnxError::from_raw(rval))
        }
    }

    pub fn default() -> Result<UsbCommsInterface, LibnxError> {
        UsbCommsInterface::new(255, 255, 255)
    }

    pub fn write_bytes(&mut self, bytes : &[u8]) -> usize {
        let ln = bytes.len();
        let begin = bytes.as_ptr();
        unsafe {
            usbCommsWriteEx(begin as *const lang_items::c_void, ln, self.id)
        }
    }

    pub fn read_bytes(&mut self, output : &mut [u8]) -> usize {
        let ln = output.len();
        let begin = output.as_ptr();
        unsafe {
            usbCommsReadEx(begin as *mut lang_items::c_void, ln, self.id)
        }
    }
}

impl Drop for UsbCommsInterface {
    fn drop(&mut self) {
        unsafe {
            usbCommsExitEx(self.id);
        }
    }
}