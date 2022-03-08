use std::str::FromStr;

use clap::{Args, CommandFactory, Parser, Subcommand};
use clap_complete::Shell;
use hanteker_lib::device::cfg::{AwgType, Coupling, DeviceFunction, Probe, Scale, TimeScale, TriggerMode, TriggerSlope};

/// A cli tool to interface with Hantek oscilloscope
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None, trailing_var_arg = true)]
pub(crate) struct Cli {
    #[clap(subcommand)]
    pub(crate) sub_commands: Commands,

    /// USB timeout in milliseconds
    #[clap(long, default_value_t = 1000)]
    pub(crate) timeout: u64,

    /// Specify multiple time to increase log level from info
    #[clap(short, long, parse(from_occurrences))]
    pub(crate) verbose: usize,

    /// Specify multiple times to decrease log level from info, takes precedence over --verbose
    #[clap(short, long, parse(from_occurrences))]
    pub(crate) silent: usize,

    #[clap(long)]
    /// Suppress warnings about UI quirks
    pub(crate) no_quirks: bool,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Commands {
    /// Operate on the device itself
    Device(DeviceCli),

    /// Operate on a scope function of the device
    Scope(ScopeCli),

    /// Operate on AWG function of the device
    Awg(AwgCli),

    /// Print device info
    Print(PrintCli),

    /// Generate shell completion script.
    Shell(ShellCli),
}

#[derive(Args, Debug)]
pub(crate) struct DeviceCli {
    #[clap(long)]
    pub(crate) start: bool,

    #[clap(long)]
    pub(crate) stop: bool,

    #[clap(short, long, arg_enum)]
    pub(crate) mode: Option<DeviceFunction>,
}

#[derive(Args, Debug)]
pub(crate) struct ScopeCli {
    #[clap(short, long, validator = channel_no_validator)]
    pub(crate) channel: Vec<usize>,

    #[clap(long)]
    pub(crate) enable_channel: bool,

    #[clap(long)]
    pub(crate) disable_channel: bool,

    #[clap(long, arg_enum)]
    pub(crate) coupling: Option<Coupling>,

    #[clap(long, arg_enum)]
    pub(crate) probe: Option<Probe>,

    #[clap(long, arg_enum)]
    pub(crate) scale: Option<Scale>,

    #[clap(long)]
    pub(crate) offset: Option<f32>,

    #[clap(long)]
    pub(crate) enable_bandwidth_limit: bool,

    #[clap(long)]
    pub(crate) disable_bandwidth_limit: bool,

    #[clap(long, arg_enum)]
    pub(crate) time_scale: Option<TimeScale>,

    #[clap(long)]
    pub(crate) time_offset: Option<f32>,

    // TODO properly name arg in clap.
    /// Takes channel no
    #[clap(long)]
    pub(crate) trigger_source: Option<usize>,

    #[clap(long, arg_enum)]
    pub(crate) trigger_slope: Option<TriggerSlope>,

    #[clap(long, arg_enum)]
    pub(crate) trigger_mode: Option<TriggerMode>,

    #[clap(long)]
    pub(crate) trigger_level: Option<f32>,

    #[clap(short = 'k', long)]
    pub(crate) capture: bool,

    #[clap(long, default_value_t = 1000)]
    pub(crate) capture_chunk: usize,
}

#[derive(Args, Debug)]
pub(crate) struct PrintCli {}

#[derive(Args, Debug)]
pub(crate) struct ShellCli {
    #[clap(short, long)]
    pub(crate) name_override: Option<String>,

    #[clap(short, long, arg_enum)]
    pub(crate) shell: Shell,
}

#[derive(Args, Debug)]
pub(crate) struct AwgCli {
    #[clap(short, long, arg_enum)]
    pub(crate) r#type: Option<AwgType>,

    #[clap(short, long)]
    pub(crate) frequency: Option<f32>,

    #[clap(short, long)]
    pub(crate) amplitude: Option<f32>,

    #[clap(short, long)]
    pub(crate) offset: Option<f32>,

    #[clap(long)]
    pub(crate) duty_square: Option<f32>,

    #[clap(long)]
    pub(crate) duty_ramp: Option<f32>,

    #[clap(long)]
    pub(crate) duty_trap_rise: Option<f32>,

    #[clap(long)]
    pub(crate) duty_trap_high: Option<f32>,

    #[clap(long)]
    pub(crate) duty_trap_low: Option<f32>,

    #[clap(long)]
    pub(crate) stop: bool,

    #[clap(long)]
    pub(crate) start: bool,
}


pub(crate) fn cli_command() -> clap::Command<'static> {
    Cli::command()
}

pub(crate) fn cli_parse() -> Cli {
    Cli::parse()
}

fn channel_no_validator(s: &str) -> Result<(), String> {
    let channel = usize::from_str(s);
    if channel.is_err() {
        return Err(format!(
            "Invalid value for channel, expecting 1 or 2, got: {}",
            s
        ));
    }
    let channel = channel.unwrap();
    if channel != 1 && channel != 2 {
        return Err(format!("Invalid channel, expecting 1 or 2, got: {}", s));
    }
    Ok(())
}
