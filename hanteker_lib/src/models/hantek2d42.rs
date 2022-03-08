use std::time::Duration;

use anyhow::bail;
use libusb::Context;
use log::{debug, trace, warn};

use crate::device::cfg::{
    AwgType, Coupling, DeviceFunction, HantekConfig, Probe, Scale, TimeScale, TriggerMode,
    TriggerSlope,
};
use crate::device::cmd::{HantekCommandBuilder, RawCommand};
use crate::device::usb::HantekUsbDevice;
use crate::models::hantek2d42_codes::*;

const IDX: u8 = 0x00;
const BOH: u8 = 0x0A;

pub struct Hantek2D42<'a> {
    pub usb: HantekUsbDevice<'a>,
    config: HantekConfig,
}

impl<'a> Hantek2D42<'a> {
    pub fn open(context: &'a Context, timeout: Duration) -> Result<Self, anyhow::Error> {
        Ok(Self {
            usb: HantekUsbDevice::open(context, timeout, (VENDOR_ID__2D42, PRODUCT_ID__2D42))?,
            config: HantekConfig::new(timeout, 2),
        })
    }

    pub fn start(&mut self) -> anyhow::Result<()> {
        let cmd: RawCommand = HantekCommandBuilder::new()
            .set_idx(IDX)
            .set_boh(BOH)
            .set_func(FUNC_SCOPE_SETTING)
            .set_cmd(SCOPE_START_STOP)
            .set_val0(1)
            .set_last(0)
            .build()
            .into();

        trace!("setting device to Start");
        let result = self.write(&cmd);

        if result.is_ok() {
            debug!("device set to Start");
            self.config.start();
            Ok(())
        } else {
            warn!(
                "failed to set device to Start, error={}",
                result.as_ref().err().unwrap()
            );
            Err(result.err().unwrap().into())
        }
    }

    pub fn stop(&mut self) -> anyhow::Result<()> {
        let cmd: RawCommand = HantekCommandBuilder::new()
            .set_idx(IDX)
            .set_boh(BOH)
            .set_func(FUNC_SCOPE_SETTING)
            .set_cmd(SCOPE_START_STOP)
            .set_val0(0)
            .set_last(0)
            .build()
            .into();

        trace!("setting device to Stop");
        let result = self.write(&cmd);

        if result.is_ok() {
            debug!("device set to Stop");
            self.config.stop();
            Ok(())
        } else {
            warn!(
                "failed to set device to Stop, error={}",
                result.as_ref().err().unwrap()
            );
            Err(result.err().unwrap().into())
        }
    }

    pub fn set_device_function(&mut self, function: DeviceFunction) -> anyhow::Result<()> {
        let cmd: RawCommand = HantekCommandBuilder::new()
            .set_idx(IDX)
            .set_boh(BOH)
            .set_func(FUNC_SCREEN_SETTING)
            .set_cmd(0)
            .set_val0(match function {
                DeviceFunction::Scope => SCREEN_VAL_SCOPE,
                DeviceFunction::AWG => SCREEN_VAL_AWG,
                DeviceFunction::DMM => SCREEN_VAL_DMM,
            })
            .set_last(0)
            .build()
            .into();

        trace!(
            "setting device function, function={}",
            function.my_to_string()
        );
        let result = self.write(&cmd);

        if result.is_ok() {
            debug!("device function set, function={}", function.my_to_string());
            self.config.set_device_function(function);
            Ok(())
        } else {
            warn!(
                "failed to set device function, error={}",
                result.as_ref().err().unwrap()
            );
            Err(result.err().unwrap().into())
        }
    }

    /// ================================================================ CHANNEL

    pub fn enable_channel(&mut self, channel_no: usize) -> anyhow::Result<()> {
        self.assert_channel_no(channel_no);

        let raw_value = 1;
        let cmd: RawCommand = HantekCommandBuilder::new()
            .set_idx(IDX)
            .set_boh(BOH)
            .set_func(FUNC_SCOPE_SETTING)
            .set_cmd(match channel_no {
                1 => SCOPE_ENABLE_CH1,
                2 => SCOPE_ENABLE_CH2,
                _ => panic!("unknown channel={}", channel_no),
            })
            .set_val0(raw_value)
            .set_last(0)
            .build()
            .into();

        trace!("enabling channel_no={}", channel_no);
        let result = self.write(&cmd);

        if result.is_ok() {
            debug!("channel enabled, channel_no={}", channel_no);
            self.config.enable_channel(channel_no);
            Ok(())
        } else {
            warn!(
                "enabling channel failed, channel_no={}, error={}",
                channel_no,
                result.as_ref().err().unwrap()
            );
            Err(result.err().unwrap().into())
        }
    }

