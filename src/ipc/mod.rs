
use super::libnx;
use super::LibnxError;
use libnx::lang_items::c_void;
use std::mem::size_of;
use std::mem;


mod payloads;
pub use self::payloads::*;

mod domain;
pub use self::domain::*;

mod session;
pub use self::session::*;



pub const IPC_MAX_BUFFERS : usize = 8;
pub const IPC_MAX_OBJECTS : usize = 8;

pub const SFCI_MAGIC : u32 = 0x49434653;
pub const SFCO_MAGIC : u32 = 0x4f434653;

pub struct IpcCommandHeader<ArgsType : IpcCommandWriteable> {
    inner : libnx::IpcCommand,
    payload : ArgsType,
}

impl <ArgsType : IpcCommandWriteable> IpcCommandHeader<ArgsType> {
    pub fn with_args(payload : ArgsType) -> IpcCommandHeader<ArgsType> {
        let inner = unsafe{ mem::zeroed() };
        IpcCommandHeader {
            inner,
            payload,
        }
    }

    pub fn add_buffer<T : Sized>(&mut self, buffer : &[T], kind : BufferType, direction : BufferDirection) {
        let buffer_ptr = buffer as *const [T] as *const c_void;
        let byte_count = buffer.len() * size_of::<T>();
        let off = match direction {
            BufferDirection::Send => self.inner.NumSend,
            BufferDirection::Recieve => self.inner.NumSend + self.inner.NumRecv,
            BufferDirection::Exchange => self.inner.NumSend + self.inner.NumRecv + self.inner.NumExch
        };
        self.inner.Buffers[off] = buffer_ptr;
        self.inner.BufferSizes[off] = byte_count;
        self.inner.BufferTypes[off] = kind.to_raw();
        match direction {
            BufferDirection::Send => {self.inner.NumSend += 1},
            BufferDirection::Recieve => {self.inner.NumRecv += 1},
            BufferDirection::Exchange => {self.inner.NumExch += 1},
        }
    }

    pub fn add_static<T : Sized>(&mut self, buffer : &[T], index : u8, direction : StaticDirection) {
        let buffer_ptr = buffer as *const [T] as *const c_void;
        let byte_count = buffer.len() * size_of::<T>();
        let buffer_index = (index as usize) * size_of::<T>();
        let off = match direction {
            StaticDirection::Send => self.inner.NumStaticIn,
            StaticDirection::Recieve => self.inner.NumStaticIn + self.inner.NumStaticOut,
        };
        self.inner.Statics[off] = buffer_ptr;
        self.inner.StaticSizes[off] = byte_count;
        self.inner.StaticIndices[off] = index;
        match direction {
            StaticDirection::Send => {self.inner.NumStaticIn += 1},
            StaticDirection::Recieve => {self.inner.NumStaticOut += 1},
        }
    }
    pub fn send_handle(&mut self, handle : IpcSession, direction : HandleDirection) {
        let off = match direction {
            HandleDirection::Copy => self.inner.NumHandlesCopy,
            HandleDirection::Move => self.inner.NumHandlesMove + self.inner.NumHandlesCopy,
        };
        self.inner.Handles[off] = handle.handle; 
        match direction {
            HandleDirection::Copy => { self.inner.NumHandlesCopy += 1},
            HandleDirection::Move => { self.inner.NumHandlesMove += 1},
        };
    }
    pub fn set_send_pid_flag(&mut self, new_flag : bool) {
        self.inner.SendPid = new_flag;
    }

