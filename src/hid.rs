use nx;

pub enum Controller
{
    Handleld,
    Player(u8),
}

pub enum Key
{
    // Fill this
}

pub enum JoyConHoldMode
{
    Default,
    Horizontal,
}

/*
unsafe fn native_to_enum(id: nx::HidControllerID) -> Controller
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
        nx::HidControllerID_CONTROLLER_HANDHELD => Controller::Handleld,
    }
}
*/

pub fn flush()
{
    unsafe
    {
        nx::hidScanInput();
    }
}

pub fn input_down() -> u64
{
    unsafe
    {
        flush();
        nx::hidKeysDown(nx::HidControllerID_CONTROLLER_P1_AUTO)
    }
}

pub fn input_up() -> u64
{
    unsafe
    {
        flush();
        nx::hidKeysUp(nx::HidControllerID_CONTROLLER_P1_AUTO)
    }
}

pub fn input_held() -> u64
{
    unsafe
    {
        flush();
        nx::hidKeysHeld(nx::HidControllerID_CONTROLLER_P1_AUTO)
    }
}

pub fn touch_coords() -> (u32, u32)
{
    unsafe
    {
        flush();
        let mut tch : nx::touchPosition = core::mem::zeroed();
        nx::hidTouchRead(&mut tch, 0);
        (tch.px, tch.py)
    }
}