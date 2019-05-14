use sys;

#[derive(Clone, Copy)]
pub enum Controller {
    Invalid,
    Handheld,
    Player(u8),
}

pub enum Key {
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
    Plus = 1024,
    Minus = 2048,
    DPadRight = 16384,
    DPadUp = 8192,
    DPadDown = 32768,
    DPadLeft = 4096,
}

pub enum JoyConHoldMode {
    Default,
    Horizontal,
}

pub fn ctrlid_to_controller(id: sys::HidControllerID) -> Controller {
    match id {
        sys::HidControllerID_CONTROLLER_PLAYER_1 => Controller::Player(1),
        sys::HidControllerID_CONTROLLER_PLAYER_2 => Controller::Player(2),
        sys::HidControllerID_CONTROLLER_PLAYER_3 => Controller::Player(3),
        sys::HidControllerID_CONTROLLER_PLAYER_4 => Controller::Player(4),
        sys::HidControllerID_CONTROLLER_PLAYER_5 => Controller::Player(5),
        sys::HidControllerID_CONTROLLER_PLAYER_6 => Controller::Player(6),
        sys::HidControllerID_CONTROLLER_PLAYER_7 => Controller::Player(7),
        sys::HidControllerID_CONTROLLER_PLAYER_8 => Controller::Player(8),
        sys::HidControllerID_CONTROLLER_HANDHELD => Controller::Handheld,
        _ => Controller::Invalid,
    }
}

pub fn controller_to_ctrlid(id: Controller) -> sys::HidControllerID {
    match id {
        Controller::Player(1) => sys::HidControllerID_CONTROLLER_PLAYER_1,
        Controller::Player(2) => sys::HidControllerID_CONTROLLER_PLAYER_2,
        Controller::Player(3) => sys::HidControllerID_CONTROLLER_PLAYER_3,
        Controller::Player(4) => sys::HidControllerID_CONTROLLER_PLAYER_4,
        Controller::Player(5) => sys::HidControllerID_CONTROLLER_PLAYER_5,
        Controller::Player(6) => sys::HidControllerID_CONTROLLER_PLAYER_6,
        Controller::Player(7) => sys::HidControllerID_CONTROLLER_PLAYER_7,
        Controller::Player(8) => sys::HidControllerID_CONTROLLER_PLAYER_8,
        Controller::Handheld => sys::HidControllerID_CONTROLLER_HANDHELD,
        _ => sys::HidControllerID_CONTROLLER_UNKNOWN,
    }
}

pub fn is_controller_connected(ctrl: Controller) -> bool {
    unsafe { sys::hidIsControllerConnected(controller_to_ctrlid(ctrl)) }
}

pub fn flush() {
    unsafe {
        sys::hidScanInput();
    }
}

pub fn input_down(ctrl: Controller) -> u64 {
    unsafe {
        flush();
        sys::hidKeysDown(controller_to_ctrlid(ctrl))
    }
}

pub fn input_up(ctrl: Controller) -> u64 {
    unsafe {
        flush();
        sys::hidKeysUp(controller_to_ctrlid(ctrl))
    }
}

pub fn input_held(ctrl: Controller) -> u64 {
    unsafe {
        flush();
        sys::hidKeysHeld(controller_to_ctrlid(ctrl))
    }
}

pub fn get_touch_count() -> u32 {
    unsafe { sys::hidTouchCount() }
}

pub fn get_touch_coords(index: u32) -> (u32, u32) {
    unsafe {
        flush();
        let mut tch: sys::touchPosition = std::mem::zeroed();
        sys::hidTouchRead(&mut tch, index);
        (tch.px, tch.py)
    }
}
