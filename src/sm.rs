use native;
use os;

pub fn initialize() -> os::Result<()>
{
    unsafe
    {
        let rc = native::smInitialize();
        result_final!(rc)
    }
}

pub fn get_service(name: &str) -> os::Result<native::Service>
{
    unsafe
    {
        let mut srv: native::Service = std::mem::zeroed();
        let rc = native::smGetService(&mut srv, name.as_ptr());
        result_final!(rc, srv)
    }
}

pub fn exit()
{
    unsafe
    {
        native::smExit();
    }
}