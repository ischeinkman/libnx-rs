use os;
use sys;

service!(Handle, sys::smInitialize(), true, sys::smExit(), {
    pub fn get_service(&self, name: &str) -> os::Result<sys::Service> {
        unsafe {
            let mut srv: sys::Service = std::mem::zeroed();
            let rc = sys::smGetService(&mut srv, name.as_ptr());
            result_final!(rc, srv)
        }
    }
});
