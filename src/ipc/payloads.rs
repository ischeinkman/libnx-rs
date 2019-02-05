

pub trait IpcCommandWriteable {
    fn write(&self, buf : &mut [u32]);
    fn word_count(&self) -> u32;
}

impl <T> IpcCommandWriteable for &[T] where T : IpcCommandWriteable {
    fn write(&self, buf : &mut [u32]) {
        let mut idx : u32 = 0; 
        for itm in self.as_ref().iter() {
            let cur_buf = &mut buf[idx as usize ..];
            if cur_buf.len() < itm.word_count() as usize {
                break;
            }
            itm.write(cur_buf);
            idx += itm.word_count();
        }
    }

    fn word_count(&self) -> u32 {
        self.as_ref().iter().map(|itm| itm.word_count()).sum()
    }
}

impl IpcCommandWriteable for u32 {
    fn write(&self, buf : &mut [u32]) {
        buf[0] = *self;
    }
    fn word_count(&self) -> u32 {
        1
    }
}

impl IpcCommandWriteable for &[u8] {
    fn write(&self, buf : &mut [u32]) {
        for (idx, byte) in self.iter().enumerate() {
            let word_idx = idx / 4usize;
            if word_idx >= buf.len() {
                break;
            }

            let word_off = idx % 4usize;
            let shift = (3 - word_off) * 8;
            let shifted_byte = (*byte as u32) << shift;

            buf[word_idx] |= shifted_byte;
        }
    }

    fn word_count(&self) -> u32{ 
        if self.len() == 0 {
            0
        }
        else { 
            (self.len()/4 + 1) as u32 
        }
    }
}

pub trait IpcCommandReadable {
    fn read(buff : &[u32]) -> Self;
    fn word_count(&self) -> u32;
}

impl IpcCommandReadable for u32 {
    fn read(buff : &[u32]) -> Self {
        buff[0]
    }
    fn word_count(&self) -> u32 {
        1
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RawIpcArgs {
    pub raw_words : Vec<u32>,
}

impl RawIpcArgs {
    pub fn new(raw_words : Vec<u32>) -> Self {
        RawIpcArgs {
            raw_words
        }
    }
}

impl IpcCommandReadable for RawIpcArgs {
    fn read(buff : &[u32]) -> RawIpcArgs {
        let mut raw_words = Vec::with_capacity(buff.len());
        unsafe {raw_words.set_len(buff.len())};
        raw_words.copy_from_slice(buff);

        RawIpcArgs::new(raw_words)
    }
    fn word_count(&self) -> u32 {
        self.raw_words.len() as u32 
    }
}

impl IpcCommandWriteable for RawIpcArgs {
    fn write(&self, buff : &mut [u32]) {
        (&mut buff[0 .. self.raw_words.len()]).copy_from_slice(&self.raw_words);
    }
    fn word_count(&self) -> u32 {
        self.raw_words.len() as u32 
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct EmptyArgs {}

impl EmptyArgs {
    pub const fn new() -> EmptyArgs {
        EmptyArgs {}
    }
}

impl IpcCommandReadable for EmptyArgs {
    fn read(_buf : &[u32]) -> EmptyArgs {
        EmptyArgs{}
    }
    fn word_count(&self) -> u32 {
        0
    }
}
impl IpcCommandWriteable for EmptyArgs {
    fn write(&self, _buff : &mut [u32]) {

    }
    fn word_count(&self) -> u32 {
        0
    }
}