use nx;

pub fn initialize() -> nx::HorizonResult<()>
{
    unsafe
    {
        let rc = nx::smInitialize();
        resultfinal!(rc)
    }
}

pub fn get_service(name: &str) -> nx::HorizonResult<nx::Service>
{
    unsafe
    {
        let mut srv: nx::Service = std::mem::zeroed();
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