use sys;

pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub micro: u8,
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.micro)
    }
}

#[allow(clippy::cast_possible_truncation)]
fn hosv_to_version(v: u32) -> Version {
    let tmpmajor: u8 = ((v >> 16) & 0xff) as u8;
    let tmpminor: u8 = ((v >> 8) & 0xff) as u8;
    let tmpmicro: u8 = (v & 0xff) as u8;
    Version {
        major: tmpmajor,
        minor: tmpminor,
        micro: tmpmicro,
    }
}

pub fn get() -> Version {
    unsafe {
        let hosv: u32 = sys::hosversionGet();
        hosv_to_version(hosv)
    }
}

#[deprecated(note = "Please use version::get() instead.")]
#[allow(clippy::module_name_repetitions)]
pub fn get_version() -> Version {
    get()
}
