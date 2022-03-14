use std::time::Duration;

use libusb::Context;
use thiserror::Error;

use crate::device::cfg::{Adjustment, AwgType, Coupling, DeviceFunction, HantekConfig, Probe, RunningStatus, Scale, TimeScale, TrapDuty, TriggerMode, TriggerSlope};
use crate::device::cmd::{HantekCommandBuilder, RawCommand};
use crate::device::usb::{HantekUsbDevice, HantekUsbError};
use crate::models::hantek2d42_codes::*;

const IDX: u8 = 0x00;
const BOH: u8 = 0x0A;
const NUM_CHANNELS: usize = 2;

const WRITE_ENDPOINT: u8 = 2;
const READ_ENDPOINT: u8 = 0x80 | 1;

#[derive(Error, Debug)]
pub enum Hantek2D42Error {
    #[error("error with usb device")]
    HantekUsbError {
        error: HantekUsbError,
        failed_action: &'static str,
    },

    #[error("missing or bad channel adjustment")]
    ChannelAdjustmentError,

    #[error("missing or bad time offset adjustment")]
    TimeOffsetAdjustmentError,

    #[error("missing or bad trigger level adjustment")]
    TriggerLevelAdjustmentError,
}

pub struct Hantek2D42<'a> {
    pub usb: HantekUsbDevice<'a>,
    config: HantekConfig,
}

impl<'a> Hantek2D42<'a> {
    pub fn new(usb: HantekUsbDevice<'a>, config: HantekConfig) -> Self {
        Self {
            usb,
            config,
        }
    }

    pub fn open(context: &'a Context, timeout: Duration) -> Result<Self, Hantek2D42Error> {
        let usb = HantekUsbDevice::open(context, timeout, (VENDOR_ID__2D42, PRODUCT_ID__2D42))
            .map_err(|error| Hantek2D42Error::HantekUsbError {
                error,
                failed_action: "device open",
            })?;
        let config = HantekConfig::new(timeout, NUM_CHANNELS);
        Ok(Self::new(usb, config))
    }

    /// ================================================================= DEVICE

    pub fn get_config(&self) -> &HantekConfig {
        &self.config
    }

    pub fn start(&mut self) -> Result<(), Hantek2D42Error> {
        let cmd: RawCommand = Self::cmd(FUNC_SCOPE_SETTING)
            .set_cmd(SCOPE_START_STOP)
            .set_val0(1)
            .into();

        self.usb
            .write(WRITE_ENDPOINT, &cmd)
            .map_err(|error| Hantek2D42Error::HantekUsbError {
                error,
                failed_action: "sending Start command to device",
            })
            .map(|_| {
                self.config.running_status = Some(RunningStatus::Start);
            })
    }

    pub fn stop(&mut self) -> Result<(), Hantek2D42Error> {
        let cmd: RawCommand = Self::cmd(FUNC_SCOPE_SETTING)
            .set_cmd(SCOPE_START_STOP)
            .set_val0(0)
            .into();

        self.usb
            .write(WRITE_ENDPOINT, &cmd)
            .map_err(|error| Hantek2D42Error::HantekUsbError {
                error,
                failed_action: "sending Stop command to device",
            })
            .map(|_| {
                self.config.running_status = Some(RunningStatus::Stop);
            })
    }

    pub fn set_device_function(&mut self, function: DeviceFunction) -> Result<(), Hantek2D42Error> {
        let cmd: RawCommand = Self::cmd(FUNC_SCREEN_SETTING)
            .set_cmd(0)
            .set_val0(match function {
                DeviceFunction::Scope => SCREEN_VAL_SCOPE,
                DeviceFunction::AWG => SCREEN_VAL_AWG,
                DeviceFunction::DMM => SCREEN_VAL_DMM,
            })
            .into();

        self.usb
            .write(WRITE_ENDPOINT, &cmd)
            .map_err(|error| Hantek2D42Error::HantekUsbError {
                error,
                failed_action: "setting device function",
            })
            .map(|_| self.config.device_function = Some(function))
    }

