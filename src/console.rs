use sys;

pub fn init()
{
    unsafe
    {
        sys::consoleInit(std::ptr::null_mut());
    }
}

pub fn flush()
{
    unsafe
    {
        sys::consoleUpdate(std::ptr::null_mut());
    }
}

pub fn clear()
{
    unsafe
    {
        sys::consoleClear();
    }
}

pub fn exit()
{
    unsafe
    {
        sys::consoleExit(std::ptr::null_mut());
    }
}