use std::time::Duration;

use libusb::{ConfigDescriptor, Context, Device, DeviceDescriptor, DeviceHandle, Language, Speed};
use log::{debug, trace};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HantekUsbError {
    #[error("failed to read from usb")]
    ReadError { error: libusb::Error },

    #[error("failed to write to usb")]
    WriteError { error: libusb::Error },

    #[error("error releasing usb interfaces")]
    UsbInterfaceReleaseError { error: libusb::Error },

    #[error("error claiming any of usb interfaces")]
    UsbInterfaceClaimError { errors: Vec<(u8, libusb::Error)> },

    #[error("error reading usb manufacturer string")]
    ManufacturerReadUsbError { error: libusb::Error },

    #[error("error reading usb product string")]
    ProductReadUsbError { error: libusb::Error },

    #[error("error reading usb languages")]
    ReadLanguagesError { error: libusb::Error },

    #[error("failed to get usb devices")]
    GetUsbDevicesError { error: libusb::Error },

    #[error("failed to open usb devices")]
    OpenUsbDeviceError { error: libusb::Error },

    #[error("failed to get usb device config")]
    GetConfigError { error: libusb::Error },

    #[error("no usb language available, can not read product string")]
    ProductReadNoLanguageAvailable,

    #[error("no usb language available, can not read manufacturer string")]
    ManufacturerReadNoLanguageAvailable,

    #[error("no usb device found with required vid={vid}, pid={pid}")]
    NoDeviceFound { vid: u16, pid: u16 },

    #[error("too many devices found, pid={pid}, vid={vid}, number_of_devices={instances}")]
    TooManyDevicesFound {
        pid: u16,
        vid: u16,
        instances: usize,
    },

    #[error("an interface is already claimed, interface_no={interface}")]
    InterfaceAlreadyClaimed { interface: u8 },

    #[error("no interface is claimed yet for the requested operation")]
    NoInterfaceClaimed,
}

impl HantekUsbError {
    // Because CLion doesn't like the Display implemented by thiserror.
    pub fn my_to_string(&self) -> impl std::fmt::Display + '_ {
        self
    }
}

pub struct HantekUsbDevice<'a> {
    timeout: Duration,
    claimed_interface: Option<u8>,
    pub device: Device<'a>,
    pub descriptor: DeviceDescriptor,
    pub handle: DeviceHandle<'a>,
    pub language: Option<Language>,
    pub config: ConfigDescriptor,
}

impl<'a> HantekUsbDevice<'a> {
    pub fn open(
        context: &'a Context,
        timeout: Duration,
        (vid, pid): (u16, u16),
    ) -> Result<Self, HantekUsbError> {
        let (device, descriptor) = Self::find_single_device(context, (vid, pid))?;

        let handle = device
            .open()
            .map_err(|error| HantekUsbError::OpenUsbDeviceError { error })?;

        let language = Self::get_device_language(&handle, timeout)?;

        let config = device
            .config_descriptor(0)
            .map_err(|error| HantekUsbError::GetConfigError { error })?;

        Ok(Self {
            timeout,
            claimed_interface: None,
            device,
            descriptor,
            handle,
            language,
            config,
        })
    }

    // =========================================================================

    fn find_devices(
        context: &Context,
        (vid, pid): (u16, u16),
    ) -> Result<Vec<(Device, DeviceDescriptor)>, HantekUsbError> {
        Ok(context
            .devices()
            .map_err(|error| HantekUsbError::GetUsbDevicesError { error })?
            .iter()
            .map(|dev| (dev.device_descriptor(), dev))
            .map(|(device_descriptor, device)| (device, device_descriptor))
            .filter_map(|it| {
                if it.1.is_err() {
                    debug!(
                        "could not open device descriptor, bus={} address={}",
                        it.0.bus_number(),
                        it.0.address()
                    );
                    None
                } else {
                    Some((it.0, it.1.unwrap()))
                }
            })
            .filter(|it| {
                if it.1.vendor_id() != vid || it.1.product_id() != pid {
                    trace!(
                        "skipping device on mismatch, pid={} vid={}",
                        it.1.product_id(),
                        it.1.vendor_id()
                    );
                    false
                } else {
                    true
                }
            })
            .collect())
    }