    /// ================================================================ CHANNEL

    pub fn enable_channel(&mut self, channel_no: usize) -> Result<(), Hantek2D42Error> {
        self.assert_channel_no(channel_no);

        let cmd: RawCommand = Self::cmd(FUNC_SCOPE_SETTING)
            .set_cmd(match channel_no {
                1 => SCOPE_ENABLE_CH1,
                2 => SCOPE_ENABLE_CH2,
                _ => unreachable!(),
            })
            .set_val0(1)
            .into();

        self.usb
            .write(WRITE_ENDPOINT, &cmd)
            .map_err(|error| Hantek2D42Error::HantekUsbError {
                error,
                failed_action: "enabling channel",
            })
            .map(|_| {
                self.config.enabled_channels.insert(channel_no, Some(true));
            })
    }

    pub fn disable_channel(&mut self, channel_no: usize) -> Result<(), Hantek2D42Error> {
        self.assert_channel_no(channel_no);

        let cmd: RawCommand = Self::cmd(FUNC_SCOPE_SETTING)
            .set_cmd(match channel_no {
                1 => SCOPE_ENABLE_CH1,
                2 => SCOPE_ENABLE_CH2,
                _ => unreachable!(),
            })
            .set_val0(0)
            .into();

        self.usb
            .write(WRITE_ENDPOINT, &cmd)
            .map_err(|error| Hantek2D42Error::HantekUsbError {
                error,
                failed_action: "disabling channel",
            })
            .map(|_| {
                self.config.enabled_channels.insert(channel_no, Some(false));
            })
    }

    pub fn set_channel_coupling(
        &mut self,
        channel_no: usize,
        coupling: Coupling,
    ) -> Result<(), Hantek2D42Error> {
        self.assert_channel_no(channel_no);

        let cmd: RawCommand = Self::cmd(FUNC_SCOPE_SETTING)
            .set_cmd(match channel_no {
                1 => SCOPE_COUPLING_CH1,
                2 => SCOPE_COUPLING_CH2,
                _ => unreachable!(),
            })
            .set_val0(match coupling {
                Coupling::AC => SCOPE_VAL_COUPLING_AC,
                Coupling::DC => SCOPE_VAL_COUPLING_DC,
                Coupling::GND => SCOPE_VAL_COUPLING_GND,
            })
            .into();

        self.usb
            .write(WRITE_ENDPOINT, &cmd)
            .map_err(|error| Hantek2D42Error::HantekUsbError {
                error,
                failed_action: "setting channel coupling",
            })
            .map(|_| {
                self.config.channel_coupling.insert(channel_no, Some(coupling));
            })
    }

    pub fn set_channel_probe(
        &mut self,
        channel_no: usize,
        probe: Probe,
    ) -> Result<(), Hantek2D42Error> {
        self.assert_channel_no(channel_no);

        let cmd: RawCommand = Self::cmd(FUNC_SCOPE_SETTING)
            .set_cmd(match channel_no {
                1 => SCOPE_PROBE_X_CH1,
                2 => SCOPE_PROBE_X_CH2,
                _ => unreachable!(),
            })
            .set_val0(match probe {
                Probe::X1 => SCOPE_VAL_PROBE_X1,
                Probe::X10 => SCOPE_VAL_PROBE_X10,
                Probe::X100 => SCOPE_VAL_PROBE_X100,
                Probe::X1000 => SCOPE_VAL_PROBE_X1000,
            })
            .into();

        self.usb
            .write(WRITE_ENDPOINT, &cmd)
            .map_err(|error| Hantek2D42Error::HantekUsbError {
                error,
                failed_action: "setting chanel probe",
            })
            .map(|_| {
                self.config.channel_probe.insert(channel_no, Some(probe));
            })
    }

