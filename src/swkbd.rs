use nx;

pub struct Keyboard
{
    kbd: nx::SwkbdConfig,
}

pub enum KeyboardPreset
{
    Default,
    Password,
    UserName,
    DownloadCode,
}

impl Keyboard
{
    pub fn new() -> Result<Self, u32>
    {
        unsafe
        {
            let mut kbd: nx::SwkbdConfig = std::mem::zeroed();
            let rc = nx::swkbdCreate(&mut kbd, 0);
            resultfinal!(rc, Keyboard { kbd: kbd })
        }
    }

    pub fn set_preset(&mut self, preset: KeyboardPreset)
    {
        unsafe
        {
            match preset
            {
                KeyboardPreset::Default => nx::swkbdConfigMakePresetDefault(&mut self.kbd),
                KeyboardPreset::Password => nx::swkbdConfigMakePresetPassword(&mut self.kbd),
                KeyboardPreset::UserName => nx::swkbdConfigMakePresetUserName(&mut self.kbd),
                KeyboardPreset::DownloadCode => nx::swkbdConfigMakePresetDownloadCode(&mut self.kbd),
            };
        }
    }

    pub fn set_ok_button_text(&mut self, text: &str)
    {
        unsafe
        {
            nx::swkbdConfigSetOkButtonText(&mut self.kbd, text.as_ptr() as *const nx::c::c_char);
        }
    }

    pub fn show(&mut self) -> Result<String, u32>
    {
        unsafe
        {
            let mut slstr: Vec<u8> = vec![0; 500];
            let slptr = slstr.as_mut_ptr();
            let rc = nx::swkbdShow(&mut self.kbd, slptr, 500);
            resultfinal!(rc, String::from_utf8_lossy(std::slice::from_raw_parts(slptr, 500)).to_string())
        }
    }
}