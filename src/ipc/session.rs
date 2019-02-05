pub use ipc::payloads::*;
pub use ipc::*;

use std::slice;

#[derive(Copy, Clone, Hash, Debug, Eq, PartialEq)]
pub struct IpcSession {
    pub handle : libnx::Handle,
}


impl IpcSession {
    pub unsafe fn query_pointer_buffer_size(&mut self) -> Result<usize, LibnxError> {
        let mut tls_buff = get_tls_space() as *mut u32;
        let mut buf = slice::from_raw_parts_mut(tls_buff, 8);
        buf[0] = IpcCommandType::Control.to_raw() as u32;
        buf[1] = 8;
        buf[2] = 0;
        buf[3] = 0;
        buf[4] = SFCI_MAGIC;
        buf[5] = 0;
        buf[6] = 3;
        buf[7] = 0;
        let err = libnx::svcSendSyncRequest(self.handle);
        if err != 0 {
            return Err(LibnxError::from_raw(err));
        }
        let out_ptr = get_tls_space() as *mut u32;
        let out_buf = slice::from_raw_parts(out_ptr, 16);
        let parsed = IpcCommandMessage::<QueryPointerBufferResponse>::parse_from_buffer(out_buf);
        Ok(parsed.payload().size as usize)

    }
    
    pub unsafe fn clone(&mut self, unknown : u32) -> Result<Self, LibnxError> {
        let mut tls_buff = get_tls_space() as *mut u32;
        let mut buf = slice::from_raw_parts_mut(tls_buff, 9);
        buf[0] = IpcCommandType::Control.to_raw() as u32;
        buf[1] = 9;
        buf[2] = 0;
        buf[3] = 0;
        buf[4] = SFCI_MAGIC;
        buf[5] = 0;
        buf[6] = 4;
        buf[7] = 0;
        buf[8] = unknown;
        let err = libnx::svcSendSyncRequest(self.handle);
        if err != 0 {
            return Err(LibnxError::from_raw(err));
        }
        let out_ptr = get_tls_space() as *mut u32;
        let out_buf = slice::from_raw_parts(out_ptr, 16);
        let parsed = IpcCommandMessage::<CloneSessionResponse>::parse_from_buffer(out_buf);
        let resp = parsed.payload();
        if resp.err_code != 0 {
            return Err(LibnxError::from_raw((resp.err_code & 0xFFFFFFFF) as u32));
        }
        else if parsed.handles().len() == 0 {
            return Err(LibnxError::from_msg(format!("Error while cloning {}: got no handles!", unknown)));
        }
        else {
            return Ok(parsed.handles()[0].into());
        }
    }
    
    pub unsafe fn close(mut self) -> Result<(), LibnxError> {
        let mut tls_buff = get_tls_space() as *mut u32;
        let mut buf = slice::from_raw_parts_mut(tls_buff, 2);
        buf[0] = IpcCommandType::Close.to_raw() as u32;
        buf[1] = 0;
        let err = libnx::svcSendSyncRequest(self.handle);
        match err {
            0 => Ok(()),
            e => Err(LibnxError::from_raw(e)),
        }
    }

    pub unsafe fn convert_to_domain(mut self) -> Result<u32, LibnxError> {
        let mut tls_buff = get_tls_space() as *mut u32;
        let mut buf = slice::from_raw_parts_mut(tls_buff, 8);
        buf[0] = IpcCommandType::Control.to_raw() as u32;
        buf[1] = 8;
        buf[4] = SFCI_MAGIC;
        buf[5] = 0;
        buf[6] = 0;
        buf[7] = 0;
        
        let err = libnx::svcSendSyncRequest(self.handle);
        if err != 0 {
            return Err(LibnxError::from_raw(err));
        }
        let out_ptr = get_tls_space() as *mut u32;
        let out_buf = slice::from_raw_parts(out_ptr, 16);
        let parsed = IpcCommandMessage::<ConvertSessionToDomainResponse>::parse_from_buffer(out_buf);
        let resp = parsed.payload();
        match resp.err_code {
            0 => Ok(resp.object_id),
            e => Err(LibnxError::from_raw((e & (u32::max_value() as u64)) as u32))
        }
    }

    pub unsafe fn dispatch_command<T : IpcCommandWriteable>(&mut self, command : IpcCommandHeader<T>) -> Result<(), LibnxError> {
        let mut tls_ptr = get_tls_space() as *mut u32;
        let tls_slice = slice::from_raw_parts_mut(tls_ptr, 64);
        command.write_header(tls_slice);
        let err = libnx::svcSendSyncRequest(self.handle);
        if err != 0 {
            Err(LibnxError::from_raw(err))
        }
        else {
            Ok(())
        }
    }

    pub unsafe fn close_object_by_id(&mut self, object_id : u32) -> Result<(), LibnxError> {
        let message_header = DomainMessageHeader::new(DomainMessageType::Close, 0, 0, object_id);
        let message : DomainMessage<EmptyArgs> = DomainMessage::with_header(message_header, EmptyArgs::new());
        let ipc_command =IpcCommandHeader::with_args(message);
        self.dispatch_command(ipc_command)
    }
} 

impl From<libnx::Handle> for IpcSession {
    fn from(inner : libnx::Handle) -> IpcSession {
        IpcSession {
            handle : inner
        }
    }
}

impl From<IpcSession> for libnx::Handle {
    fn from(wrapper : IpcSession) -> libnx::Handle {
        wrapper.handle
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct QueryPointerBufferResponse {
    magic : u64, 
    err_code : u64, 
    size : u32, 
}

impl IpcCommandReadable for QueryPointerBufferResponse {
    fn read(buff : &[u32]) -> Self {
        let magic = ((buff[1] as u64) << 24) | (buff[0] as u64);
        let err_code = ((buff[3] as u64) << 24) | (buff[2] as u64);
        let size = buff[4];
        QueryPointerBufferResponse {
            magic, 
            err_code,
            size, 
        }

    }
    fn word_count(&self) -> u32 {
        5
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct CloneSessionResponse {
    magic : u64, 
    err_code : u64, 

}
impl IpcCommandReadable for CloneSessionResponse {
    fn read(buff : &[u32]) -> Self {
        let magic = ((buff[1] as u64) << 24) | (buff[0] as u64);
        let err_code = ((buff[3] as u64) << 24) | (buff[2] as u64);
        CloneSessionResponse {
            magic, 
            err_code,
        }

    }
    fn word_count(&self) -> u32 {
        4
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct ConvertSessionToDomainResponse {
    magic : u64, 
    err_code : u64, 
    object_id : u32, 
}

impl IpcCommandReadable for ConvertSessionToDomainResponse {
    fn read(buff : &[u32]) -> Self {
        let magic = ((buff[1] as u64) << 24) | (buff[0] as u64);
        let err_code = ((buff[3] as u64) << 24) | (buff[2] as u64);
        let object_id = buff[4];
        ConvertSessionToDomainResponse {
            magic, 
            err_code,
            object_id,
        }

    }
    fn word_count(&self) -> u32 {
        5
    }
}