use nx;

pub fn initialize()
{
    unsafe
    {
        nx::consoleInit(std::ptr::null_mut());
    }
}

pub fn flush()
{
    unsafe
    {
        nx::consoleUpdate(std::ptr::null_mut());
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
        nx::consoleExit(std::ptr::null_mut());
    }
}