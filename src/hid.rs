#![macro_use]

use nx;

#[macro_export]
macro_rules! hidany
{
    ($ipt:expr, $($id:expr),*) =>
    {{
        let mut hmatch = false;
        $(
            if ($ipt & ($id as u64)) != 0
            {
                hmatch = true;
            }
        )*
        hmatch
    }};
}

#[macro_export]
macro_rules! hidall
{
    ($ipt:expr, $($id:expr),*) =>
    {{
        let mut hmatch = true;
        $(
            if ($ipt & ($id as u64)) == 0
            {
                hmatch = false;
            }
        )*
        hmatch
    }};
}

pub enum Controller
{
    Invalid,
    Handheld,
    Player(u8),
}

pub enum Key
{
    None,
    A = 1,
    B = 2,
    X = 4,
    Y = 8,
    LStick = 16,
    RStick = 32,
    L = 64,
    R = 128,
    ZL = 256,
    ZR = 512,
}

pub enum JoyConHoldMode
{
    Default,
    Horizontal,
}

fn ctrlid_to_controller(id: nx::HidControllerID) -> Controller
{
    match id
    {
        nx::HidControllerID_CONTROLLER_PLAYER_1 => Controller::Player(1),
        nx::HidControllerID_CONTROLLER_PLAYER_2 => Controller::Player(2),
        nx::HidControllerID_CONTROLLER_PLAYER_3 => Controller::Player(3),
        nx::HidControllerID_CONTROLLER_PLAYER_4 => Controller::Player(4),
        nx::HidControllerID_CONTROLLER_PLAYER_5 => Controller::Player(5),
        nx::HidControllerID_CONTROLLER_PLAYER_6 => Controller::Player(6),
        nx::HidControllerID_CONTROLLER_PLAYER_7 => Controller::Player(7),
        nx::HidControllerID_CONTROLLER_PLAYER_8 => Controller::Player(8),
        nx::HidControllerID_CONTROLLER_HANDHELD => Controller::Handheld,
        _ => Controller::Invalid
    }
}

fn controller_to_ctrlid(id: Controller) -> nx::HidControllerID
{
    match id
    {
        Controller::Player(1) => nx::HidControllerID_CONTROLLER_PLAYER_1,
        Controller::Player(2) => nx::HidControllerID_CONTROLLER_PLAYER_2,
        Controller::Player(3) => nx::HidControllerID_CONTROLLER_PLAYER_3,
        Controller::Player(4) => nx::HidControllerID_CONTROLLER_PLAYER_4,
        Controller::Player(5) => nx::HidControllerID_CONTROLLER_PLAYER_5,
        Controller::Player(6) => nx::HidControllerID_CONTROLLER_PLAYER_6,
        Controller::Player(7) => nx::HidControllerID_CONTROLLER_PLAYER_7,
        Controller::Player(8) => nx::HidControllerID_CONTROLLER_PLAYER_8,
        Controller::Handheld => nx::HidControllerID_CONTROLLER_HANDHELD,
        _ => nx::HidControllerID_CONTROLLER_UNKNOWN,
    }
}

fn key_to_enum(id: nx::HidControllerKeys) -> Key
{
    Key::None
}

pub fn is_controller_connected(ctrl: Controller) -> bool
{
    unsafe
    {
        nx::hidIsControllerConnected(controller_to_ctrlid(ctrl))
    }
}

pub fn flush()
{
    unsafe
    {
        nx::hidScanInput();
    }
}

pub fn input_down(ctrl: Controller) -> u64
{
    unsafe
    {
        flush();
        nx::hidKeysDown(controller_to_ctrlid(ctrl))
    }
}

pub fn input_up(ctrl: Controller) -> u64
{
    unsafe
    {
        flush();
        nx::hidKeysUp(controller_to_ctrlid(ctrl))
    }
}

pub fn input_held(ctrl: Controller) -> u64
{
    unsafe
    {
        flush();
        nx::hidKeysHeld(controller_to_ctrlid(ctrl))
    }
}

pub fn get_touch_count() -> u32
{
    unsafe
    {
        nx::hidTouchCount()
    }
}

pub fn get_touch_coords(index: u32) -> (u32, u32)
{
    unsafe
    {
        flush();
        let mut tch: nx::touchPosition = std::mem::zeroed();
        nx::hidTouchRead(&mut tch, index);
        (tch.px, tch.py)
    }
}