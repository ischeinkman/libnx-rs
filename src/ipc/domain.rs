pub use ipc::payloads::*;
pub use ipc::*;


#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum DomainMessageType {
    Invalid, 
    SendMessage,
    Close, 
}

impl DomainMessageType {
    pub fn from_raw(raw : u8) -> DomainMessageType {
        match raw {
            1 => DomainMessageType::SendMessage,
            2 => DomainMessageType::Close, 
            _ => DomainMessageType::Invalid,
        }
    }

    pub fn to_raw(&self) -> u8 {
        match *self {
            DomainMessageType::SendMessage => 1,
            DomainMessageType::Close => 2, 
            _ => 0,
        }
    }
}


#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct DomainMessageHeader {
    pub kind : u8, 
    pub object_id_count : u8, 
    pub length : u16, 
    pub object_id : u32, 
    pub pad : [u32 ; 2],
}

impl DomainMessageHeader {
    pub fn new(kind : DomainMessageType, object_id_count : u8, length : u16,  object_id : u32) -> DomainMessageHeader {
        DomainMessageHeader {
            kind : kind.to_raw(), 
            object_id_count,
            length,
            object_id,
            pad : [0, 0]
        }
    }
    pub fn empty() -> DomainMessageHeader {
        DomainMessageHeader {
            kind : DomainMessageType::Invalid.to_raw(),
            object_id_count : 0, 
            length : 0, 
            object_id : 0, 
            pad : [0, 0]
        }
    }
}

impl IpcCommandReadable for DomainMessageHeader {
    fn word_count(&self) -> u32 {
        4
    }
    fn read(buff : &[u32]) -> Self {
        let metadata = buff[0];
        let kind = (metadata & 0xFF_00_00_00 >> 24) as u8;
        let object_id_count = (metadata & 0x00_FF_00_00 >> 16) as u8;
        let length = (metadata & 0xFFFF) as u16;
        let object_id = buff[1];
        let pad = [buff[2], buff[3]];
        DomainMessageHeader {
            kind, 
            object_id_count,
            length, 
            object_id, 
            pad,
        }
    }
}

impl IpcCommandWriteable for DomainMessageHeader {
    fn word_count(&self) -> u32 {
        4
    }
    fn write(&self, buff : &mut [u32]) {
        let metadata = ((self.kind as u32) << 24) | ((self.object_id_count as u32) << 16)  | (self.length as u32);
        buff[0] = metadata;
        buff[1] = self.object_id;
        buff[2] = self.pad[0];
        buff[3] = self.pad[1];
    }
}

pub struct DomainMessage<T> {
    header : DomainMessageHeader,
    payload : T,
    object_ids : [u32 ; IPC_MAX_OBJECTS],
}

impl <T> DomainMessage<T> {

    pub fn new(payload : T) -> Self {
        DomainMessage {
            header : DomainMessageHeader::empty(),
            payload,
            object_ids : [0 ; IPC_MAX_OBJECTS]
        }
    }

    pub fn with_header(header : DomainMessageHeader, payload : T) -> Self {
        DomainMessage {
            header, 
            payload, 
            object_ids : [0 ; IPC_MAX_OBJECTS]
        }
    }

    pub fn header_mut(&mut self) -> &mut DomainMessageHeader {
        &mut self.header
    }

    pub fn header(&self) -> DomainMessageHeader {
        self.header
    }

    pub fn payload(&self) -> &T {
        &self.payload
    }

    pub fn add_object_id(&mut self, id : u32) {
        self.object_ids[self.header.object_id_count as usize] = id;
        self.header.object_id_count += 1;
    }

    pub fn object_ids(&self) -> &[u32] {
        &self.object_ids[0 .. self.header.object_id_count as usize]
    }
}

impl <T : IpcCommandReadable> IpcCommandReadable for DomainMessage<T> {
    fn word_count(&self) -> u32 {
        IpcCommandReadable::word_count(&self.header) + IpcCommandReadable::word_count(&self.payload) + self.header.object_id_count as u32
    }
    fn read(buff : &[u32]) -> Self {
        let header = DomainMessageHeader::read(buff);
        let payload = T::read(&buff[4..]);
        let object_id_start = IpcCommandReadable::word_count(&header) + IpcCommandReadable::word_count(&payload);
        let mut object_ids = [0u32 ; IPC_MAX_OBJECTS];
        for idx_offset in 0 .. header.object_id_count as usize {
            object_ids[idx_offset] = buff[object_id_start as usize + idx_offset];
        }
        DomainMessage {
            header, 
            payload,
            object_ids,
        }
    }
}


impl <T : IpcCommandWriteable> IpcCommandWriteable for DomainMessage<T> {
    fn word_count(&self) -> u32 {
         IpcCommandWriteable::word_count(&self.header) + IpcCommandWriteable::word_count(&self.payload) + self.header.object_id_count as u32
    }
    fn write(&self, buff : &mut [u32]) {
        self.header.write(buff);
        self.payload.write(&mut buff[4..]);
        let object_id_start = IpcCommandWriteable::word_count(&self.header) + IpcCommandWriteable::word_count(&self.payload);
        self.object_ids.as_ref().write(&mut buff[object_id_start as usize ..]);
    }
}

pub struct DomainResponseHeader {
    num_object_ids : u32,
}

impl IpcCommandReadable for DomainResponseHeader {
    fn word_count(&self) -> u32 {
        4
    }
    fn read(buff : &[u32]) -> Self {
        DomainResponseHeader {
            num_object_ids : buff[0],
        }
    }
}

impl IpcCommandWriteable for DomainResponseHeader {
    fn word_count(&self) -> u32 {
        4
    }
    fn write(&self, buff : &mut [u32]) {
        buff[0] = self.num_object_ids;
    }
}