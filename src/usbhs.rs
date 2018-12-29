use super::LibnxError;
use super::libnx;
use super::libnx as libnx_bindings;
use super::usb::{
    TransferType, EndpointDirection, 
    UsbEndpointDescriptor, UsbInterfaceDescriptor,
    UsbConfigDescriptor, UsbDeviceDescriptor,
};
pub struct UsbHsContext {}

const IFACE_SIZE : usize = std::mem::size_of::<libnx_bindings::UsbHsInterface>();
impl UsbHsContext {
    pub fn initialize() -> Result<Self, LibnxError> {
        let retval = unsafe { libnx_bindings::usbHsInitialize() };
        match retval {
            0 => Ok(UsbHsContext {}),
            er => Err(LibnxError::from_raw(er))
        }
    }

    pub fn query_available_interfaces(&self, filter : InterfaceFilter, max_interfaces : usize) -> Result<Vec<Interface>, LibnxError> {
        let mut raw_buff : Vec<libnx_bindings::UsbHsInterface> = Vec::with_capacity(max_interfaces + 2);
        let buff_ptr = raw_buff.as_mut_slice().as_mut_ptr();
        let mut retsize = 0;
        let mut retsize_ptr = &mut retsize as *mut _;
        let filter_ptr = &filter.inner as *const libnx_bindings::UsbHsInterfaceFilter;

        let err = unsafe{ libnx_bindings::usbHsQueryAvailableInterfaces(filter_ptr, buff_ptr, max_interfaces * IFACE_SIZE, retsize_ptr)};
        if err != 0 {
            return Err(LibnxError::from_raw(err));
        }
        unsafe {raw_buff.set_len(std::ptr::read(retsize_ptr) as usize)};
        let retval = raw_buff.into_iter().map(Interface::from_inner).collect();
        Ok(retval)
    }
    
    pub fn query_all_interfaces(&self, filter : InterfaceFilter, max_interfaces : usize) -> Result<Vec<Interface>, LibnxError> {
        let mut raw_buff : Vec<libnx_bindings::UsbHsInterface> = Vec::with_capacity(max_interfaces + 2);
        let buff_ptr = raw_buff.as_mut_slice().as_mut_ptr();
        let mut retsize = 0;
        let mut retsize_ptr = &mut retsize as *mut _ as *mut _;
        let filter_ptr = &filter.inner as *const libnx_bindings::UsbHsInterfaceFilter;

        let err = unsafe{ libnx_bindings::usbHsQueryAllInterfaces(filter_ptr, buff_ptr, max_interfaces * IFACE_SIZE, retsize_ptr)};
        if err != 0 {
            return Err(LibnxError::from_raw(err));
        }
        unsafe {raw_buff.set_len(retsize)};
        let retval = raw_buff.into_iter().map(Interface::from_inner).collect();
        Ok(retval)
    }
    
    pub fn query_acquired_interfaces(&self, max_interfaces : usize) -> Result<Vec<Interface>, LibnxError> {
        let mut raw_buff : Vec<libnx_bindings::UsbHsInterface> = Vec::with_capacity(max_interfaces + 2);
        let buff_ptr = raw_buff.as_mut_slice().as_mut_ptr();
        let mut retsize = 0;
        let mut retsize_ptr = &mut retsize as *mut _ as *mut _;

        let err = unsafe{ libnx_bindings::usbHsQueryAcquiredInterfaces(buff_ptr, max_interfaces * IFACE_SIZE, retsize_ptr)};
        if err != 0 {
            return Err(LibnxError::from_raw(err));
        }
        unsafe {raw_buff.set_len(retsize)};
        let retval = raw_buff.into_iter().map(Interface::from_inner).collect();
        Ok(retval)
    }