    pub fn disable_channel(&mut self, channel_no: usize) -> anyhow::Result<()> {
        self.assert_channel_no(channel_no);

        let raw_value = 0;
        let cmd: RawCommand = HantekCommandBuilder::new()
            .set_idx(IDX)
            .set_boh(BOH)
            .set_func(FUNC_SCOPE_SETTING)
            .set_cmd(match channel_no {
                1 => SCOPE_ENABLE_CH1,
                2 => SCOPE_ENABLE_CH2,
                _ => panic!("unknown channel={}", channel_no),
            })
            .set_val0(raw_value)
            .set_last(0)
            .build()
            .into();

        trace!("enabling channel_no={}", channel_no);
        let result = self.write(&cmd);

        if result.is_ok() {
            debug!("channel enabled, channel_no={}", channel_no);
            self.config.disable_channel(channel_no);
            Ok(())
        } else {
            warn!(
                "enabling channel failed, channel_no={}, error={}",
                channel_no,
                result.as_ref().err().unwrap()
            );
            Err(result.err().unwrap().into())
        }
    }

    pub fn set_channel_coupling(
        &mut self,
        channel_no: usize,
        coupling: Coupling,
    ) -> anyhow::Result<()> {
        self.assert_channel_no(channel_no);

        let cmd: RawCommand = HantekCommandBuilder::new()
            .set_idx(IDX)
            .set_boh(BOH)
            .set_func(FUNC_SCOPE_SETTING)
            .set_cmd(match channel_no {
                1 => SCOPE_COUPLING_CH1,
                2 => SCOPE_COUPLING_CH2,
                _ => panic!("unknown channel={}", channel_no),
            })
            .set_val0(match coupling {
                Coupling::AC => SCOPE_VAL_COUPLING_AC,
                Coupling::DC => SCOPE_VAL_COUPLING_DC,
                Coupling::GND => SCOPE_VAL_COUPLING_GND,
            })
            .set_last(0)
            .build()
            .into();

        trace!(
            "setting channel coupling, channel_no={}, coupling={}",
            channel_no,
            coupling.my_to_string()
        );
        let result = self.write(&cmd);

        if result.is_ok() {
            debug!(
                "channel coupling set, channel_no={}, coupling={}",
                channel_no,
                coupling.my_to_string()
            );
            self.config.set_channel_coupling(channel_no, coupling);
            Ok(())
        } else {
            warn!(
                "setting coupling failed, channel_no={}, coupling={} error={}",
                channel_no,
                coupling.my_to_string(),
                result.as_ref().err().unwrap()
            );
            Err(result.err().unwrap().into())
        }
    }

    pub fn set_channel_probe(&mut self, channel_no: usize, probe: Probe) -> anyhow::Result<()> {
        self.assert_channel_no(channel_no);

        let cmd: RawCommand = HantekCommandBuilder::new()
            .set_idx(IDX)
            .set_boh(BOH)
            .set_func(FUNC_SCOPE_SETTING)
            .set_cmd(match channel_no {
                1 => SCOPE_PROBE_X_CH1,
                2 => SCOPE_PROBE_X_CH2,
                _ => panic!("unknown channel={}", channel_no),
            })
            .set_val0(match probe {
                Probe::X1 => SCOPE_VAL_PROBE_X1,
                Probe::X10 => SCOPE_VAL_PROBE_X10,
                Probe::X100 => SCOPE_VAL_PROBE_X100,
                Probe::X1000 => SCOPE_VAL_PROBE_X1000,
            })
            .set_last(0)
            .build()
            .into();

        trace!(
            "setting channel probe, channel_no={}, probe={}",
            channel_no,
            probe.my_to_string()
        );
        let result = self.write(&cmd);

        if result.is_ok() {
            debug!(
                "channel probe set, channel_no={}, probe={}",
                channel_no,
                probe.my_to_string()
            );
            self.config.set_channel_probe(channel_no, probe);
            Ok(())
        } else {
            warn!(
                "setting probe failed, channel_no={}, probe={} error={}",
                channel_no,
                probe.my_to_string(),
                result.as_ref().err().unwrap()
            );
            Err(result.err().unwrap().into())
        }
    }