    pub fn set_channel_scale(
        &mut self,
        channel_no: usize,
        scale: Scale,
    ) -> Result<(), Hantek2D42Error> {
        self.assert_channel_no(channel_no);

        let cmd: RawCommand = Self::cmd(FUNC_SCOPE_SETTING)
            .set_cmd(match channel_no {
                1 => SCOPE_SCALE_CH1,
                2 => SCOPE_SCALE_CH2,
                _ => unreachable!(),
            })
            .set_val0(match scale {
                Scale::mv10 => SCOPE_VAL_SCALE_10mV,
                Scale::mv20 => SCOPE_VAL_SCALE_20mV,
                Scale::mv50 => SCOPE_VAL_SCALE_50mV,
                Scale::mv100 => SCOPE_VAL_SCALE_100mV,
                Scale::mv200 => SCOPE_VAL_SCALE_200mV,
                Scale::mv500 => SCOPE_VAL_SCALE_500mV,
                Scale::v1 => SCOPE_VAL_SCALE_1V,
                Scale::v2 => SCOPE_VAL_SCALE_2V,
                Scale::v5 => SCOPE_VAL_SCALE_5V,
                Scale::v10 => SCOPE_VAL_SCALE_10V,
            })
            .into();

        self.usb
            .write(WRITE_ENDPOINT, &cmd)
            .map_err(|error| Hantek2D42Error::HantekUsbError {
                error,
                failed_action: "setting channel scale",
            })
            .map(|_| {
                self.config.channel_offset_adjustment.insert(channel_no, Some(Adjustment::new(
                    4.0 * scale.raw_value(),
                    -4.0 * scale.raw_value(),
                )));
                self.config.channel_scale.insert(channel_no, Some(scale));
            })
    }

    pub fn set_channel_offset_with_auto_adjustment(
        &mut self,
        channel_no: usize,
        offset: f32,
    ) -> Result<(), Hantek2D42Error> {
        if offset.is_nan() || offset.is_infinite() {
            panic!(
                "invalid value for channel offset, channel_no={}, offset={}",
                channel_no, offset
            );
        }
        // TODO sanitize offset value range.

        let adjustment = self.config.channel_offset_adjustment[&channel_no].as_ref();
        if adjustment.is_none() {
            return Err(Hantek2D42Error::ChannelAdjustmentError);
        }
        let adjustment = adjustment.unwrap();
        if !adjustment.are_limits_sane() || adjustment.limits_are_zero() {
            return Err(Hantek2D42Error::ChannelAdjustmentError);
        }

        let dev_offset = {
            let mut dev_offset = offset - adjustment.lower;
            dev_offset *= 200.0;
            dev_offset /= adjustment.upper - adjustment.lower;
            dev_offset
        };

        self.set_channel_offset(channel_no, dev_offset as u8)
    }

    pub fn set_channel_offset(
        &mut self,
        channel_no: usize,
        offset: u8,
    ) -> Result<(), Hantek2D42Error> {
        self.assert_channel_no(channel_no);
        // TODO sanitize offset value range.

        let cmd: RawCommand = Self::cmd(FUNC_SCOPE_SETTING)
            .set_cmd(match channel_no {
                1 => SCOPE_OFFSET_CH1,
                2 => SCOPE_OFFSET_CH2,
                _ => unreachable!(),
            })
            .set_val0(offset)
            .into();

        self.usb
            .write(WRITE_ENDPOINT, &cmd)
            .map_err(|error| Hantek2D42Error::HantekUsbError {
                error,
                failed_action: "setting channel offset",
            })
            .map(|_| {
                self.config.channel_offset.insert(channel_no, Some(offset as f32));
            })
    }

    pub fn channel_enable_bandwidth_limit(
        &mut self,
        channel_no: usize,
    ) -> Result<(), Hantek2D42Error> {
        self.assert_channel_no(channel_no);

        let cmd: RawCommand = Self::cmd(FUNC_SCOPE_SETTING)
            .set_cmd(match channel_no {
                1 => SCOPE_BW_LIMIT_CH1,
                2 => SCOPE_BW_LIMIT_CH2,
                _ => unreachable!(),
            })
            .set_val0(1)
            .into();

        self.usb
            .write(WRITE_ENDPOINT, &cmd)
            .map_err(|error| Hantek2D42Error::HantekUsbError {
                error,
                failed_action: "enabling channel bandwidth limit",
            })
            .map(|_| {
                self.config.channel_bandwidth_limit.insert(channel_no, Some(true));
            })
    }

