use std::str::FromStr;

use clap::{Args, CommandFactory, Parser, Subcommand};
use clap_complete::Shell;
use hanteker_lib::device::cfg::{AwgType, Coupling, DeviceFunction, Probe, Scale, TimeScale, TriggerMode, TriggerSlope};

/// A cli tool to interface with Hantek oscilloscope
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None, trailing_var_arg = true)]
pub(crate) struct Cli {
    #[clap(subcommand)]
    pub sub_commands: Commands,

    /// USB timeout in milliseconds
    #[clap(long, default_value_t = 1000)]
    pub timeout: u64,

    /// Specify multiple time to increase log level from info
    #[clap(short, long, parse(from_occurrences))]
    pub verbose: usize,

    /// Specify multiple times to decrease log level from info, takes precedence over --verbose
    #[clap(short, long, parse(from_occurrences))]
    pub silent: usize,

    #[clap(long)]
    /// Suppress warnings about UI quirks
    pub no_quirks: bool,
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
    pub start: bool,

    #[clap(long)]
    pub stop: bool,

    #[clap(short, long, arg_enum)]
    pub mode: Option<DeviceFunction>,
}

#[derive(Args, Debug)]
pub(crate) struct ScopeCli {
    #[clap(short, long, validator = channel_no_validator)]
    pub channel: Vec<usize>,

    #[clap(long)]
    pub enable_channel: bool,

    #[clap(long)]
    pub disable_channel: bool,

    #[clap(long, arg_enum)]
    pub coupling: Option<Coupling>,

    #[clap(long, arg_enum)]
    pub probe: Option<Probe>,

    #[clap(long, arg_enum)]
    pub scale: Option<Scale>,

    #[clap(long)]
    pub offset: Option<f32>,

    #[clap(long)]
    pub enable_bandwidth_limit: bool,

    #[clap(long)]
    pub disable_bandwidth_limit: bool,

    #[clap(long, arg_enum)]
    pub time_scale: Option<TimeScale>,

    #[clap(long)]
    pub time_offset: Option<f32>,

    // TODO properly name arg in clap.
    /// Takes channel no
    #[clap(long)]
    pub trigger_source: Option<usize>,

    #[clap(long, arg_enum)]
    pub trigger_slope: Option<TriggerSlope>,

    #[clap(long, arg_enum)]
    pub trigger_mode: Option<TriggerMode>,

    #[clap(long)]
    pub trigger_level: Option<f32>,

    #[clap(short = 'k', long)]
    pub capture: bool,

    #[clap(long, default_value_t = 1000)]
    pub capture_chunk: usize,
}

#[derive(Args, Debug)]
pub(crate) struct PrintCli {}

#[derive(Args, Debug)]
pub(crate) struct ShellCli {
    #[clap(short, long)]
    pub name_override: Option<String>,

    #[clap(short, long, arg_enum)]
    pub shell: Shell,
}

#[derive(Args, Debug)]
pub(crate) struct AwgCli {
    #[clap(short, long, arg_enum)]
    pub r#type: Option<AwgType>,

    #[clap(short, long)]
    pub frequency: Option<f32>,

    #[clap(short, long)]
    pub amplitude: Option<f32>,

    #[clap(short, long)]
    pub offset: Option<f32>,

    #[clap(long)]
    pub duty_square: Option<f32>,

    #[clap(long)]
    pub duty_ramp: Option<f32>,

    #[clap(long)]
    pub duty_trap_rise: Option<f32>,

    #[clap(long)]
    pub duty_trap_high: Option<f32>,

    #[clap(long)]
    pub duty_trap_low: Option<f32>,

    #[clap(long)]
    pub stop: bool,

    #[clap(long)]
    pub start: bool,
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