    pub fn acquire_interface(&mut self, interface : &Interface) -> Result<ClientInterfaceSession, LibnxError> {
        use std::fs::{File, OpenOptions};
        use std::io::{Write};
        let mut log_hs = OpenOptions::new().append(true).create(true).open("libnx_usbhs_log.txt").map_err(|e| LibnxError::from_msg(format!("std::io::err: {:?}", e)))?;
        log_hs.write_fmt(format_args!("Starting acqiface using {:?}\n", interface));
        log_hs.flush();
        let mut retval_inner : libnx_bindings::UsbHsClientIfSession = unsafe {std::mem::zeroed()};
        let retval_inner_ptr = &mut retval_inner as *mut libnx_bindings::UsbHsClientIfSession;
        let iface_inner_ptr = &interface.inner as *const libnx_bindings::UsbHsInterface as *mut libnx_bindings::UsbHsInterface;
        let err = unsafe { libnx_bindings::usbHsAcquireUsbIf(retval_inner_ptr, iface_inner_ptr) };
        log_hs.write_fmt(format_args!("Got retval {}\n", err));
        log_hs.write_fmt(format_args!("Retval: {{ ID: {:?}, inf: {:?} }}\n", retval_inner.ID, Interface::from_inner(retval_inner.inf)));
        log_hs.flush();
        match err {
            0 => Ok(ClientInterfaceSession::from_inner(retval_inner)),
            e => Err(LibnxError::from_raw(e))
        }
    }
}

impl Drop for UsbHsContext {
    fn drop(&mut self) {
        unsafe { libnx_bindings::usbHsExit() };
    }
}


pub struct ClientInterfaceSession {
    inner : libnx_bindings::UsbHsClientIfSession,
}

impl ClientInterfaceSession {
    pub fn from_inner(inner : libnx_bindings::UsbHsClientIfSession) -> ClientInterfaceSession {
        ClientInterfaceSession {
            inner, 
        }
    }

    pub fn id(&self) -> i32 {
        self.inner.ID
    }
    pub fn ctrl_transfer(&mut self, request_type : u8, request : u8, wvalue : u16, windex : u16, output : &mut [u8]) -> Result<usize, LibnxError> {
        let mut transferred_size : u32 = 0;
        let s = &mut self.inner as *mut libnx_bindings::UsbHsClientIfSession;
        let err = unsafe {libnx_bindings::usbHsIfCtrlXfer(
            s,
            request_type,
            request,
            wvalue,
            windex,
            output.len() as u16,
            output.as_mut_ptr() as *mut _,
            &mut transferred_size as *mut _,
        )};
        match err {
            0 => Ok(transferred_size as usize),
            e => Err(LibnxError::from_raw(e))
        }
    }
    pub fn open_endpoint_raw(&mut self, max_urb_count : u16, max_transfer_size : u32, endpoint_desc : &UsbEndpointDescriptor) -> Result<ClientEndpointSession, LibnxError> {
        use std::fs::{File, OpenOptions};
        use std::io::{Write};
        let mut log_hs = OpenOptions::new().append(true).create(true).open("libnx_usbhs_log.txt").map_err(|e| LibnxError::from_msg(format!("std::io::err: {:?}", e)))?;
        let raw_epdesc = endpoint_desc.inner();
        log_hs.write_fmt(format_args!("Starting acqep using {:?}\n", endpoint_desc));
        log_hs.write_fmt(format_args!("Params: {{ max_urb_count: {}, max_transfer_size: {} }}\n", max_urb_count, max_transfer_size));
        log_hs.flush();
        let ep_desc_ptr = &raw_epdesc as *const _ as *mut _;
        let iface_sess_ptr = &mut self.inner as *mut _;
        let mut retval_inner : libnx_bindings::UsbHsClientEpSession = unsafe {std::mem::zeroed()};
        let retval_inner_ptr = &mut retval_inner as *mut _;

        let err = unsafe {libnx_bindings::usbHsIfOpenUsbEp(iface_sess_ptr, retval_inner_ptr, max_urb_count, max_transfer_size, ep_desc_ptr)};
        log_hs.write_fmt(format_args!("Got retval of {}\n", err));
        log_hs.write_fmt(format_args!("Output ptr: {:?}\n", retval_inner));
        log_hs.flush();
        if err != 0 {
            Err(LibnxError::from_raw(err))
        }
        else if retval_inner.desc != endpoint_desc.inner() {
            Err(LibnxError::from_msg(format!("Error: output descriptor != input.\nInput:\n{:?}\nOutput:\n{:?}", endpoint_desc.inner(), retval_inner.desc)))
        }
        else {
            Ok(ClientEndpointSession::from_inner(retval_inner))
        }
    }

    pub fn open_endpoint(&mut self, interface : &UsbEndpointDescriptor) -> Result<ClientEndpointSession, LibnxError> {
        let max_urb_count = 1;
        let max_transfer_size = interface.max_packet_size() as u32;
        self.open_endpoint_raw(max_urb_count, max_transfer_size, interface)
    }

