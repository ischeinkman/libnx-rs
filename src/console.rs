#![cfg(not(feature="sysroot"))]

use super::LibnxError;
use super::libnx::{
    lang_items,
    consoleInit, 
    PrintConsole,
    consoleClear,
    consoleUpdate, 
    consoleExit
};

use std::ptr;

pub struct ConsoleHandle {
    inner : *mut PrintConsole
}

impl ConsoleHandle {
    pub fn init_default() -> ConsoleHandle {
        unsafe {
            consoleInit(ptr::null_mut());
        }
        ConsoleHandle {
            inner : ptr::null_mut()
        }
    }

    pub fn update(&mut self) {
        unsafe {
            consoleUpdate(self.inner);
        }
    }

    pub fn clear(&mut self) {
        unsafe {
            consoleClear();
        }
    }
}

impl Default for ConsoleHandle {
    fn default() -> ConsoleHandle {
        ConsoleHandle::init_default()
    }
}

impl Drop for ConsoleHandle {
    fn drop(&mut self) {
        unsafe {
            consoleExit(self.inner);
        }
    }
}