    pub fn channel_disable_bandwidth_limit(
        &mut self,
        channel_no: usize,
    ) -> Result<(), Hantek2D42Error> {
        self.assert_channel_no(channel_no);

        let cmd: RawCommand = Self::cmd(FUNC_SCOPE_SETTING)
            .set_cmd(match channel_no {
                1 => SCOPE_BW_LIMIT_CH1,
                2 => SCOPE_BW_LIMIT_CH2,
                _ => unreachable!(),
            })
            .set_val0(0)
            .into();

        self.usb
            .write(WRITE_ENDPOINT, &cmd)
            .map_err(|error| Hantek2D42Error::HantekUsbError {
                error,
                failed_action: "disabling channel bandwidth limit",
            })
            .map(|_| {
                self.config.channel_bandwidth_limit.insert(channel_no, Some(false));
            })
    }

    pub fn capture(
        &mut self,
        channels: &[usize],
        num_samples: usize,
    ) -> Result<Vec<u8>, Hantek2D42Error> {
        for channel_no in channels {
            self.assert_channel_no(*channel_no);
        }

        let num_channels = {
            let ch1 = if channels.contains(&1) { 1 } else { 0 };
            let ch2 = if channels.contains(&2) { 1 } else { 0 };
            ch1 + ch2
        };

        let cmd: RawCommand = Self::cmd(FUNC_SCOPE_CAPTURE)
            .set_cmd(SCOPE_START_RECV)
            .set_val_u16(
                ((num_samples * num_channels) / 2) as u16,
                ((num_samples * num_channels) / 2) as u16,
            )
            .into();

        let mut buffer = vec![0; (num_samples * num_channels) as usize];
        let mut count = 0;
        while count < (num_samples as usize) {
            let length = if (num_samples * num_channels) - count < 64 {
                num_samples - count
            } else {
                64
            };
            self.usb.write(WRITE_ENDPOINT, &cmd).map_err(|error| {
                Hantek2D42Error::HantekUsbError {
                    error,
                    failed_action: "capture write command",
                }
            })?;
            let buf = &mut buffer[count..(count + length)];
            let actual_len = self.usb.read(READ_ENDPOINT, buf).map_err(|error| {
                Hantek2D42Error::HantekUsbError {
                    error,
                    failed_action: "capture read",
                }
            })?;
            count += actual_len;
        }

        Ok(buffer)
    }

    /// ================================================================== SCOPE

