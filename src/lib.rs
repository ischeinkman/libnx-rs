#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]
extern crate core;

pub mod libnx;

pub unsafe fn test()
{
    libnx::consoleInit(std::ptr::null_mut());
    println!("Hello from libnx!");
    loop
    {
        libnx::hidScanInput();
        let k = libnx::hidKeysDown(libnx::HidControllerID_CONTROLLER_P1_AUTO) as u32;
        if (k & libnx::HidControllerKeys_KEY_A) != 0
        {
            println!("A was pressed!");
        }
        else if (k & libnx::HidControllerKeys_KEY_PLUS) != 0
        {
            println!("Plus was pressed. Exiting...");
            libnx::consoleUpdate(std::ptr::null_mut());
            break;
        }
        libnx::consoleUpdate(std::ptr::null_mut());
    }
    libnx::consoleUpdate(std::ptr::null_mut());
}