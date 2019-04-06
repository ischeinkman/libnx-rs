use native;

pub fn init()
{
    unsafe
    {
        native::consoleInit(std::ptr::null_mut());
    }
}

pub fn flush()
{
    unsafe
    {
        native::consoleUpdate(std::ptr::null_mut());
    }
}

pub fn clear()
{
    unsafe
    {
        native::consoleClear();
    }
}

pub fn exit()
{
    unsafe
    {
        native::consoleExit(std::ptr::null_mut());
    }
}