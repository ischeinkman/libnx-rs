use native;

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

fn ctrlid_to_controller(id: native::HidControllerID) -> Controller
{
    match id
    {
        native::HidControllerID_CONTROLLER_PLAYER_1 => Controller::Player(1),
        native::HidControllerID_CONTROLLER_PLAYER_2 => Controller::Player(2),
        native::HidControllerID_CONTROLLER_PLAYER_3 => Controller::Player(3),
        native::HidControllerID_CONTROLLER_PLAYER_4 => Controller::Player(4),
        native::HidControllerID_CONTROLLER_PLAYER_5 => Controller::Player(5),
        native::HidControllerID_CONTROLLER_PLAYER_6 => Controller::Player(6),
        native::HidControllerID_CONTROLLER_PLAYER_7 => Controller::Player(7),
        native::HidControllerID_CONTROLLER_PLAYER_8 => Controller::Player(8),
        native::HidControllerID_CONTROLLER_HANDHELD => Controller::Handheld,
        _ => Controller::Invalid
    }
}

fn controller_to_ctrlid(id: Controller) -> native::HidControllerID
{
    match id
    {
        Controller::Player(1) => native::HidControllerID_CONTROLLER_PLAYER_1,
        Controller::Player(2) => native::HidControllerID_CONTROLLER_PLAYER_2,
        Controller::Player(3) => native::HidControllerID_CONTROLLER_PLAYER_3,
        Controller::Player(4) => native::HidControllerID_CONTROLLER_PLAYER_4,
        Controller::Player(5) => native::HidControllerID_CONTROLLER_PLAYER_5,
        Controller::Player(6) => native::HidControllerID_CONTROLLER_PLAYER_6,
        Controller::Player(7) => native::HidControllerID_CONTROLLER_PLAYER_7,
        Controller::Player(8) => native::HidControllerID_CONTROLLER_PLAYER_8,
        Controller::Handheld => native::HidControllerID_CONTROLLER_HANDHELD,
        _ => native::HidControllerID_CONTROLLER_UNKNOWN,
    }
}

fn key_to_enum(id: native::HidControllerKeys) -> Key
{
    // TODO: Port all keys
    Key::None
}

pub fn is_controller_connected(ctrl: Controller) -> bool
{
    unsafe
    {
        native::hidIsControllerConnected(controller_to_ctrlid(ctrl))
    }
}

pub fn flush()
{
    unsafe
    {
        native::hidScanInput();
    }
}

pub fn input_down(ctrl: Controller) -> u64
{
    unsafe
    {
        flush();
        native::hidKeysDown(controller_to_ctrlid(ctrl))
    }
}

pub fn input_up(ctrl: Controller) -> u64
{
    unsafe
    {
        flush();
        native::hidKeysUp(controller_to_ctrlid(ctrl))
    }
}

pub fn input_held(ctrl: Controller) -> u64
{
    unsafe
    {
        flush();
        native::hidKeysHeld(controller_to_ctrlid(ctrl))
    }
}

pub fn get_touch_count() -> u32
{
    unsafe
    {
        native::hidTouchCount()
    }
}

pub fn get_touch_coords(index: u32) -> (u32, u32)
{
    unsafe
    {
        flush();
        let mut tch: native::touchPosition = std::mem::zeroed();
        native::hidTouchRead(&mut tch, index);
        (tch.px, tch.py)
    }
}