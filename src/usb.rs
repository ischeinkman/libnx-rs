
use super::LibnxError;
use super::libnx;
use super::libnx as libnx_bindings;

#[derive(Debug, Copy, Clone)]
pub struct UsbEndpointDescriptor { inner : libnx_bindings::usb_endpoint_descriptor}

impl From<libnx_bindings::usb_endpoint_descriptor> for UsbEndpointDescriptor {
    fn from(inner : libnx_bindings::usb_endpoint_descriptor) -> UsbEndpointDescriptor {
        UsbEndpointDescriptor::from_inner(inner)
    }
}

impl UsbEndpointDescriptor {
    pub fn from_inner(inner : libnx_bindings::usb_endpoint_descriptor) -> UsbEndpointDescriptor {
        UsbEndpointDescriptor {
            inner
        }
    }
    
    pub fn inner(&self) -> libnx_bindings::usb_endpoint_descriptor {
        self.inner
    }

    pub fn is_empty(&self) -> bool {
        self.inner.bLength == 0
    }
    pub fn direction(&self) -> EndpointDirection {
        let direction_byte = self.inner.bEndpointAddress;
        EndpointDirection::from_address_byte(direction_byte)
    }
    pub fn attributes_byte(&self) -> u8 {
        self.inner.bmAttributes
    }
    pub fn transfer_type(&self) -> TransferType {
        let attr_byte = self.attributes_byte();
        TransferType::from_address_byte(attr_byte)
    }

    pub fn address(&self) -> u8 {
        self.inner.bEndpointAddress
    }

    pub fn max_packet_size(&self) -> u16 {
        self.inner.wMaxPacketSize
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum EndpointDirection {
    IN, 
    OUT, 
}

impl EndpointDirection {

    const BYTE_MASK : u8 = 0x80;
    
    const OUT_BYTE : u8 = libnx_bindings::usb_endpoint_direction_USB_ENDPOINT_OUT as u8;
    const IN_BYTE : u8 = libnx_bindings::usb_endpoint_direction_USB_ENDPOINT_IN as u8;

    pub fn from_address_byte(byte : u8) -> EndpointDirection {
        if byte & EndpointDirection::IN_BYTE != 0 {
            EndpointDirection::IN
        }
        else {
            EndpointDirection::OUT
        }
    }

    pub fn to_address_byte(&self) -> u8 {
        match *self {
            EndpointDirection::OUT => EndpointDirection::OUT_BYTE,
            EndpointDirection::IN => EndpointDirection::IN_BYTE,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum TransferType {
    CONTROL, 
    ISOCHRONOUS,
    BULK,
    INTERRUPT,
    BULK_STREAM,
    UNKNOWN,
}

impl TransferType {

    const BYTE_MASK : u8 = 0x03;

    const CONTROL_BYTE : u8 = 0;
    const ISO_BYTE : u8 = 1;
    const BULK_BYTE : u8 = 2;
    const INTERRUPT_BYTE : u8 = 3;
    const BULK_STREAM_BYTE : u8 = 4;

    pub fn from_address_byte(byte : u8) -> TransferType {
        match (byte & TransferType::BYTE_MASK) {
            TransferType::CONTROL_BYTE => TransferType::CONTROL,
            TransferType::ISO_BYTE => TransferType::ISOCHRONOUS,
            TransferType::BULK_BYTE => TransferType::BULK,
            TransferType::INTERRUPT_BYTE => TransferType::INTERRUPT,
            TransferType::BULK_STREAM_BYTE => TransferType::BULK_STREAM,
            _ => TransferType::UNKNOWN,
        }
    }
    pub fn to_address_byte(&self) -> u8 {
        match *self {
            TransferType::CONTROL => TransferType::CONTROL_BYTE,
            TransferType::ISOCHRONOUS => TransferType::ISO_BYTE,
            TransferType::BULK => TransferType::BULK_BYTE,
            TransferType::INTERRUPT => TransferType::INTERRUPT_BYTE,
            TransferType::BULK_STREAM => TransferType::BULK_STREAM_BYTE,
            TransferType::UNKNOWN => 0xff,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct UsbInterfaceDescriptor {
    inner : libnx_bindings::usb_interface_descriptor
}

impl UsbInterfaceDescriptor {
    pub fn from_inner(inner : libnx_bindings::usb_interface_descriptor) -> UsbInterfaceDescriptor {
        UsbInterfaceDescriptor {
            inner
        }
    }

    pub fn inner(&self) -> libnx_bindings::usb_interface_descriptor {
        self.inner
    }
    
    pub fn is_empty(&self) -> bool {
        self.inner.bLength == 0
    }

    pub fn class(&self) -> u8 {
        self.inner.bInterfaceClass
    }

    pub fn subclass(&self) -> u8 {
        self.inner.bInterfaceSubClass
    }

    pub fn protocol(&self) -> u8 {
        self.inner.bInterfaceProtocol
    }

    pub fn endpoints_len(&self) -> u8 {
        self.inner.bNumEndpoints
    }
}

#[derive(Clone, Copy, Debug)]
pub struct UsbDeviceDescriptor {
    inner : libnx_bindings::usb_device_descriptor
}

impl UsbDeviceDescriptor {
    pub fn from_inner(inner : libnx_bindings::usb_device_descriptor) -> UsbDeviceDescriptor {
        UsbDeviceDescriptor {
            inner
        }
    }

    pub fn inner(&self) -> libnx_bindings::usb_device_descriptor {
        self.inner
    }
    pub fn is_empty(&self) -> bool {
        self.inner.bLength == 0
    }
    pub fn class(&self) -> u8 {
        self.inner.bDeviceClass
    }

    pub fn subclass(&self) -> u8 {
        self.inner.bDeviceSubClass
    }

    pub fn protocol(&self) -> u8 {
        self.inner.bDeviceProtocol
    }

    pub fn product_id(&self) -> u16 {
        self.inner.idProduct
    }

    pub fn vendor_id(&self) -> u16 {
        self.inner.idVendor
    }

    pub fn configurations_len(&self) -> u8 {
        self.inner.bNumConfigurations
    }

    pub fn max_packet_size(&self) -> u8 {
        self.inner.bMaxPacketSize0
    }
}

#[derive(Clone, Copy, Debug)]
pub struct UsbConfigDescriptor {
    inner : libnx_bindings::usb_config_descriptor,
}

impl UsbConfigDescriptor {
    pub fn from_inner(inner : libnx_bindings::usb_config_descriptor) -> UsbConfigDescriptor {
        UsbConfigDescriptor {
            inner
        }
    }
    
    pub fn inner(&self) -> libnx_bindings::usb_config_descriptor {
        self.inner
    }
    pub fn is_empty(&self) -> bool {
        self.inner.bLength == 0
    }

    pub fn interfaces_len(&self) -> u8 {
        self.inner.bNumInterfaces
    }

    pub fn attributes_byte(&self) -> u8 {
        self.inner.bmAttributes
    }

}