    pub fn reset(&mut self) -> Result<(), LibnxError> {
        let err = unsafe { libnx_bindings::usbHsIfResetDevice(&mut self.inner as *mut libnx_bindings::UsbHsClientIfSession) };
        match err {
            0 => Ok(()),
            e => Err(LibnxError::from_raw(e)),
        }
    }

}

impl Drop for ClientInterfaceSession {
    fn drop(&mut self) {
        unsafe {
            libnx_bindings::usbHsIfClose(&mut self.inner as *mut libnx_bindings::UsbHsClientIfSession);
        }
    }
}

pub struct ClientEndpointSession {
    inner : libnx_bindings::UsbHsClientEpSession,
}

impl ClientEndpointSession {
    pub fn from_inner(inner : libnx_bindings::UsbHsClientEpSession) -> ClientEndpointSession {
        ClientEndpointSession {
            inner, 
        }
    }
    pub fn read(&mut self, output : &mut [u8]) -> Result<usize, LibnxError> {
        use std::fs::{File, OpenOptions};
        use std::io::{Write};
        let mut log_hs = OpenOptions::new().append(true).create(true).open("libnx_usbhs_log.epread.txt").map_err(|e| LibnxError::from_msg(format!("std::io::err: {:?}", e)))?;
        log_hs.write_fmt(format_args!("Starting read using {:?}\n", self.inner));
        log_hs.flush();
        if output.is_empty() {
            return Err(LibnxError::from_msg(format!("Can't read to empty output!")));
        }
        else if self.endpoint_desc().direction() != EndpointDirection::IN {
            return Err(LibnxError::from_msg(format!("Can't read from write endpoint!")));
        }

        let s = &mut self.inner as *mut libnx_bindings::UsbHsClientEpSession;
        let buffer = output as *mut [u8] as *mut libnx_bindings::lang_items::c_void;
        let size = output.len() as u32;
        let mut transfered_size_loc : u32 = 0;
        let transfered_size = &mut transfered_size_loc as *mut u32;
        log_hs.write_fmt(format_args!("Raw params: {{ buffer : {:?}, buffer_size : {} }}\n", buffer as usize, size));
        let err = unsafe {libnx_bindings::usbHsEpPostBuffer(s, buffer, size, transfered_size)};
        log_hs.write_fmt(format_args!("Got retval of {}\n", transfered_size_loc));
        log_hs.write_fmt(format_args!("Output: {:?}\n", output));
        log_hs.flush();
        match err {
            0 => Ok(transfered_size_loc as usize),
            e => Err(LibnxError::from_raw(e)),
        }
    }
    pub fn write(&mut self, input : &[u8]) -> Result<usize, LibnxError> {
        use std::fs::{File, OpenOptions};
        use std::io::{Write};
        let mut log_hs = OpenOptions::new().append(true).create(true).open("libnx_usbhs_log.epwrite.txt").map_err(|e| LibnxError::from_msg(format!("std::io::err: {:?}", e)))?;
        log_hs.write_fmt(format_args!("Starting write using {:?}\n", self.inner));
        log_hs.write_fmt(format_args!("Output {:?}\n", input));
        log_hs.flush();
        if input.is_empty() {
            return Err(LibnxError::from_msg(format!("Can't write from empty output!")));
        }
        else if self.endpoint_desc().direction() != EndpointDirection::OUT {
            return Err(LibnxError::from_msg(format!("Can't write to read endpoint!")));
        }

        let s = &mut self.inner as *mut libnx_bindings::UsbHsClientEpSession;
        let buffer = input as *const [u8] as *mut libnx_bindings::lang_items::c_void;
        let size = input.len() as u32;
        let mut transfered_size_loc : u32 = 0;
        let transfered_size = &mut transfered_size_loc as *mut u32;
        log_hs.write_fmt(format_args!("Raw params: {{ buffer : {:?}, buffer_size : {} }}\n", buffer as usize, size));
        log_hs.flush();
        let err = unsafe {libnx_bindings::usbHsEpPostBuffer(s, buffer, size, transfered_size)};
        log_hs.write_fmt(format_args!("Got retval of {}\n", err));
        log_hs.write_fmt(format_args!("Output: {:?}\n", transfered_size_loc));
        log_hs.flush();
        match err {
            0 => Ok(transfered_size_loc as usize),
            e => Err(LibnxError::from_raw(e)),
        }
    }

    pub fn endpoint_desc(&self) -> UsbEndpointDescriptor {
        UsbEndpointDescriptor::from_inner(self.inner.desc)
    }
}

