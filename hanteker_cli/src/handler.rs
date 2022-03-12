use std::io::Write;
use std::{env, io};

use anyhow::bail;
use clap_complete::generate;
use log::warn;

use hanteker_lib::models::hantek2d42::Hantek2D42;

use crate::cli::{cli_command, AwgCli, Cli, DeviceCli, ScopeCli, ShellCli};

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

    if cli.mode.is_some() {
        hantek.set_device_function(cli.mode.as_ref().unwrap().clone())?;
    }

    Ok(())
}

pub(crate) fn handle_scope(
    _parent: &Cli,
    cli: &ScopeCli,
    hantek: &mut Hantek2D42,
) -> anyhow::Result<()> {
    if cli.enable_channel && cli.disable_channel {
        bail!("must not specify disable-channel and enable-channel at the same time.");
    }
    if cli.enable_bandwidth_limit && cli.disable_bandwidth_limit {
        bail!(
            "must not specify disable-bandwidth-limit and enable-bandwidth-limit at the same time."
        );
    }
    if cli.channel.is_empty() {
        bail!("at least one channel must be specified");
    }

    if cli.enable_channel {
        for channel in &cli.channel {
            hantek.enable_channel(*channel)?;
        }
    }
    if cli.disable_channel {
        for channel in &cli.channel {
            hantek.disable_channel(*channel)?;
        }
    }

    if cli.enable_bandwidth_limit {
        for channel in &cli.channel {
            hantek.channel_enable_bandwidth_limit(*channel)?;
        }
    }
    if cli.disable_bandwidth_limit {
        for channel in &cli.channel {
            hantek.channel_disable_bandwidth_limit(*channel)?;
        }
    }

    if cli.probe.is_some() {
        for channel in &cli.channel {
            hantek.set_channel_probe(*channel, cli.probe.as_ref().unwrap().clone())?;
        }
    }

    if cli.scale.is_some() {
        for channel in &cli.channel {
            hantek.set_channel_scale(*channel, cli.scale.as_ref().unwrap().clone())?;
        }
    }

    if cli.offset.is_some() {
        for channel in &cli.channel {
            let result =
                hantek.set_channel_offset_with_auto_adjustment(*channel, cli.offset.unwrap());
            if result.is_err() {
                result?;
            }
        }
    }

    if cli.time_scale.is_some() {
        hantek.set_time_scale(cli.time_scale.as_ref().unwrap().clone())?;
    }
    if cli.time_offset.is_some() {
        hantek.set_time_offset_with_auto_adjustment(cli.time_offset.unwrap())?;
    }

    if cli.trigger_source.is_some() {
        hantek.set_trigger_source(cli.trigger_source.unwrap())?;
    }
    if cli.trigger_level.is_some() {
        hantek.set_trigger_level_with_auto_adjustment(cli.trigger_level.unwrap())?;
    }
    if cli.trigger_slope.is_some() {
        hantek.set_trigger_slope(cli.trigger_slope.as_ref().unwrap().clone())?;
    }
    if cli.trigger_mode.is_some() {
        hantek.set_trigger_mode(cli.trigger_mode.as_ref().unwrap().clone())?;
    }

    if cli.capture {
        let out = std::io::stdout();
        let mut lock = out.lock();
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

    Ok(())
}

pub(crate) fn handle_awg(
    parent: &Cli,
    cli: &AwgCli,
    hantek: &mut Hantek2D42,
) -> anyhow::Result<()> {
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
