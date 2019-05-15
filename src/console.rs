use sys;

handle!(sys::consoleInit(std::ptr::null_mut()), sys::consoleExit(std::ptr::null_mut()), {
    pub fn clear(&self) {
        unsafe {
            sys::consoleClear();
        }
    }

    pub fn flush(&self) {
        unsafe {
            sys::consoleUpdate(std::ptr::null_mut());
        }
    }
});
