use os;
use sys;

pub fn initialize() -> os::Result<()> {
    unsafe {
        let rc = sys::smInitialize();
        result_final!(rc)
    }
}

pub fn get_service(name: &str) -> os::Result<sys::Service> {
    unsafe {
        let mut srv: sys::Service = std::mem::zeroed();
        let rc = sys::smGetService(&mut srv, name.as_ptr());
        result_final!(rc, srv)
    }
}

pub fn exit() {
    unsafe {
        sys::smExit();
    }
}
