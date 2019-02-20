
use crate::ipc::*;
use crate::libnx;
use crate::LibnxError;

use std::ffi::CString;


pub struct SmContext {

}

impl SmContext {
    pub fn initialize() -> Result<SmContext, LibnxError> {
        let err = unsafe {libnx::smInitialize()};
        match err {
            0 | 0x80C16 => Ok(SmContext{}),
            e => Err(LibnxError::from_raw(e)),
        }
    }

    pub fn get_service(&mut self, name : &str) -> Result<Service, LibnxError> {
        let mut service_out = libnx::Service {
            handle : u32::max_value(), 
            object_id : u32::max_value(),
            type_ : u32::max_value(),
        };
        let c_str = CString::new(name).map_err(|e| LibnxError::from_msg(format!("CString create err: {:?}", e)))?;
        let err = unsafe { libnx::smGetService(&mut service_out as *mut _, c_str.as_ptr() as *const u8) };
        match err {
            0 => Ok(service_out.into()),
            e => Err(LibnxError::from_raw(e))
        }
    }

    pub fn register_service(&mut self, name : &str, is_light : bool, max_sessions : u32) -> Result<IpcSession, LibnxError> {
        let c_str = CString::new(name).map_err(|e| LibnxError::from_msg(format!("CString create err: {:?}", e)))?;
        let mut out_handle = 0;
        let err = unsafe { libnx::smRegisterService(&mut out_handle as *mut _, c_str.as_ptr() as *const u8, is_light, max_sessions as i32)};
        match err {
            0 => Ok(out_handle.into()),
            e => Err(LibnxError::from_raw(e))
        }
    }

    pub fn unregister_service(&mut self, name : &str) -> Result<(), LibnxError> {
        let c_str = CString::new(name).map_err(|e| LibnxError::from_msg(format!("CString create err: {:?}", e)))?;
        let err = unsafe { libnx::smUnregisterService(c_str.as_ptr() as *const u8)};
        match err {
            0 => Ok(()),
            e => Err(LibnxError::from_raw(e))
        }
    }

    pub fn add_override_handle(&mut self, name : &str, handle : IpcSession) {
        unsafe {
            libnx::smAddOverrideHandle(Self::encode_name(name), handle.handle);
        }
    }

    pub fn get_service_original(&mut self, name : &str) -> Result<IpcSession, LibnxError> {
        let mut retval_inner : libnx::Handle = u32::max_value();
        let encoded_name = Self::encode_name(name);
        let err = unsafe {libnx::smGetServiceOriginal(&mut retval_inner as *mut _, encoded_name)};
        match err {
            0 => Ok(IpcSession::from(retval_inner)),
            e => Err(LibnxError::from_raw(e)),
        }
    }

    pub fn encode_name(name : &str) -> u64 {
        let name_bytes = name.as_bytes();
        let mut retval = 0;
        for idx in 0.. name_bytes.len() {
            let cur_byte = name_bytes[idx] as u64;
            let shifted_byte = cur_byte << (8 * idx);
            retval += shifted_byte;
        }
        retval
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum ServiceType {
    Uninitialized, 
    Normal, 
    Domain, 
    DomainSubservice, 
    Override,
}

impl From<libnx::ServiceType> for ServiceType {
    fn from(inner : libnx::ServiceType) -> ServiceType {
        match inner {
            libnx::ServiceType_ServiceType_Uninitialized => ServiceType::Uninitialized,
            libnx::ServiceType_ServiceType_Normal => ServiceType::Normal,
            libnx::ServiceType_ServiceType_Domain => ServiceType::Domain,
            libnx::ServiceType_ServiceType_DomainSubservice => ServiceType::DomainSubservice,
            libnx::ServiceType_ServiceType_Override => ServiceType::Override,
            _ => ServiceType::Uninitialized,
        }
    }
}

pub const INVALID_OBJECT_ID : u32 = u32::max_value();

pub struct Service {
    handle : IpcSession,
    object_id : u32, 
    kind : ServiceType
}

impl From<libnx::Service> for Service {
    fn from(inner : libnx::Service) -> Service {
        Service {
            handle : inner.handle.into(),
            kind : inner.type_.into(),
            object_id : inner.object_id,
        }
    }
}

impl Service {

    pub fn create(session : IpcSession) -> Service {
        Service {
            handle : session,
            object_id : INVALID_OBJECT_ID,
            kind : ServiceType::Normal,
        }
    }

    pub fn is_override(&self) -> bool {
        self.kind == ServiceType::Override
    }

    pub fn is_active(&self) -> bool {
        self.kind != ServiceType::Override
    }

    pub fn is_domain(&self) -> bool {
        self.kind == ServiceType::Domain
    }
    
    pub fn is_domain_subservice(&self) -> bool {
        self.kind == ServiceType::DomainSubservice
    }

    pub fn handle(&self) -> IpcSession {
        self.handle
    }

    pub fn object_id(&self) -> u32 {
        self.object_id
    }

    pub fn kind(&self) -> ServiceType {
        self.kind
    }

    pub fn close(&mut self) -> Result<(), LibnxError> {
        match self.kind {
            ServiceType::Normal | ServiceType::Domain => {
                unsafe {
                    self.handle.close()?;
                    let err = libnx::svcCloseHandle(self.handle.handle);
                    if err != 0 {
                        return Err(LibnxError::from_raw(err));
                    }
                };
            },
            ServiceType::DomainSubservice => {
                unsafe {
                    self.handle.close_object_by_id(self.object_id)?;
                }
            },
            _ => {}
        };
        self.kind = ServiceType::Uninitialized;
        Ok(())
    }
}

use std::time::Duration;
impl Waitable for Service {
    type Trigger = IpcSession; 
    fn wait_synchronization(&self, timeout : Duration) -> Result<IpcSession, LibnxError> {
        self.handle.wait_synchronization(timeout)
    }
}