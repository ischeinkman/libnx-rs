use native;

pub fn get_current_process_handle() -> u32
{
    0xffff8001
}

pub fn is_nso() -> bool
{
    unsafe
    {
        native::envIsNso()
    }
}

pub fn is_nro() -> bool
{
    !is_nso()
}

pub fn nro_exec(path: String)
{
    unsafe
    {
        native::envSetNextLoad(path.as_ptr(), path.as_ptr());
        std::process::exit(0);
    }
}