    pub fn write_header(&self, buf : &mut [u32]) {
        let mut idx = 0;

        let num_static_part : u32 = (self.inner.NumStaticIn << 16) as u32;
        let num_send_part : u32 = (self.inner.NumSend << 20) as u32;
        let num_recv_part : u32 = (self.inner.NumRecv << 24) as u32;
        let num_exch_part : u32 = (self.inner.NumExch << 28) as u32;
        buf[idx] = (IpcCommandType::Control.to_raw() as u32) | num_static_part | num_recv_part | num_exch_part | num_send_part;
        idx += 1;
        let fill_in_size_later_idx = idx;

        if self.inner.NumStaticOut > 0 {
            buf[idx]= ( (self.inner.NumStaticOut + 2) << 10 ) as u32;
        }
        else {
            buf[idx]= 0;
        }

        if self.inner.SendPid || self.inner.NumHandlesCopy > 0 || self.inner.NumHandlesMove > 0 {
            buf[idx] |= 0x80000000;
            idx += 1;
            let send_pid_bit = if self.inner.SendPid { 1u32 } else { 0u32 };
            buf[idx] = send_pid_bit | (self.inner.NumHandlesCopy << 1) as u32 | (self.inner.NumHandlesMove << 5) as u32;
            idx += 1;

            if self.inner.SendPid {
                idx += 2;
            }

            for i in 0 .. (self.inner.NumHandlesCopy + self.inner.NumHandlesMove) {
                buf[idx] = self.inner.Handles[i];
                idx += 1;
            }
        }
        else {
            idx += 1;
        }

        for i in 0 .. self.inner.NumStaticIn {
            let static_addr = self.inner.Statics[i] as usize as u64;
            let lower_addr_word = (static_addr & u32::max_value() as u64) as u32;

            let upper_addr_bits = (static_addr >> 32) as u32;
            let packed_word =(self.inner.StaticIndices[i] as u32) | (self.inner.StaticSizes[i] << 16) as u32 | ((upper_addr_bits & 15) << 12) | (((upper_addr_bits >> 4) & 15) << 6);
            buf[idx] = packed_word;
            buf[idx + 1] = lower_addr_word;
            idx += 2;
        }

        for i in 0 .. (self.inner.NumSend + self.inner.NumRecv + self.inner.NumExch) {
            let buffer_addr = self.inner.Buffers[i] as usize;
            buf[idx] = self.inner.BufferSizes[i] as u32;
            buf[idx + 1] = (buffer_addr & u32::max_value() as usize) as u32;
            buf[idx + 2] = (((buffer_addr >> 32) & 15) << 28) as u32 | ((buffer_addr >> 36) << 2) as u32 | self.inner.BufferTypes[i];
            idx += 3;
        }

        let padding = (4 - ((idx) % 4)) % 4;
        idx += padding;
        let _raw_idx = idx; 
        let mut raw_size = self.payload.word_count();
        self.payload.write(&mut buf[idx ..]);
        idx += raw_size as usize;


        for i in 0 .. self.inner.NumStaticOut {
            let off = self.inner.NumStaticIn + i;
            let sz = self.inner.StaticSizes[off];
            let to_output : u16= if sz > u16::max_value() as usize { 0 } else { sz as u16};
            if i%2 == 0 {
                buf[idx] |= (to_output << 16) as u32; 
            }
            else {
                buf[idx] |= to_output as u32;
                idx += 1;
            }
        }

        let u16s_size = ((2*self.inner.NumStaticOut) + 3)/4;
        idx += u16s_size;
        raw_size += u16s_size as u32;

        buf[fill_in_size_later_idx] |= raw_size;

        for i in 0 ..self.inner.NumStaticOut {
            let off = self.inner.NumStaticIn + i;
            let static_addr = self.inner.Statics[off] as usize;
            let lower_addr_word = (static_addr & u32::max_value() as usize) as u32;

            let upper_addr_bits = (static_addr >> 32) as u32;
            let packed_word =(self.inner.StaticIndices[i] as u32) | (self.inner.StaticSizes[i] << 16) as u32 | ((upper_addr_bits & 15) << 12) | (((upper_addr_bits >> 4) & 15) << 6);

            buf[idx] = lower_addr_word;
            buf[idx + 1] = packed_word;
            idx += 2;
        }
    }

}

pub struct IpcCommandMessage<T : IpcCommandReadable> {
    inner : libnx::IpcParsedCommand,
    data : T,
}

