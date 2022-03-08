use std::time::Duration;

use anyhow::bail;
use libusb::{ConfigDescriptor, Context, Device, DeviceDescriptor, DeviceHandle, Language, Speed};
use log::{debug, error, trace};

pub struct HantekUsbDevice<'a> {
    timeout: Duration,
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
    ) -> Result<Self, anyhow::Error> {
        let (device, descriptor) = Self::find_single_device(context, (vid, pid))?;
        let handle = device.open()?;
        let language = Self::get_device_language(&handle, timeout)?;
        let config = device.config_descriptor(0)?;

        Ok(Self {
            timeout,
            device,
            descriptor,
            handle,
            language,
            config,
        })
    }

    fn find_devices(context: &Context, (vid, pid): (u16, u16)) -> Vec<(Device, DeviceDescriptor)> {
        let mut hantek_devices = Vec::with_capacity(1);

        let all_devices = context.devices();
        if all_devices.is_err() {
            error!("failed to get usb devices");
            return hantek_devices;
        }
        let all_devices = all_devices.unwrap();

        for device in all_devices.iter() {
            let device_descriptor = device.device_descriptor();
            if device_descriptor.is_err() {
                debug!(
                    "could not open device descriptor, bus={} address={}",
                    device.bus_number(),
                    device.address()
                );
                continue;
            }
            let device_descriptor = device_descriptor.unwrap();

            if device_descriptor.vendor_id() != vid || device_descriptor.product_id() != pid {
                trace!(
                    "skipping device on mismatch, pid={} vid={}",
                    device_descriptor.product_id(),
                    device_descriptor.vendor_id()
                );
                continue;
            }

            hantek_devices.push((device, device_descriptor));
        }

        hantek_devices
    }

    fn find_single_device(
        context: &Context,
        (vid, pid): (u16, u16),
    ) -> anyhow::Result<(Device, DeviceDescriptor)> {
        let mut devices = Self::find_devices(context, (vid, pid));
        if devices.is_empty() {
            bail!("no device found");
        }
        if devices.len() > 1 {
            bail!("too many devices found: {}", devices.len());
        }

        Ok(devices.pop().unwrap())
    }

    fn get_device_language(
        handle: &DeviceHandle,
        timeout: Duration,
    ) -> anyhow::Result<Option<Language>> {
        Ok(handle.read_languages(timeout).map(|mut it| {
            if it.len() > 1 {
                trace!(
                    "multiple languages available, choosing first. Number of languages={}",
                    it.len()
                )
            }
            it.pop()
        })?)
    }
}

impl<'a> HantekUsbDevice<'a> {
    pub fn get_manufacturer(&self) -> anyhow::Result<String> {
        if self.language.is_none() {
            bail!("no device language available, can not read manufacturer.");
        }

        Ok(self.handle.read_manufacturer_string(
            self.language.unwrap(),
            &self.descriptor,
            self.timeout,
        )?)
    }

    pub fn get_product(&self) -> anyhow::Result<String> {
        if self.language.is_none() {
            bail!("no device language available, can not read product.");
        }

        Ok(self.handle.read_product_string(
            self.language.unwrap(),
            &self.descriptor,
            self.timeout,
        )?)
    }

    pub fn pid(&self) -> u16 {
        self.descriptor.product_id()
    }

    pub fn vid(&self) -> u16 {
        self.descriptor.vendor_id()
    }

    pub fn pretty_printed_device_info(&self) -> String {
        format!(
            "{}\n{}\n{}",
            format!(
                "USB Bus={:03} Device={:03} ID={:04X}:{:04X} Speed={}",
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
            ),
            format!(
                "manufacturer={}",
                self.get_manufacturer().unwrap_or("ERROR".to_string())
            ),
            format!(
                "product={}",
                self.get_product().unwrap_or("ERROR".to_string())
            ),
        )
    }

    pub fn claim(&mut self) -> anyhow::Result<()> {
        for interface in self.config.interfaces() {
            if self.handle.claim_interface(interface.number()).is_ok() {
                return Ok(());
            }
        }

        bail!("could not claim any of interfaces");
    }

    pub fn release(&mut self) -> anyhow::Result<()> {
        let mut any_error = false;
        for interface in self.config.interfaces() {
            if self.handle.release_interface(interface.number()).is_err() {
                any_error = true;
            }
        }

        if any_error {
            bail!("could not release at least one of interfaces");
        } else {
            Ok(())
        }
    }
}