impl Drop for ClientEndpointSession {
    fn drop(&mut self) {
        unsafe {
            libnx_bindings::usbHsEpClose(&mut self.inner as *mut _);
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct InterfaceFilter {
    inner : libnx_bindings::UsbHsInterfaceFilter
}

impl Default for InterfaceFilter {
    fn default() -> InterfaceFilter {
        InterfaceFilter::new()
    }
}

impl InterfaceFilter {
    pub fn new() -> InterfaceFilter {
        let inner = unsafe {std::mem::zeroed()};
        InterfaceFilter {
            inner
        }
    }

    pub fn with_vendor(mut self, vendor_id : u16) -> InterfaceFilter {
        let flag = libnx_bindings::UsbHsInterfaceFilterFlags_UsbHsInterfaceFilterFlags_idVendor;
        self.inner.Flags |= flag;
        self.inner.idVendor = vendor_id;
        self
    }
    pub fn with_product(mut self, product_id : u16) -> InterfaceFilter {
        let flag = libnx_bindings::UsbHsInterfaceFilterFlags_UsbHsInterfaceFilterFlags_idProduct;
        self.inner.Flags |= flag;
        self.inner.idProduct = product_id;
        self
    }

    pub fn with_device_class(mut self, class : u8) -> InterfaceFilter {
        let flag = libnx_bindings::UsbHsInterfaceFilterFlags_UsbHsInterfaceFilterFlags_bDeviceClass;
        self.inner.Flags |= flag;
        self.inner.bDeviceClass = class;
        self
    }
    pub fn with_device_subclass(mut self, subclass : u8) -> InterfaceFilter {
        let flag = libnx_bindings::UsbHsInterfaceFilterFlags_UsbHsInterfaceFilterFlags_bDeviceSubClass;
        self.inner.Flags |= flag;
        self.inner.bDeviceSubClass = subclass;
        self
    }
    pub fn with_device_protocol(mut self, protocol : u8) -> InterfaceFilter {
        let flag = libnx_bindings::UsbHsInterfaceFilterFlags_UsbHsInterfaceFilterFlags_bDeviceProtocol;
        self.inner.Flags |= flag;
        self.inner.bDeviceProtocol = protocol;
        self
    }
    pub fn with_interface_class(mut self, class : u8) -> InterfaceFilter {
        let flag = libnx_bindings::UsbHsInterfaceFilterFlags_UsbHsInterfaceFilterFlags_bInterfaceClass;
        self.inner.Flags |= flag;
        self.inner.bInterfaceClass = class;
        self
    }
    pub fn with_interface_subclass(mut self, subclass : u8) -> InterfaceFilter {
        let flag = libnx_bindings::UsbHsInterfaceFilterFlags_UsbHsInterfaceFilterFlags_bInterfaceSubClass;
        self.inner.Flags |= flag;
        self.inner.bInterfaceSubClass = subclass;
        self
    }
    pub fn with_interface_protocol(mut self, protocol : u8) -> InterfaceFilter {
        let flag = libnx_bindings::UsbHsInterfaceFilterFlags_UsbHsInterfaceFilterFlags_bInterfaceProtocol;
        self.inner.Flags |= flag;
        self.inner.bInterfaceProtocol = protocol;
        self
    }
}

#[derive(Copy, Clone)]
pub struct Interface {
    inner : libnx_bindings::UsbHsInterface,
}

impl Interface {
    pub fn from_inner(inner : libnx_bindings::UsbHsInterface) -> Interface {
        Interface {
            inner
        }
    }

    pub fn into_inner(self) -> libnx_bindings::UsbHsInterface {
        self.inner
    }

    pub fn config_desc(&self) -> UsbConfigDescriptor {
        UsbConfigDescriptor::from_inner(self.inner.config_desc)
    }

    pub fn device_desc(&self) -> UsbDeviceDescriptor {
        UsbDeviceDescriptor::from_inner(self.inner.device_desc)
    }

    pub fn info(&self) -> InterfaceInfo {
        InterfaceInfo::from_inner(self.inner.inf)
    }

}

use std::fmt;
impl fmt::Debug for Interface {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        let pathstr_cstr = unsafe { std::ffi::CStr::from_ptr(&self.inner.pathstr as *const _) };
        let pathstr = pathstr_cstr.to_string_lossy();
        write!(f, "Interface:{{ inf: {:?}, pathstr: {:?}, busid: {:?}, deviceid: {:?}, device_desc: {:?}, config_desc: {:?}, timestamp: {:?} }}", 
        self.inner.inf, pathstr, self.inner.busID, self.inner.deviceID, self.inner.device_desc, self.inner.config_desc, self.inner.timestamp)
    }
}

impl PartialEq for Interface {
    fn eq(&self, other : &Interface) -> bool {
        let exp_ptr = &self.inner as *const _ as *const u64;
        let act_ptr = &other.inner as *const _ as *const u64;
        for idx in 0 .. (IFACE_SIZE/8) {
            let exp_bt = unsafe {
                std::ptr::read(exp_ptr.offset(idx as isize))
            };
            let act_bt = unsafe {
                std::ptr::read(act_ptr.offset(idx as isize))
            };
            if exp_bt != act_bt {
                return false;
            }
        }
        true
    }
}

impl Eq for Interface {}

pub struct InterfaceAvailableEvent {
    event : libnx_bindings::Event, 
    index : u8, 
}

impl InterfaceAvailableEvent {
    pub fn create(autoclear : bool, index : u8, filter : InterfaceFilter) -> Result<InterfaceAvailableEvent, LibnxError> {
        if index > 2 {
            return Err(LibnxError::from_msg(format!("Index needs to be 0..2; passed {}.", index)));
        }
        let mut inner = unsafe {std::mem::zeroed()};
        let err = unsafe{ libnx_bindings::usbHsCreateInterfaceAvailableEvent(&mut inner as *mut _, autoclear, index, &filter.inner as *const _ as *mut _) };
        match err {
            0 => Ok(InterfaceAvailableEvent{ event : inner, index }),
            e => Err(LibnxError::from_raw(e))
        }
    }

    fn destroy_internal(&mut self) -> Result<(), LibnxError> {
        let err = unsafe { libnx_bindings::usbHsDestroyInterfaceAvailableEvent(&mut self.event as *mut _, self.index) };
        match err {
            0 => Ok(()),
            e => Err(LibnxError::from_raw(e)),
        }
    }

    pub fn wait(&self, nanos : u64) -> Result<(), LibnxError> {
        let err = unsafe{libnx_bindings::eventWait(&self.event as *const _ as *mut _, nanos)};
        match err {
            0 => Ok(()),
            e => Err(LibnxError::from_raw(e))
        }
    }
}

impl Drop for InterfaceAvailableEvent {
    fn drop(&mut self) {
        self.destroy_internal();
    }
}

pub struct InterfaceInfo {
    inner : libnx_bindings::UsbHsInterfaceInfo,
}

impl InterfaceInfo {
    pub fn from_inner(inner : libnx_bindings::UsbHsInterfaceInfo) -> InterfaceInfo {
        InterfaceInfo {
            inner
        }
    }

    pub fn id(&self) -> i32 {
        self.inner.ID
    }

    pub fn device_id(&self) -> u32 {
        self.inner.deviceID_2
    }

    pub fn interface_desc(&self) -> UsbInterfaceDescriptor {
        UsbInterfaceDescriptor::from_inner(self.inner.interface_desc)
    }

    fn output_endpoint_descs_raw(&self) -> &[libnx_bindings::usb_endpoint_descriptor] {
        &self.inner.output_endpoint_descs
    }

    pub fn output_endpoint_descs<'a>(&'a self) -> impl Iterator<Item=UsbEndpointDescriptor> + 'a {
        self.output_endpoint_descs_raw().iter().map(|inner| UsbEndpointDescriptor::from_inner(*inner)).filter(|desc| !desc.is_empty())
    }

    fn input_endpoint_descs_raw(&self) -> &[libnx_bindings::usb_endpoint_descriptor] {
        &self.inner.input_endpoint_descs
    }
    
    pub fn input_endpoint_descs<'a>(&'a self) -> impl Iterator<Item=UsbEndpointDescriptor> + 'a {
        self.input_endpoint_descs_raw().iter().map(|inner| UsbEndpointDescriptor::from_inner(*inner)).filter(|desc| !desc.is_empty())
    }

    pub fn endpoint_descriptors<'a>(&'a self) -> impl Iterator<Item=UsbEndpointDescriptor> + 'a {
        let inputs = self.input_endpoint_descs();
        let outputs = self.output_endpoint_descs();
        inputs.chain(outputs)
    }
}