impl <T : IpcCommandReadable> IpcCommandMessage <T> {
    pub fn parse_from_buffer(buffer : &[u32]) -> IpcCommandMessage<T> {
        let mut r : libnx::IpcParsedCommand = unsafe {mem::zeroed()};

        let ctrl0 : u32 = buffer[0];
        let ctrl1 : u32 = buffer[1];

        let mut idx = 2;

        r.IsDomainRequest = false;
        r.IsDomainResponse = false;

        r.CommandType = ctrl0 & 0xffff;
        r.HasPid = false;
        r.RawSize = ((ctrl1 & 0x1ff) * 4) as usize;
        r.NumHandles = 0;
        
        r.NumStaticsOut = (ctrl1 as usize >> 10) & 15;
        if r.NumStaticsOut >> 1 != 0 {
            r.NumStaticsOut -= 1; // Value 2  . Single descriptor
        }
        if r.NumStaticsOut >> 1 != 0 { 
            r.NumStaticsOut -= 1; // Value 3+ . (Value - 2) descriptors
        }

        if ctrl1 & 0x80000000 != 0 {
            let ctrl2 = buffer[idx];
            idx += 1;

            if ctrl2 & 1 != 0 {
                r.HasPid = true;
                let first_word = buffer[idx] as u64;
                let second_word = buffer[idx + 1] as u64;
                idx += 2;
                r.Pid = (second_word << 32) | (first_word);
            }

            let num_handles_copy = ((ctrl2 >> 1) & 15) as usize;
            let num_handles_move = ((ctrl2 >> 5) & 15) as usize;

            let mut num_handles = num_handles_copy + num_handles_move;
            let idx_after_handles = idx + num_handles;

            if num_handles > IPC_MAX_OBJECTS {
                num_handles = IPC_MAX_OBJECTS;
            }

            for i in 0 .. num_handles {
                r.Handles[i] = buffer[idx + i];
                r.WasHandleCopied[i] = i < num_handles_copy;
            }

            r.NumHandles = num_handles;
            idx = idx_after_handles;
        }

        let mut num_statics = (ctrl0 as usize >> 16) & 15;
        let idx_after_statics = idx + num_statics*2;

        if num_statics > IPC_MAX_BUFFERS {
            num_statics = IPC_MAX_BUFFERS;
        }

        for i in 0..num_statics {
            let static_packed = buffer[idx] as usize;
            let static_addr = buffer[idx + 1];

            r.Statics[i] = (static_addr as usize | (((static_packed >> 12) & 15) << 32) | (((static_packed >> 6) & 15) << 36)) as *mut _;
            r.StaticSizes[i]   = static_packed >> 16;
            r.StaticIndices[i] = (static_packed & 63) as u8;

            idx += 2;
        }
        r.NumStatics = num_statics as usize;
        idx = idx_after_statics;

        let num_bufs_send = ((ctrl0 >> 20) & 15) as usize;
        let num_bufs_recv = ((ctrl0 >> 24) & 15) as usize;
        let num_bufs_exch = ((ctrl0 >> 28) & 15) as usize;

        let mut num_bufs = num_bufs_send + num_bufs_recv + num_bufs_exch;
        let data_nopad_idx = idx + num_bufs * 3;
        let padding_word_count = 4 - (data_nopad_idx % 4);
        let data_idx = data_nopad_idx + padding_word_count;

        let data = T::read(&buffer[data_idx..]);

        if num_bufs > IPC_MAX_BUFFERS as usize {
            num_bufs = IPC_MAX_BUFFERS as usize;
        }

        for i in 0..num_bufs {
            let buff_size = buffer[idx + 0];
            let buff_addr = buffer[idx + 1] as usize;
            let buff_pack = buffer[idx + 2] as usize;
            let ptr_addr = buff_addr | ((buff_pack >> 28) << 32) | (((buff_pack >> 2) & 15) << 36);
            r.Buffers[i] = ptr_addr as *mut _;
            r.BufferSizes[i] = buff_size as usize;
            r.BufferTypes[i] = (buff_pack & 3) as u32;
            idx += 3;
        }
        r.NumBuffers = num_bufs as usize;
        let mut retval = IpcCommandMessage {
            inner : r, 
            data : data
        };
        retval.inner.Raw = &retval.data as *const _ as *mut _;
        return retval;
    }

    pub fn payload(&self) -> &T {
        &self.data
    }