    pub fn set_time_scale(&mut self, time_scale: TimeScale) -> Result<(), Hantek2D42Error> {
        let raw = match time_scale {
            TimeScale::ns5 => SCOPE_VAL_SCALE_TIME_5ns,
            TimeScale::ns10 => SCOPE_VAL_SCALE_TIME_10ns,
            TimeScale::ns20 => SCOPE_VAL_SCALE_TIME_20ns,
            TimeScale::ns50 => SCOPE_VAL_SCALE_TIME_50ns,
            TimeScale::ns100 => SCOPE_VAL_SCALE_TIME_100ns,
            TimeScale::ns200 => SCOPE_VAL_SCALE_TIME_200ns,
            TimeScale::ns500 => SCOPE_VAL_SCALE_TIME_500ns,
            TimeScale::us1 => SCOPE_VAL_SCALE_TIME_1us,
            TimeScale::us2 => SCOPE_VAL_SCALE_TIME_2us,
            TimeScale::us5 => SCOPE_VAL_SCALE_TIME_5us,
            TimeScale::us10 => SCOPE_VAL_SCALE_TIME_10us,
            TimeScale::us20 => SCOPE_VAL_SCALE_TIME_20us,
            TimeScale::us50 => SCOPE_VAL_SCALE_TIME_50us,
            TimeScale::us100 => SCOPE_VAL_SCALE_TIME_100us,
            TimeScale::us200 => SCOPE_VAL_SCALE_TIME_200us,
            TimeScale::us500 => SCOPE_VAL_SCALE_TIME_500us,
            TimeScale::ms1 => SCOPE_VAL_SCALE_TIME_1ms,
            TimeScale::ms2 => SCOPE_VAL_SCALE_TIME_2ms,
            TimeScale::ms5 => SCOPE_VAL_SCALE_TIME_5ms,
            TimeScale::ms10 => SCOPE_VAL_SCALE_TIME_10ms,
            TimeScale::ms20 => SCOPE_VAL_SCALE_TIME_20ms,
            TimeScale::ms50 => SCOPE_VAL_SCALE_TIME_50ms,
            TimeScale::ms100 => SCOPE_VAL_SCALE_TIME_100ms,
            TimeScale::ms200 => SCOPE_VAL_SCALE_TIME_200ms,
            TimeScale::ms500 => SCOPE_VAL_SCALE_TIME_500ms,
            TimeScale::s1 => SCOPE_VAL_SCALE_TIME_1s,
            TimeScale::s2 => SCOPE_VAL_SCALE_TIME_2s,
            TimeScale::s5 => SCOPE_VAL_SCALE_TIME_5s,
            TimeScale::s10 => SCOPE_VAL_SCALE_TIME_10s,
            TimeScale::s20 => SCOPE_VAL_SCALE_TIME_20s,
            TimeScale::s50 => SCOPE_VAL_SCALE_TIME_50s,
            TimeScale::s100 => SCOPE_VAL_SCALE_TIME_100s,
            TimeScale::s200 => SCOPE_VAL_SCALE_TIME_200s,
            TimeScale::s500 => SCOPE_VAL_SCALE_TIME_500s,
        };

        let cmd: RawCommand = Self::cmd(FUNC_SCOPE_SETTING)
            .set_cmd(SCOPE_SCALE_TIME)
            .set_val0(raw)
            .into();

        self.usb
            .write(WRITE_ENDPOINT, &cmd)
            .map_err(|error| Hantek2D42Error::HantekUsbError {
                error,
                failed_action: "setting time scale",
            })
            .map(|_| {
                self.config.time_offset_adjustment = Some(Adjustment::new(
                    15.0 * (raw as f32),
                    -15.0 * (raw as f32),
                ));
                self.config.time_scale = Some(time_scale);
            })
    }

    pub fn set_time_offset_with_auto_adjustment(
        &mut self,
        time_offset: f32,
    ) -> Result<(), Hantek2D42Error> {
        if time_offset.is_nan() || time_offset.is_infinite() {
            panic!("bad value for time_offset={}", time_offset);
        }

        let adjustment = self.config.time_offset_adjustment.as_ref();
        if adjustment.is_none() {
            return Err(Hantek2D42Error::TimeOffsetAdjustmentError);
        }
        let adjustment = adjustment.unwrap();
        if !adjustment.are_limits_sane() || adjustment.limits_are_zero() {
            return Err(Hantek2D42Error::TimeOffsetAdjustmentError);
        }

        // TODO somehow set upper 2 bytes to zero.
        let dev_time_offset = {
            let mut dev_time_offset = time_offset - adjustment.lower / 15.0 * 6.0;
            dev_time_offset *= 15.0 * 2.0 * 25.0;
            dev_time_offset /= adjustment.upper - adjustment.lower;
            dev_time_offset = dev_time_offset.round();
            dev_time_offset
        };

        self.set_time_offset(dev_time_offset as u32)
    }

    pub fn set_time_offset(&mut self, time_offset: u32) -> Result<(), Hantek2D42Error> {
        let cmd: RawCommand = Self::cmd(FUNC_SCOPE_SETTING)
            .set_cmd(SCOPE_OFFSET_TIME)
            .set_val_u32(time_offset)
            .into();

        self.usb
            .write(WRITE_ENDPOINT, &cmd)
            .map_err(|error| Hantek2D42Error::HantekUsbError {
                error,
                failed_action: "setting time offset",
            })
            .map(|_| {
                self.config.time_offset = Some(time_offset as f32);
            })
    }

