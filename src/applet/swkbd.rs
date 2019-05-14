use os;
use sys;

pub struct Keyboard {
    kbd: sys::SwkbdConfig,
}

pub enum KeyboardPreset {
    Default,
    Password,
    UserName,
    DownloadCode,
}

impl Keyboard {
    pub fn new() -> os::Result<Self> {
        unsafe {
            let mut kbd: sys::SwkbdConfig = std::mem::zeroed();
            let rc = sys::swkbdCreate(&mut kbd, 0);
            result_final!(rc, Keyboard { kbd: kbd })
        }
    }

    pub fn set_preset(&mut self, preset: KeyboardPreset) {
        unsafe {
            match preset {
                KeyboardPreset::Default => sys::swkbdConfigMakePresetDefault(&mut self.kbd),
                KeyboardPreset::Password => sys::swkbdConfigMakePresetPassword(&mut self.kbd),
                KeyboardPreset::UserName => sys::swkbdConfigMakePresetUserName(&mut self.kbd),
                KeyboardPreset::DownloadCode => {
                    sys::swkbdConfigMakePresetDownloadCode(&mut self.kbd)
                }
            };
        }
    }

    pub fn set_ok_button_text(&mut self, text: String) {
        unsafe {
            sys::swkbdConfigSetOkButtonText(&mut self.kbd, text.as_ptr());
        }
    }

    pub fn show(&mut self) -> os::Result<String> {
        unsafe {
            let mut slstr: Vec<u8> = vec![0; 500];
            let slptr = slstr.as_mut_ptr();
            let rc = sys::swkbdShow(&mut self.kbd, slptr, 500);
            result_final!(
                rc,
                String::from_utf8_lossy(std::slice::from_raw_parts(slptr, 500)).to_string()
            )
        }
    }
}

impl Drop for Keyboard {
    fn drop(&mut self) {
        unsafe {
            sys::swkbdClose(&mut self.kbd);
        }
    }
}