    pub fn buffers<'a>(&'a self) -> impl Iterator<Item=BufferInfo> + 'a {
        let ptr_itr = self.inner.Buffers.iter();
        let sizes_itr = self.inner.BufferSizes.iter();
        let kinds_itr = self.inner.BufferTypes.iter();
        let dir_itr = self.inner.BufferDirections.iter();

        let info_itr = ptr_itr.zip(sizes_itr.zip(kinds_itr.zip(dir_itr)));
        info_itr.map(|(&address, (&size, (&knd, &dir)))|{
            let kind = BufferType::from_raw(knd);
            let direction = BufferDirection::from_raw(dir).unwrap();
            BufferInfo {
                address : address as *const _, 
                size, 
                kind, 
                direction
            }
        })
    }

    pub fn handles(&self) -> &[libnx::Handle] {
        &self.inner.Handles[0 .. self.inner.NumHandles]    
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct BufferInfo {
    pub address : *const c_void,
    pub size : usize, 
    pub kind : BufferType, 
    pub direction : BufferDirection,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum BufferType {
    Normal,
    Type1, 
    Invalid, 
    Type3,
}

impl BufferType {
    pub fn from_raw(raw : u32) -> Self {
        match raw {
            0 => BufferType::Normal,
            1 => BufferType::Type1, 
            3 => BufferType::Type3,
            _ => BufferType::Invalid, 
        }

    }
    pub fn to_raw(self) -> u32 {
        match self {
            BufferType::Normal => 0,
            BufferType::Type1 => 1, 
            BufferType::Invalid => 2, 
            BufferType::Type3 => 3,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum BufferDirection {
    Send, 
    Recieve, 
    Exchange,
}

impl BufferDirection {
    pub fn from_raw(raw : u32) -> Option<Self> {
        match raw {
            0 => Some(BufferDirection::Send), 
            1 => Some(BufferDirection::Recieve), 
            2 => Some(BufferDirection::Exchange), 
            _ => None
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct HandleInfo {
    pub handle : u32, 
    pub direction : HandleDirection,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum HandleDirection {
    Copy, 
    Move, 
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct StaticInfo {
    pub address : *const c_void,
    pub size : usize, 
    pub direction : StaticDirection,
    pub index : u8,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum StaticDirection {
    Send, 
    Recieve
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum IpcCommandType {
    Invalid,
    LegacyRequest,
    Close,
    LegacyControl,
    Request,
    Control,
    RequestWithContext,
    ControlWithContext,
}

impl IpcCommandType {
    pub fn from_raw(raw : u8) -> IpcCommandType {
        match raw {
            1 => IpcCommandType::LegacyRequest,
            2 => IpcCommandType::Close, 
            3 => IpcCommandType::LegacyControl,
            4 => IpcCommandType::Request,
            5 => IpcCommandType::Control,
            6 => IpcCommandType::RequestWithContext,
            7 => IpcCommandType::ControlWithContext, 
            _ => IpcCommandType::Invalid,
        }
    }

    pub fn to_raw(&self) -> u8 {
        match *self {
            IpcCommandType::LegacyRequest => 1,
            IpcCommandType::Close =>  2,
            IpcCommandType::LegacyControl => 3,
            IpcCommandType::Request => 4,
            IpcCommandType::Control => 5,
            IpcCommandType::RequestWithContext => 6,
            IpcCommandType::ControlWithContext =>  7,
            IpcCommandType::Invalid => 0,
        }
    }
}

impl From<u8> for IpcCommandType {
    fn from(raw : u8) -> IpcCommandType {
        IpcCommandType::from_raw(raw)
    }
}

//Thanks roblabla
#[cfg(all(target_os = "horizon", target_arch="aarch64"))]
unsafe fn get_tls_space() -> *mut c_void {
    let addr: *mut c_void;
    asm!("mrs $0, tpidrro_el0" : "=r" (addr));
    if addr.is_null() {
        panic!("TLS Pointer is null");
    }
    addr
}

#[cfg(not(all(target_os = "horizon", target_arch="aarch64")))]
unsafe fn get_tls_space() -> *mut c_void {
    use std::ptr;
    ptr::null_mut()
}

#[test]
fn text_example_ipc() {
    let outbuffer : &[u32] = &[
        0x00_00_00_04, 0x00_00_0c_09, 0,0,
        0x49_43_46_53, 0, 0x08, 0, 
        0x500, 0x0, 0x30, 0x03_00_00_10, 0x00_30_00_00
    ];

    let parsed : IpcCommandMessage<RawIpcArgs>= IpcCommandMessage::parse_from_buffer(&outbuffer);
    let data = parsed.payload();
    println!("{:x?}", data);
    assert_eq!(SFCI_MAGIC, data.raw_words[0]);
}