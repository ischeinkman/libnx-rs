use nx;

pub fn initialize()
{
    unsafe
    {
        nx::consoleInit(core::ptr::null_mut());
    }
}

pub fn flush()
{
    unsafe
    {
        nx::consoleUpdate(core::ptr::null_mut());
    }
}

pub fn clear()
{
    unsafe
    {
        nx::consoleClear();
    }
}

pub fn exit()
{
    unsafe
    {
        nx::consoleExit(core::ptr::null_mut());
    }
}