    pub fn set_trigger_source(&mut self, channel_no: usize) -> Result<(), Hantek2D42Error> {
        self.assert_channel_no(channel_no);

        let scale = self
            .config
            .channel_scale[&channel_no]
            .as_ref()
            .map(|it| it.raw_value());
        if scale.is_none() {
            return Err(Hantek2D42Error::TriggerLevelAdjustmentError);
        }
        let scale = scale.unwrap();

        let cmd: RawCommand = Self::cmd(FUNC_SCOPE_SETTING)
            .set_cmd(SCOPE_TRIGGER_SOURCE)
            .set_val0((channel_no - 1) as u8)
            .into();

        self.usb
            .write(WRITE_ENDPOINT, &cmd)
            .map_err(|error| Hantek2D42Error::HantekUsbError {
                error,
                failed_action: "setting trigger source",
            })
            .map(|_| {
                self.config.trigger_source_channel = Some(channel_no);
                self.config.trigger_level_adjustment = Some(Adjustment::new(
                    4.0 * scale,
                    -4.0 * scale,
                ));
            })
    }

    pub fn set_trigger_slope(
        &mut self,
        trigger_slope: TriggerSlope,
    ) -> Result<(), Hantek2D42Error> {
        let cmd: RawCommand = Self::cmd(FUNC_SCOPE_SETTING)
            .set_cmd(SCOPE_TRIGGER_SLOPE)
            .set_val0(match trigger_slope {
                TriggerSlope::Rising => SCOPE_VAL_TRIGGER_SLOPE_RISING,
                TriggerSlope::Falling => SCOPE_VAL_TRIGGER_SLOPE_FALLING,
                TriggerSlope::Both => SCOPE_VAL_TRIGGER_SLOPE_BOTH,
            })
            .into();

        self.usb
            .write(WRITE_ENDPOINT, &cmd)
            .map_err(|error| Hantek2D42Error::HantekUsbError {
                error,
                failed_action: "setting trigger slope",
            })
            .map(|_| {
                self.config.trigger_slope = Some(trigger_slope);
            })
    }

    pub fn set_trigger_mode(&mut self, trigger_mode: TriggerMode) -> Result<(), Hantek2D42Error> {
        let cmd: RawCommand = Self::cmd(FUNC_SCOPE_SETTING)
            .set_cmd(SCOPE_TRIGGER_MODE)
            .set_val0(match trigger_mode {
                TriggerMode::Auto => SCOPE_VAL_TRIGGER_MODE_AUTO,
                TriggerMode::Normal => SCOPE_VAL_TRIGGER_MODE_NORMAL,
                TriggerMode::Single => SCOPE_VAL_TRIGGER_MODE_SINGLE,
            })
            .into();

        self.usb
            .write(WRITE_ENDPOINT, &cmd)
            .map_err(|error| Hantek2D42Error::HantekUsbError {
                error,
                failed_action: "setting trigger mode",
            })
            .map(|_| {
                self.config.trigger_mode = Some(trigger_mode);
            })
    }

    pub fn set_trigger_level_with_auto_adjustment(
        &mut self,
        trigger_level: f32,
    ) -> Result<(), Hantek2D42Error> {
        if trigger_level.is_nan() || trigger_level.is_infinite() {
            panic!(
                "invalid value for trigger level, trigger_level={}",
                trigger_level
            );
        }

        let adjustment = self.config.trigger_level_adjustment.as_ref();
        if adjustment.is_none() {
            return Err(Hantek2D42Error::TriggerLevelAdjustmentError);
        }
        let adjustment = adjustment.unwrap();
        if !adjustment.are_limits_sane() || adjustment.limits_are_zero() {
            return Err(Hantek2D42Error::TriggerLevelAdjustmentError);
        }

        let dev_trigger_level = {
            let mut dev_trigger_level = trigger_level - adjustment.lower;
            dev_trigger_level *= 200.0;
            dev_trigger_level /= adjustment.upper - adjustment.lower;
            dev_trigger_level
        };

        self.set_trigger_level(dev_trigger_level as u8)
    }

