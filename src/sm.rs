use nx;

pub unsafe fn initialize() -> Result<(), u32>
{
    let rc = nx::smInitialize();
    resultfinal!(rc)
}

pub unsafe fn get_service(name: &str) -> Result<nx::Service, u32>
{
    let mut srv : nx::Service = core::mem::zeroed();
    let rc = nx::smGetService(&mut srv, name.as_ptr());
    resultfinal!(rc, srv);
}

pub unsafe fn exit()
{
    nx::smExit();
}