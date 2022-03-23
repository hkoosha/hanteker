use std::fmt::Display;
use std::io::Write;
use std::{env, io};

use anyhow::bail;
use clap_complete::generate;
use hanteker_lib::device::cfg::DeviceFunction;
use hanteker_lib::models::hantek2d42::Hantek2D42;
use log::{error, warn};

use crate::cli::{cli_command, AwgCli, CaptureCli, ChannelCli, Cli, DeviceCli, ScopeCli, ShellCli};

pub(crate) fn handle_shell(_parent: &Cli, s: &ShellCli) {
    let name = match &s.name_override {
        Some(name) => name.clone(),
        None => env::args().into_iter().next().unwrap(),
    };
    generate(s.shell, &mut cli_command(), name, &mut io::stdout());
}

pub(crate) fn handle_print(_parent: &Cli, hantek: &mut Hantek2D42) -> anyhow::Result<()> {
    println!("{}", hantek.usb.pretty_printed_device_info());
    Ok(())
}

pub(crate) fn handle_device(
    _parent: &Cli,
    cli: &DeviceCli,
    hantek: &mut Hantek2D42,
) -> anyhow::Result<()> {
    if cli.start && cli.stop {
        bail!("must not specify start and stop at the same time.");
    }

    if cli.start {
        hantek.start()?;
    }
    if cli.stop {
        hantek.stop()?;
    }

    if let Some(mode) = &cli.mode {
        hantek.set_device_function(mode.clone())?;
    }

    Ok(())
}

pub(crate) fn handle_scope(
    _parent: &Cli,
    cli: &ScopeCli,
    hantek: &mut Hantek2D42,
) -> anyhow::Result<()> {
    if cli.force_mode {
        hantek.set_device_function(DeviceFunction::Scope)?;
    }

    if let Some(time_scale) = &cli.time_scale {
        hantek.set_time_scale(time_scale.clone())?;
    }
    if let Some(time_offset) = &cli.time_offset {
        hantek.set_time_offset_with_auto_adjustment(*time_offset)?;
    }

    if let Some(trigger_source) = &cli.trigger_source {
        hantek.set_trigger_source(*trigger_source)?;
    }
    if let Some(trigger_level) = &cli.trigger_level {
        hantek.set_trigger_level_with_auto_adjustment(*trigger_level)?;
    }
    if let Some(trigger_slope) = &cli.trigger_slope {
        hantek.set_trigger_slope(trigger_slope.clone())?;
    }
    if let Some(trigger_mode) = &cli.trigger_mode {
        hantek.set_trigger_mode(trigger_mode.clone())?;
    }

    Ok(())
}

pub(crate) fn handle_channel(
    _parent: &Cli,
    cli: &ChannelCli,
    hantek: &mut Hantek2D42,
) -> anyhow::Result<()> {
    if cli.force_mode {
        hantek.set_device_function(DeviceFunction::Scope)?;
    }

    if cli.enable {
        hantek.enable_channel(cli.channel)?;
    }
    if cli.disable {
        hantek.disable_channel(cli.channel)?;
    }

    if cli.enable_bandwidth_limit {
        hantek.channel_enable_bandwidth_limit(cli.channel)?;
    }
    if cli.disable_bandwidth_limit {
        hantek.channel_disable_bandwidth_limit(cli.channel)?;
    }

    if let Some(probe) = &cli.probe {
        hantek.set_channel_probe(cli.channel, probe.clone())?;
    }

    if let Some(scale) = &cli.scale {
        hantek.set_channel_scale(cli.channel, scale.clone())?;
    }

    if let Some(offset) = &cli.offset {
        hantek.set_channel_offset_with_auto_adjustment(cli.channel, *offset)?;
    }

    Ok(())
}

pub(crate) fn handle_capture(
    _parent: &Cli,
    cli: &CaptureCli,
    hantek: &mut Hantek2D42,
) -> anyhow::Result<()> {
    if cli.force_mode {
        hantek.set_device_function(DeviceFunction::Scope)?;
    }

    let out = std::io::stdout();
    let mut lock = out.lock();

    match cli.num_captures {
        None => {
            loop {
                let captured = hantek
                    .capture(&cli.channel, cli.capture_chunk)
                    .expect("capture failed");
                if lock.write_all(&captured).is_err() || lock.flush().is_err() {
                    // Probably stream closed.
                    std::process::exit(0);
                }
            }
        }
        Some(num) => {
            for _ in 0..num {
                let captured = hantek.capture(&cli.channel, cli.capture_chunk);

                if let Err(e) = captured {
                    // Cast to make CLion happy.
                    error!("error: {}", &e as &dyn Display);
                    std::process::exit(1);
                }

                let captured = captured.unwrap();
                if lock.write_all(&captured).is_err() || lock.flush().is_err() {
                    // Probably stream closed.
                    std::process::exit(0);
                }
            }
            Ok(())
        }
    }
}

pub(crate) fn handle_awg(
    parent: &Cli,
    cli: &AwgCli,
    hantek: &mut Hantek2D42,
) -> anyhow::Result<()> {
    if cli.force_mode {
        hantek.set_device_function(DeviceFunction::AWG)?;
    }

    if (cli.duty_trap_high.is_some() || cli.duty_trap_low.is_some() || cli.duty_trap_rise.is_some())
        && (cli.duty_trap_high.is_none()
            || cli.duty_trap_rise.is_none()
            || cli.duty_trap_low.is_none())
    {
        bail!("When specifying duty for trap, all three duties must be specified at the same time: high, low and rise.");
    }

    if cli.start && cli.stop {
        bail!("must not specify awg start and stop at the same time.");
    }

    if cli.r#type.is_some() {
        hantek.set_awg_type(cli.r#type.as_ref().unwrap().clone())?;
    }

    if cli.frequency.is_some() {
        hantek.set_awg_frequency(cli.frequency.unwrap())?;
    }

    if cli.amplitude.is_some() {
        hantek.set_awg_amplitude(cli.amplitude.unwrap())?;
    }

    if cli.offset.is_some() {
        hantek.set_awg_offset(cli.offset.unwrap())?;
        if !parent.no_quirks {
            // Had me scratching my head for a while wondering why...
            warn!(
                "The offset in the UI will not be updated properly, but it is set. \
                   This is a bug in the device firmware."
            );
        }
    }

    if cli.duty_square.is_some() {
        hantek.set_awg_duty_square(cli.duty_square.unwrap())?;
    }

    if cli.duty_ramp.is_some() {
        hantek.set_awg_duty_ramp(cli.duty_ramp.unwrap())?;
    }

    if cli.duty_trap_rise.is_some() {
        hantek.set_awg_duty_trap(
            cli.duty_trap_high.unwrap(),
            cli.duty_trap_low.unwrap(),
            cli.duty_trap_rise.unwrap(),
        )?;
    }

    if cli.start {
        hantek.awg_start()?;
        if !parent.no_quirks {
            warn!(
                "The running status in the UI will not be updated properly, but it is set. \
               This is a bug in the device firmware."
            );
        }
    }
    if cli.stop {
        hantek.awg_stop()?;
        if !parent.no_quirks {
            warn!(
                "The running status in the UI will not be updated properly, but it is set. \
               This is a bug in the device firmware."
            );
        }
    }

    Ok(())
}