    pub fn set_trigger_level(&mut self, trigger_level: u8) -> Result<(), Hantek2D42Error> {
        let cmd: RawCommand = Self::cmd(FUNC_SCOPE_SETTING)
            .set_cmd(SCOPE_TRIGGER_LEVEL)
            .set_val0(trigger_level)
            .into();

        self.usb
            .write(WRITE_ENDPOINT, &cmd)
            .map_err(|error| Hantek2D42Error::HantekUsbError {
                error,
                failed_action: "setting trigger level",
            })
            .map(|_| {
                self.config.trigger_level = Some(trigger_level as f32)
            })
    }

    ///=================================================================== AWG

    pub fn set_awg_type(&mut self, awg_type: AwgType) -> Result<(), Hantek2D42Error> {
        let cmd: RawCommand = Self::cmd(FUNC_AWG_SETTING)
            .set_cmd(AWG_TYPE)
            .set_val0(match awg_type {
                AwgType::Square => AWG_VAL_TYPE_SQUARE,
                AwgType::Ramp => AWG_VAL_TYPE_RAMP,
                AwgType::Sin => AWG_VAL_TYPE_SIN,
                AwgType::Trap => AWG_VAL_TYPE_TRAP,
                AwgType::Arb1 => AWG_VAL_TYPE_ARB1,
                AwgType::Arb2 => AWG_VAL_TYPE_ARB2,
                AwgType::Arb3 => AWG_VAL_TYPE_ARB3,
                AwgType::Arb4 => AWG_VAL_TYPE_ARB4,
            })
            .into();

        self.usb
            .write(WRITE_ENDPOINT, &cmd)
            .map_err(|error| Hantek2D42Error::HantekUsbError {
                error,
                failed_action: "setting awg mode",
            })
            .map(|_| {
                self.config.awg_type = Some(awg_type);
            })
    }

    pub fn set_awg_frequency(&mut self, frequency: f32) -> Result<(), Hantek2D42Error> {
        // TODO sanitize frequency?

        let cmd: RawCommand = Self::cmd(FUNC_AWG_SETTING)
            .set_cmd(AWG_FREQ)
            .set_val_u32(frequency as u32)
            .into();

        self.usb
            .write(WRITE_ENDPOINT, &cmd)
            .map_err(|error| Hantek2D42Error::HantekUsbError {
                error,
                failed_action: "setting awg frequency",
            })
            .map(|_| {
                self.config.awg_frequency = Some(frequency);
            })
    }

    pub fn set_awg_amplitude(&mut self, amplitude: f32) -> Result<(), Hantek2D42Error> {
        // TODO sanitize amplitude?

        let raw = (amplitude.abs() * 1000.0) as u16;
        let sign = if amplitude.is_sign_negative() {
            1u16
        } else {
            0u16
        };
        let cmd: RawCommand = Self::cmd(FUNC_AWG_SETTING)
            .set_cmd(AWG_AMPLITUDE)
            .set_val_u16(raw, sign)
            .into();

        self.usb
            .write(WRITE_ENDPOINT, &cmd)
            .map_err(|error| Hantek2D42Error::HantekUsbError {
                error,
                failed_action: "setting awg amplitude",
            })
            .map(|_| {
                self.config.awg_amplitude = Some(amplitude);
            })
    }

    pub fn set_awg_offset(&mut self, offset: f32) -> Result<(), Hantek2D42Error> {
        // TODO sanitize offset?

        let raw = (offset.abs() * 1000.0) as u16;
        let sign = if offset.is_sign_negative() {
            1u16
        } else {
            0u16
        };
        let cmd: RawCommand = Self::cmd(FUNC_AWG_SETTING)
            .set_cmd(AWG_OFFSET)
            .set_val_u16(raw, sign)
            .into();

        self.usb
            .write(WRITE_ENDPOINT, &cmd)
            .map_err(|error| Hantek2D42Error::HantekUsbError {
                error,
                failed_action: "setting awg offset",
            })
            .map(|_| {
                self.config.awg_offset = Some(offset);
            })
    }

