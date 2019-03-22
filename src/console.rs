use nx;

pub unsafe fn initialize()
{
    nx::consoleInit(core::ptr::null_mut());
}

pub unsafe fn flush()
{
    nx::consoleUpdate(core::ptr::null_mut());
}

pub unsafe fn exit()
{
    nx::consoleExit(core::ptr::null_mut());
}