    fn find_single_device(
        context: &Context,
        (vid, pid): (u16, u16),
    ) -> Result<(Device, DeviceDescriptor), HantekUsbError> {
        let mut devices = Self::find_devices(context, (vid, pid))?;

        match devices.len() {
            0 => Err(HantekUsbError::NoDeviceFound { vid, pid }),
            1 => Ok(devices.pop().unwrap()),
            _ => Err(HantekUsbError::TooManyDevicesFound {
                vid,
                pid,
                instances: devices.len(),
            }),
        }
    }

    fn get_device_language(
        handle: &DeviceHandle,
        timeout: Duration,
    ) -> Result<Option<Language>, HantekUsbError> {
        handle
            .read_languages(timeout)
            .map(|mut languages| {
                if languages.len() > 1 {
                    trace!(
                        "multiple languages available, choosing first. Number of languages={}",
                        languages.len()
                    )
                }
                languages.pop()
            })
            .map_err(|error| HantekUsbError::ReadLanguagesError { error })
    }

    // =========================================================================

    pub fn get_manufacturer(&self) -> Result<String, HantekUsbError> {
        if self.language.is_none() {
            return Err(HantekUsbError::ManufacturerReadNoLanguageAvailable);
        }

        self.handle
            .read_manufacturer_string(self.language.unwrap(), &self.descriptor, self.timeout)
            .map_err(|error| HantekUsbError::ManufacturerReadUsbError { error })
    }

    pub fn get_product(&self) -> Result<String, HantekUsbError> {
        if self.language.is_none() {
            return Err(HantekUsbError::ProductReadNoLanguageAvailable);
        }

        self.handle
            .read_product_string(self.language.unwrap(), &self.descriptor, self.timeout)
            .map_err(|error| HantekUsbError::ProductReadUsbError { error })
    }

    pub fn claim(&mut self) -> Result<(), HantekUsbError> {
        if let Some(already_claimed) = self.claimed_interface {
            return Err(HantekUsbError::InterfaceAlreadyClaimed {
                interface: already_claimed,
            });
        }

        let mut errors = vec![];
        for interface in self.config.interfaces() {
            let try_claim = self.handle.claim_interface(interface.number());
            if try_claim.is_ok() {
                self.claimed_interface = Some(interface.number());
                return Ok(());
            } else {
                errors.push((interface.number(), try_claim.err().unwrap()));
            }
        }

        Err(HantekUsbError::UsbInterfaceClaimError { errors })
    }

    pub fn release(&mut self) -> Result<(), HantekUsbError> {
        match self.claimed_interface {
            None => Ok(()),
            Some(interface_number) => self
                .handle
                .release_interface(interface_number)
                .map_err(|error| HantekUsbError::UsbInterfaceReleaseError { error }),
        }
    }

    pub fn write(&mut self, endpoint: u8, buf: &[u8]) -> Result<usize, HantekUsbError> {
        if self.claimed_interface.is_none() {
            return Err(HantekUsbError::NoInterfaceClaimed);
        }

        self.handle
            .write_bulk(endpoint, buf, self.timeout)
            .map_err(|error| HantekUsbError::WriteError { error })
    }

    pub fn read(&mut self, endpoint: u8, buf: &mut [u8]) -> Result<usize, HantekUsbError> {
        if self.claimed_interface.is_none() {
            return Err(HantekUsbError::NoInterfaceClaimed);
        }

        self.handle
            .read_bulk(endpoint, buf, self.timeout)
            .map_err(|error| HantekUsbError::ReadError { error })
    }

    pub fn pid(&self) -> u16 {
        self.descriptor.product_id()
    }

    pub fn vid(&self) -> u16 {
        self.descriptor.vendor_id()
    }

    pub fn pretty_printed_device_info(&self) -> String {
        format!(
            "USB Bus={:03} Device={:03} ID={:04X}:{:04X} Speed={}\n\
            manufacturer={}\n\
            product={}",
            self.device.bus_number(),
            self.device.address(),
            self.pid(),
            self.vid(),
            match self.device.speed() {
                Speed::Unknown => "Unknown",
                Speed::Low => "Low (1.5MPps)",
                Speed::Full => "Full (12MBps)",
                Speed::High => "High (480MBps)",
                Speed::Super => "Super (5000MBps)",
            },
            self.get_manufacturer()
                .unwrap_or_else(|_| "ERROR".to_string()),
            self.get_product().unwrap_or_else(|_| "ERROR".to_string()),
        )
    }
}