    pub fn set_awg_duty_square(&mut self, duty: f32) -> Result<(), Hantek2D42Error> {
        // TODO sanitize duty?

        let raw = (duty * 100.0) as u16;
        let cmd: RawCommand = Self::cmd(FUNC_AWG_SETTING)
            .set_cmd(AWG_SQUARE_DUTY)
            .set_val_u16(raw, 0)
            .into();

        self.usb
            .write(WRITE_ENDPOINT, &cmd)
            .map_err(|error| Hantek2D42Error::HantekUsbError {
                error,
                failed_action: "setting awg square duty",
            })
            .map(|_| {
                self.config.awg_duty_square = Some(duty);
            })
    }

    pub fn set_awg_duty_ramp(&mut self, duty: f32) -> Result<(), Hantek2D42Error> {
        // TODO sanitize duty?

        let raw = (duty * 100.0) as u16;

        let cmd: RawCommand = Self::cmd(FUNC_AWG_SETTING)
            .set_cmd(AWG_RAMP_DUTY)
            .set_val_u16(raw, 0)
            .into();

        self.usb
            .write(WRITE_ENDPOINT, &cmd)
            .map_err(|error| Hantek2D42Error::HantekUsbError {
                error,
                failed_action: "setting awg ramp duty",
            })
            .map(|_| {
                self.config.awg_duty_ramp = Some(duty);
            })
    }

    pub fn set_awg_duty_trap(
        &mut self,
        high: f32,
        low: f32,
        rise: f32,
    ) -> Result<(), Hantek2D42Error> {
        // TODO sanitize high, low, rise?

        let raw_high = (high * 100.0) as u8;
        let raw_low = (low * 100.0) as u8;
        let raw_rise = (rise * 100.0) as u8;

        let cmd: RawCommand = Self::cmd(FUNC_AWG_SETTING)
            .set_cmd(AWG_TRAP_DUTY)
            .set_val_u8(raw_rise, raw_high, raw_low, 0)
            .into();

        self.usb
            .write(WRITE_ENDPOINT, &cmd)
            .map_err(|error| Hantek2D42Error::HantekUsbError {
                error,
                failed_action: "setting awg ramp duty",
            })
            .map(|_| {
                self.config.awg_duty_trap = Some(TrapDuty {
                    high,
                    low,
                    rise,
                });
            })
    }

    pub fn awg_start(&mut self) -> Result<(), Hantek2D42Error> {
        let cmd: RawCommand = Self::cmd(FUNC_AWG_SETTING)
            .set_cmd(AWG_START_STOP)
            .set_val0(1)
            .into();

        self.usb
            .write(WRITE_ENDPOINT, &cmd)
            .map_err(|error| Hantek2D42Error::HantekUsbError {
                error,
                failed_action: "awg start",
            })
            .map(|_| {
                self.config.awg_running_status = Some(RunningStatus::Start);
            })
    }

    pub fn awg_stop(&mut self) -> Result<(), Hantek2D42Error> {
        let cmd: RawCommand = Self::cmd(FUNC_AWG_SETTING)
            .set_cmd(AWG_START_STOP)
            .set_val0(0)
            .into();

        self.usb
            .write(WRITE_ENDPOINT, &cmd)
            .map_err(|error| Hantek2D42Error::HantekUsbError {
                error,
                failed_action: "awg stop",
            })
            .map(|_| {
                self.config.awg_running_status = Some(RunningStatus::Stop);
            })
    }

    ///=============================================================== INTERNAL

    fn cmd(func: u16) -> HantekCommandBuilder {
        HantekCommandBuilder::new()
            .set_idx(IDX)
            .set_boh(BOH)
            .set_func(func)
            .set_last(0)
    }

    fn assert_channel_no(&self, channel_no: usize) {
        if channel_no != 1 && channel_no != 2 {
            panic!(
                "channel_no out of bound, expected 1 or 2, got: {}",
                channel_no
            );
        }
    }
}