    pub fn set_channel_scale(&mut self, channel_no: usize, scale: Scale) -> anyhow::Result<()> {
        self.assert_channel_no(channel_no);

        let cmd: RawCommand = HantekCommandBuilder::new()
            .set_idx(IDX)
            .set_boh(BOH)
            .set_func(FUNC_SCOPE_SETTING)
            .set_cmd(match channel_no {
                1 => SCOPE_SCALE_CH1,
                2 => SCOPE_SCALE_CH2,
                _ => panic!("unknown channel={}", channel_no),
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
            .set_last(0)
            .build()
            .into();

        trace!(
            "setting channel scale, channel_no={}, scale={}",
            channel_no,
            scale.my_to_string()
        );
        let result = self.write(&cmd);

        if result.is_ok() {
            debug!(
                "channel scale set, channel_no={}, scale={}",
                channel_no,
                scale.my_to_string()
            );
            self.config.set_channel_adjustment(
                channel_no,
                4.0 * scale.raw_value(),
                -4.0 * scale.raw_value(),
                8.0 * scale.raw_value() / 200.0,
                scale.raw_value(),
            );
            self.config.set_channel_scale(channel_no, scale);
            Ok(())
        } else {
            warn!(
                "setting scale failed, channel_no={}, scale={} error={}",
                channel_no,
                scale.my_to_string(),
                result.as_ref().err().unwrap()
            );
            Err(result.err().unwrap().into())
        }
    }

    pub fn set_channel_offset_with_auto_adjustment(
        &mut self,
        channel_no: usize,
        offset: f32,
    ) -> anyhow::Result<()> {
        self.assert_channel_no(channel_no);
        if offset.is_nan() || offset.is_infinite() {
            panic!(
                "invalid value for channel offset, channel_no={}, offset={}",
                channel_no, offset
            );
        }
        // TODO sanitize offset value range.

        let adjustment = self.config.get_channel_adjustment(channel_no);
        if adjustment.is_none() {
            bail!("adjustment missing for channel_no={}", channel_no);
        }
        let adjustment = adjustment.unwrap();
        if !adjustment.are_limits_sane() || adjustment.limits_are_zero() {
            bail!("adjustment are not sane for channel_no={}", channel_no);
        }

        let dev_offset = {
            let mut dev_offset = offset - adjustment.lower;
            dev_offset *= 200.0;
            dev_offset /= adjustment.upper - adjustment.lower;
            dev_offset
        };

        let cmd: RawCommand = HantekCommandBuilder::new()
            .set_idx(IDX)
            .set_boh(BOH)
            .set_func(FUNC_SCOPE_SETTING)
            .set_cmd(match channel_no {
                1 => SCOPE_OFFSET_CH1,
                2 => SCOPE_OFFSET_CH2,
                _ => panic!("unknown channel={}", channel_no),
            })
            .set_val0(dev_offset as u8)
            .set_last(0)
            .build()
            .into();

        trace!(
            "setting channel offset, channel_no={}, offset={} (raw={})",
            channel_no,
            offset,
            dev_offset as u8
        );
        let result = self.write(&cmd);

        if result.is_ok() {
            debug!(
                "channel offset set, channel_no={}, offset={} (raw={})",
                channel_no, offset, dev_offset as u8
            );
            self.config.set_channel_offset(channel_no, offset);
            Ok(())
        } else {
            warn!(
                "setting offset failed, channel_no={}, error={}",
                channel_no,
                result.as_ref().err().unwrap()
            );
            Err(result.err().unwrap().into())
        }
    }

    pub fn channel_enable_bandwidth_limit(&mut self, channel_no: usize) -> anyhow::Result<()> {
        self.assert_channel_no(channel_no);

        let raw_value = 1;
        let cmd: RawCommand = HantekCommandBuilder::new()
            .set_idx(IDX)
            .set_boh(BOH)
            .set_func(FUNC_SCOPE_SETTING)
            .set_cmd(match channel_no {
                1 => SCOPE_BW_LIMIT_CH1,
                2 => SCOPE_BW_LIMIT_CH2,
                _ => panic!("unknown channel={}", channel_no),
            })
            .set_val0(raw_value)
            .set_last(0)
            .build()
            .into();

        trace!("enabling bandwidth limit on channel_no={}", channel_no);
        let result = self.write(&cmd);

        if result.is_ok() {
            debug!("channel bandwidth limit enabled, channel_no={}", channel_no);
            self.config.channel_disable_bandwidth_limit(channel_no);
            Ok(())
        } else {
            warn!(
                "enabling channel bandwidth limit failed, channel_no={}, error={}",
                channel_no,
                result.as_ref().err().unwrap()
            );
            Err(result.err().unwrap().into())
        }
    }

    pub fn channel_disable_bandwidth_limit(&mut self, channel_no: usize) -> anyhow::Result<()> {
        self.assert_channel_no(channel_no);

        let raw_value = 0;
        let cmd: RawCommand = HantekCommandBuilder::new()
            .set_idx(IDX)
            .set_boh(BOH)
            .set_func(FUNC_SCOPE_SETTING)
            .set_cmd(match channel_no {
                1 => SCOPE_BW_LIMIT_CH1,
                2 => SCOPE_BW_LIMIT_CH2,
                _ => panic!("unknown channel={}", channel_no),
            })
            .set_val0(raw_value)
            .set_last(0)
            .build()
            .into();

        trace!("disabling bandwidth limit on channel_no={}", channel_no);
        let result = self.write(&cmd);

        if result.is_ok() {
            debug!("channel bandwidth limit disable, channel_no={}", channel_no);
            self.config.channel_enable_bandwidth_limit(channel_no);
            Ok(())
        } else {
            warn!(
                "disabling channel bandwidth limit failed, channel_no={}, error={}",
                channel_no,
                result.as_ref().err().unwrap()
            );
            Err(result.err().unwrap().into())
        }
    }

    pub fn capture_inf<W>(&self, channels: &[usize], chunk_size: usize, mut output: W) -> !
        where W: std::io::Write {
        loop {
            let captured = self.capture(channels, chunk_size).expect("failed");
            if output.write_all(&captured).is_err() || output.flush().is_err() {
                std::process::exit(0);
            }
        }
    }

    pub fn capture(&self, channels: &[usize], num_samples: usize) -> anyhow::Result<Vec<u8>> {
        for channel_no in channels {
            self.assert_channel_no(*channel_no);
        }

        let num_channels = {
            let ch1 = if channels.contains(&1) { 1 } else { 0 };
            let ch2 = if channels.contains(&2) { 1 } else { 0 };
            ch1 + ch2
        };

        let cmd: RawCommand = HantekCommandBuilder::new()
            .set_idx(IDX)
            .set_boh(BOH)
            .set_func(FUNC_SCOPE_CAPTURE)
            .set_cmd(SCOPE_START_RECV)
            .set_val_u16(
                ((num_samples * num_channels) / 2) as u16,
                ((num_samples * num_channels) / 2) as u16,
            )
            .set_last(0)
            .build()
            .into();

        // TODO samples or num_samples?
        let mut buffer = vec![0; (num_samples * num_channels) as usize];
        let mut count = 0;
        while count < (num_samples as usize) {
            let length = if (num_samples * num_channels) - count < 64 { num_samples - count } else { 64 };
            self.write(&cmd)?;
            let buf = &mut buffer[count..(count + length)];
            // println!("{}, {}, {:?}", count, length, &buf);
            let endpoint = 0x80 | 1;
            let actual_len = self.usb.handle.read_bulk(
                endpoint, buf, self.config.get_timeout())?;
            count += actual_len;
        }

        Ok(buffer)
    }

    /// ================================================================== SCOPE

    pub fn set_time_scale(&mut self, time_scale: TimeScale) -> anyhow::Result<()> {
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

        let cmd: RawCommand = HantekCommandBuilder::new()
            .set_idx(IDX)
            .set_boh(BOH)
            .set_func(FUNC_SCOPE_SETTING)
            .set_cmd(SCOPE_SCALE_TIME)
            .set_val0(raw)
            .set_last(0)
            .build()
            .into();

        trace!("setting time scale={}", time_scale.my_to_string());
        let result = self.write(&cmd);

        if result.is_ok() {
            debug!("time scale set, time_scale={}", time_scale.my_to_string());
            self.config.set_time_offset_adjustment(
                15.0 * (raw as f32),
                -15.0 * (raw as f32),
                (raw as f32) / 25.0,
                raw as f32,
            );
            self.config.set_time_scale(time_scale);
            Ok(())
        } else {
            warn!(
                "setting time scale failed, error={}",
                result.as_ref().err().unwrap()
            );
            Err(result.err().unwrap().into())
        }
    }

    pub fn set_time_offset_with_auto_adjustment(&mut self, time_offset: f32) -> anyhow::Result<()> {
        if time_offset.is_nan() || time_offset.is_infinite() {
            panic!("bad value for time_offset={}", time_offset);
        }

        let adjustment = self.config.get_time_offset_adjustment();
        if adjustment.is_none() {
            bail!("time offset adjustment missing");
        }
        let adjustment = adjustment.unwrap();
        if !adjustment.are_limits_sane() || adjustment.limits_are_zero() {
            bail!("adjustment are not sane for time_offset");
        }

        // TODO somehow set upper 2 bytes to zero.
        let dev_time_offset = {
            let mut dev_time_offset = time_offset - adjustment.lower / 15.0 * 6.0;
            dev_time_offset *= 15.0 * 2.0 * 25.0;
            dev_time_offset /= adjustment.upper - adjustment.lower;
            dev_time_offset = dev_time_offset.round();
            dev_time_offset
        };

        let cmd: RawCommand = HantekCommandBuilder::new()
            .set_idx(IDX)
            .set_boh(BOH)
            .set_func(FUNC_SCOPE_SETTING)
            .set_cmd(SCOPE_OFFSET_TIME)
            .set_val_u32(dev_time_offset as u32)
            .set_last(0)
            .build()
            .into();

        trace!(
            "setting time offset={} (raw={})",
            time_offset,
            dev_time_offset as u32
        );
        let result = self.write(&cmd);

        if result.is_ok() {
            debug!(
                "time offset set, time_offset={} (raw={})",
                time_offset, dev_time_offset as u32
            );
            self.config.set_time_offset(time_offset);
            Ok(())
        } else {
            warn!(
                "setting time offset failed, error={}",
                result.as_ref().err().unwrap()
            );
            Err(result.err().unwrap().into())
        }
    }

    pub fn set_trigger_source(&mut self, channel_no: usize) -> anyhow::Result<()> {
        self.assert_channel_no(channel_no);

        let cmd: RawCommand = HantekCommandBuilder::new()
            .set_idx(IDX)
            .set_boh(BOH)
            .set_func(FUNC_SCOPE_SETTING)
            .set_cmd(SCOPE_TRIGGER_SOURCE)
            .set_val0((channel_no - 1) as u8)
            .set_last(0)
            .build()
            .into();

        trace!("setting trigger source to channel={}", channel_no);
        let result = self.write(&cmd);

        if result.is_ok() {
            debug!("trigger source set, channel={}", channel_no);
            self.config.set_trigger_source_channel_no(channel_no);

            let scale = self
                .config
                .get_channel_scale(channel_no)
                .map(|it| it.raw_value());
            if scale.is_none() {
                bail!(
                    "can not set trigger level adjustments, channel scale missing, channel_no={}",
                    channel_no
                );
            }
            let scale = scale.unwrap();

            self.config.set_trigger_level_adjustment(
                4.0 * scale,
                -4.0 * scale,
                8.0 * scale / 200.0,
                scale,
            );

            Ok(())
        } else {
            warn!(
                "setting trigger source failed, error={}",
                result.as_ref().err().unwrap()
            );
            Err(result.err().unwrap().into())
        }
    }

    pub fn set_trigger_slope(&mut self, trigger_slope: TriggerSlope) -> anyhow::Result<()> {
        let cmd: RawCommand = HantekCommandBuilder::new()
            .set_idx(IDX)
            .set_boh(BOH)
            .set_func(FUNC_SCOPE_SETTING)
            .set_cmd(SCOPE_TRIGGER_SLOPE)
            .set_val0(match trigger_slope {
                TriggerSlope::Rising => SCOPE_VAL_TRIGGER_SLOPE_RISING,
                TriggerSlope::Falling => SCOPE_VAL_TRIGGER_SLOPE_FALLING,
                TriggerSlope::Both => SCOPE_VAL_TRIGGER_SLOPE_BOTH,
            })
            .set_last(0)
            .build()
            .into();

        trace!(
            "setting trigger slope, trigger_slope={}",
            trigger_slope.my_to_string()
        );
        let result = self.write(&cmd);

        if result.is_ok() {
            debug!(
                "trigger slope set, trigger_slope={}",
                trigger_slope.my_to_string()
            );
            self.config.set_trigger_slope(trigger_slope);
            Ok(())
        } else {
            warn!(
                "setting trigger slope failed, error={}",
                result.as_ref().err().unwrap()
            );
            Err(result.err().unwrap().into())
        }
    }

    pub fn set_trigger_mode(&mut self, trigger_mode: TriggerMode) -> anyhow::Result<()> {
        let cmd: RawCommand = HantekCommandBuilder::new()
            .set_idx(IDX)
            .set_boh(BOH)
            .set_func(FUNC_SCOPE_SETTING)
            .set_cmd(SCOPE_TRIGGER_MODE)
            .set_val0(match trigger_mode {
                TriggerMode::Auto => SCOPE_VAL_TRIGGER_MODE_AUTO,
                TriggerMode::Normal => SCOPE_VAL_TRIGGER_MODE_NORMAL,
                TriggerMode::Single => SCOPE_VAL_TRIGGER_MODE_SINGLE,
            })
            .set_last(0)
            .build()
            .into();

        trace!(
            "setting trigger mode, trigger_mode={}",
            trigger_mode.my_to_string()
        );
        let result = self.write(&cmd);

        if result.is_ok() {
            debug!(
                "trigger mode set, trigger_mode={}",
                trigger_mode.my_to_string()
            );
            self.config.set_trigger_mode(trigger_mode);
            Ok(())
        } else {
            warn!(
                "setting trigger mode failed, error={}",
                result.as_ref().err().unwrap()
            );
            Err(result.err().unwrap().into())
        }
    }

    pub fn set_trigger_level_with_auto_adjustment(
        &mut self,
        trigger_level: f32,
    ) -> anyhow::Result<()> {
        if trigger_level.is_nan() || trigger_level.is_infinite() {
            panic!(
                "invalid value for trigger level, trigger_level={}",
                trigger_level
            );
        }

        let adjustment = self.config.get_trigger_level_adjustment();
        if adjustment.is_none() {
            bail!("trigger level adjustment missing");
        }
        let adjustment = adjustment.unwrap();
        if !adjustment.are_limits_sane() || adjustment.limits_are_zero() {
            bail!("adjustment are not sane for trigger_level");
        }

        let dev_trigger_level = {
            let mut dev_trigger_level = trigger_level - adjustment.lower;
            dev_trigger_level *= 200.0;
            dev_trigger_level /= adjustment.upper - adjustment.lower;
            dev_trigger_level
        };

        let cmd: RawCommand = HantekCommandBuilder::new()
            .set_idx(IDX)
            .set_boh(BOH)
            .set_func(FUNC_SCOPE_SETTING)
            .set_cmd(SCOPE_TRIGGER_LEVEL)
            .set_val0(dev_trigger_level as u8)
            .set_last(0)
            .build()
            .into();

        trace!(
            "setting trigger level, trigger_level={} (raw={})",
            trigger_level,
            dev_trigger_level as u8
        );
        let result = self.write(&cmd);

        if result.is_ok() {
            debug!(
                "trigger level set, trigger_level={} (raw={})",
                trigger_level, dev_trigger_level as u8
            );
            self.config.set_trigger_level(trigger_level);
            Ok(())
        } else {
            warn!(
                "setting time offset failed, error={}",
                result.as_ref().err().unwrap()
            );
            Err(result.err().unwrap().into())
        }
    }

    ///=================================================================== AWG

    pub fn set_awg_type(&mut self, awg_type: AwgType) -> anyhow::Result<()> {
        let cmd: RawCommand = HantekCommandBuilder::new()
            .set_idx(IDX)
            .set_boh(BOH)
            .set_func(FUNC_AWG_SETTING)
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
            .set_last(0)
            .build()
            .into();

        trace!("setting awg mode, mode={}", awg_type.my_to_string());
        let result = self.write(&cmd);

        if result.is_ok() {
            debug!("awg mode set, mode={}", awg_type.my_to_string());
            self.config.set_awg_type(awg_type);
            Ok(())
        } else {
            warn!(
                "failed to set awg mode, error={}",
                result.as_ref().err().unwrap()
            );
            Err(result.err().unwrap().into())
        }
    }

    pub fn set_awg_frequency(&mut self, frequency: f32) -> anyhow::Result<()> {
        // TODO sanitize frequency?

        let cmd: RawCommand = HantekCommandBuilder::new()
            .set_idx(IDX)
            .set_boh(BOH)
            .set_func(FUNC_AWG_SETTING)
            .set_cmd(AWG_FREQ)
            .set_val_u32(frequency as u32)
            .set_last(0)
            .build()
            .into();

        trace!("setting awg frequency, frequency={}", frequency);
        let result = self.write(&cmd);

        if result.is_ok() {
            debug!("awg frequency set, frequency={}", frequency);
            self.config.set_awg_frequency(frequency);
            Ok(())
        } else {
            warn!(
                "failed to set awg frequency, error={}",
                result.as_ref().err().unwrap()
            );
            Err(result.err().unwrap().into())
        }
    }

    pub fn set_awg_amplitude(&mut self, amplitude: f32) -> anyhow::Result<()> {
        // TODO sanitize amplitude?

        let raw = (amplitude.abs() * 1000.0) as u16;
        let sign = if amplitude.is_sign_negative() {
            1u16
        } else {
            0u16
        };
        let cmd: RawCommand = HantekCommandBuilder::new()
            .set_idx(IDX)
            .set_boh(BOH)
            .set_func(FUNC_AWG_SETTING)
            .set_cmd(AWG_AMPLITUDE)
            .set_val_u16(raw, sign)
            .set_last(0)
            .build()
            .into();

        trace!(
            "setting awg amplitude, amplitude={} (raw={}/{})",
            amplitude,
            raw,
            sign
        );
        let result = self.write(&cmd);

        if result.is_ok() {
            debug!(
                "awg amplitude set, amplitude={} (raw={}/{})",
                amplitude, raw, sign
            );
            self.config.set_awg_amplitude(amplitude);
            Ok(())
        } else {
            warn!(
                "failed to set awg amplitude, error={}",
                result.as_ref().err().unwrap()
            );
            Err(result.err().unwrap().into())
        }
    }

    pub fn set_awg_offset(&mut self, offset: f32) -> anyhow::Result<()> {
        // TODO sanitize offset?

        let raw = (offset.abs() * 1000.0) as u16;
        let sign = if offset.is_sign_negative() {
            1u16
        } else {
            0u16
        };
        let cmd: RawCommand = HantekCommandBuilder::new()
            .set_idx(IDX)
            .set_boh(BOH)
            .set_func(FUNC_AWG_SETTING)
            .set_cmd(AWG_OFFSET)
            .set_val_u16(raw, sign)
            .set_last(0)
            .build()
            .into();

        trace!(
            "setting awg offset, offset={} (raw={}/{})",
            offset,
            raw,
            sign
        );
        let result = self.write(&cmd);

        if result.is_ok() {
            debug!("awg offset set, offset={} (raw={}/{})", offset, raw, sign);
            self.config.set_awg_offset(offset);
            Ok(())
        } else {
            warn!(
                "failed to set awg offset, error={}",
                result.as_ref().err().unwrap()
            );
            Err(result.err().unwrap().into())
        }
    }

    pub fn set_awg_duty_square(&mut self, duty: f32) -> anyhow::Result<()> {
        // TODO sanitize duty?

        let raw = (duty * 100.0) as u16;
        let cmd: RawCommand = HantekCommandBuilder::new()
            .set_idx(IDX)
            .set_boh(BOH)
            .set_func(FUNC_AWG_SETTING)
            .set_cmd(AWG_SQUARE_DUTY)
            .set_val_u16(raw, 0)
            .set_last(0)
            .build()
            .into();

        trace!("setting awg square duty, duty={} (raw={})", duty, raw);
        let result = self.write(&cmd);

        if result.is_ok() {
            debug!("awg square duty set, duty={} (raw={})", duty, raw);
            self.config.set_awg_duty_square(duty);
            Ok(())
        } else {
            warn!(
                "failed to set awg square duty, error={}",
                result.as_ref().err().unwrap()
            );
            Err(result.err().unwrap().into())
        }
    }

    pub fn set_awg_duty_ramp(&mut self, duty: f32) -> anyhow::Result<()> {
        // TODO sanitize duty?

        let raw = (duty * 100.0) as u16;
        let cmd: RawCommand = HantekCommandBuilder::new()
            .set_idx(IDX)
            .set_boh(BOH)
            .set_func(FUNC_AWG_SETTING)
            .set_cmd(AWG_RAMP_DUTY)
            .set_val_u16(raw, 0)
            .set_last(0)
            .build()
            .into();

        trace!("setting awg ramp duty, duty={} (raw={})", duty, raw);
        let result = self.write(&cmd);

        if result.is_ok() {
            debug!("awg ramp duty set, duty={} (raw={})", duty, raw);
            self.config.set_awg_duty_ramp(duty);
            Ok(())
        } else {
            warn!(
                "failed to set awg ramp duty, error={}",
                result.as_ref().err().unwrap()
            );
            Err(result.err().unwrap().into())
        }
    }

    pub fn set_awg_duty_trap(&mut self, high: f32, low: f32, rise: f32) -> anyhow::Result<()> {
        // TODO sanitize high, low, rise?

        let raw_high = (high * 100.0) as u8;
        let raw_low = (low * 100.0) as u8;
        let raw_rise = (rise * 100.0) as u8;
        let cmd: RawCommand = HantekCommandBuilder::new()
            .set_idx(IDX)
            .set_boh(BOH)
            .set_func(FUNC_AWG_SETTING)
            .set_cmd(AWG_TRAP_DUTY)
            .set_val_u8(raw_rise, raw_high, raw_low, 0)
            .set_last(0)
            .build()
            .into();

        trace!(
            "setting awg trap duty, high={} low={} rise={} (raw={}/{}/{})",
            high,
            low,
            rise,
            raw_high,
            raw_low,
            raw_rise
        );
        let result = self.write(&cmd);

        if result.is_ok() {
            debug!(
                "awg trap duty set, high={} low={} rise={} (raw={}/{}/{})",
                high, low, rise, raw_high, raw_low, raw_rise
            );
            self.config.set_awg_duty_trap(high, low, rise);
            Ok(())
        } else {
            warn!(
                "failed to set awg trap duty, error={}",
                result.as_ref().err().unwrap()
            );
            Err(result.err().unwrap().into())
        }
    }

    pub fn awg_start(&mut self) -> anyhow::Result<()> {
        let cmd: RawCommand = HantekCommandBuilder::new()
            .set_idx(IDX)
            .set_boh(BOH)
            .set_func(FUNC_AWG_SETTING)
            .set_cmd(AWG_START_STOP)
            .set_val0(1)
            .set_last(0)
            .build()
            .into();

        trace!("setting awg to Start");
        let result = self.write(&cmd);

        if result.is_ok() {
            debug!("awg set to Start");
            self.config.awg_start();
            Ok(())
        } else {
            warn!(
                "failed to set awg to Start, error={}",
                result.as_ref().err().unwrap()
            );
            Err(result.err().unwrap().into())
        }
    }

    pub fn awg_stop(&mut self) -> anyhow::Result<()> {
        let cmd: RawCommand = HantekCommandBuilder::new()
            .set_idx(IDX)
            .set_boh(BOH)
            .set_func(FUNC_AWG_SETTING)
            .set_cmd(AWG_START_STOP)
            .set_val0(0)
            .set_last(0)
            .build()
            .into();

        trace!("setting awg to Stop");
        let result = self.write(&cmd);

        if result.is_ok() {
            debug!("awg set to Stop");
            self.config.stop();
            Ok(())
        } else {
            warn!(
                "failed to set awg to Stop, error={}",
                result.as_ref().err().unwrap()
            );
            Err(result.err().unwrap().into())
        }
    }

    ///=============================================================== INTERNAL

    fn write(&self, cmd: &RawCommand) -> libusb::Result<usize> {
        self.usb
            .handle
            .write_bulk(2, cmd, self.config.get_timeout())
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
