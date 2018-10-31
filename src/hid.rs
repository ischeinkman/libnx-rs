
use super::libnx::{
    hidScanInput, 
    hidKeysHeld,
    hidKeysDown,
    hidKeysUp
};


pub use super::libnx::HidControllerKeys;
pub use super::libnx::HidControllerID;

pub struct HidContext {

}

impl HidContext {
    pub fn scan_input(&mut self) {
        unsafe {
            hidScanInput();
        }
    }

    pub fn get_controller_from_raw(&self, id : u8) -> Controller {
        match id {
            0 => self.get_controller(HidControllerID::CONTROLLER_PLAYER_1),
            1 => self.get_controller(HidControllerID::CONTROLLER_PLAYER_2),
            2 => self.get_controller(HidControllerID::CONTROLLER_PLAYER_3),
            3 => self.get_controller(HidControllerID::CONTROLLER_PLAYER_4),
            4 => self.get_controller(HidControllerID::CONTROLLER_PLAYER_5),
            5 => self.get_controller(HidControllerID::CONTROLLER_PLAYER_6),
            6 => self.get_controller(HidControllerID::CONTROLLER_PLAYER_7),
            7 => self.get_controller(HidControllerID::CONTROLLER_PLAYER_8),
            8 => self.get_controller(HidControllerID::CONTROLLER_HANDHELD),
            9 => self.get_controller(HidControllerID::CONTROLLER_UNKNOWN),
            _ => self.get_controller(HidControllerID::CONTROLLER_P1_AUTO)
        }
    }

    pub fn get_controller(&self, id : HidControllerID) -> Controller {
        Controller {
            controller : id
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Controller {
    controller : HidControllerID
}

impl Controller {

    pub fn keys_down_raw(&self) -> u64 {
        unsafe {
            hidKeysDown(self.controller)
        }
    }
    pub fn keys_up_raw(&self) -> u64 {
        unsafe {
            hidKeysUp(self.controller)
        }
    }

    pub fn keys_held_raw(&self) -> u64 {
        unsafe {
            hidKeysHeld(self.controller)
        }
    }
}