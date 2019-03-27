use nx;

pub fn initialize() -> Result<(), u32>
{
    unsafe
    {
        let rc = nx::smInitialize();
        resultfinal!(rc)
    }
}

pub fn get_service(name: &str) -> Result<nx::Service, u32>
{
    unsafe
    {
        let mut srv : nx::Service = core::mem::zeroed();
        let rc = nx::smGetService(&mut srv, name.as_ptr());
        resultfinal!(rc, srv)
    }
}

pub fn exit()
{
    unsafe
    {
        nx::smExit();
    }
}