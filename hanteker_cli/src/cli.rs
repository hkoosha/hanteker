use clap::{Args, CommandFactory, Parser, Subcommand};
use clap_complete::Shell;

use hanteker_lib::device::cfg::{
    AwgType, Coupling, DeviceFunction, Probe, Scale, TimeScale, TriggerMode, TriggerSlope,
};

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

    /// Operate on a scope channel
    Channel(ChannelCli),

    /// Capture scope channels
    Capture(CaptureCli),

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
    /// Set device to scope mode before running any other command
    #[clap(short, long)]
    pub(crate) force_mode: bool,

    #[clap(long, arg_enum)]
    pub(crate) time_scale: Option<TimeScale>,

    #[clap(long)]
    pub(crate) time_offset: Option<f32>,

    #[clap(long, value_name = "CHANNEL")]
    pub(crate) trigger_source: Option<usize>,

    #[clap(long, arg_enum)]
    pub(crate) trigger_slope: Option<TriggerSlope>,

    #[clap(long, arg_enum)]
    pub(crate) trigger_mode: Option<TriggerMode>,

    #[clap(long)]
    pub(crate) trigger_level: Option<f32>,
}

#[derive(Args, Debug)]
pub(crate) struct ChannelCli {
    /// Set device to scope mode before running any other command
    #[clap(short, long)]
    pub(crate) force_mode: bool,

    #[clap(short, long, possible_values = ["1", "2"])]
    pub(crate) channel: usize,

    #[clap(long, group = "channel-status")]
    pub(crate) enable: bool,

    #[clap(long, group = "channel-status")]
    pub(crate) disable: bool,

    #[clap(long, arg_enum)]
    pub(crate) coupling: Option<Coupling>,

    #[clap(long, arg_enum)]
    pub(crate) probe: Option<Probe>,

    #[clap(long, arg_enum)]
    pub(crate) scale: Option<Scale>,

    #[clap(long)]
    pub(crate) offset: Option<f32>,

    #[clap(long, group = "bandwidth-limit-status")]
    pub(crate) enable_bandwidth_limit: bool,

    #[clap(long, group = "bandwidth-limit-status")]
    pub(crate) disable_bandwidth_limit: bool,
}

#[derive(Args, Debug)]
pub(crate) struct CaptureCli {
    /// Set device to scope mode before running any other command
    #[clap(short, long)]
    pub(crate) force_mode: bool,

    #[clap(short, long, possible_values = ["1", "2"])]
    pub(crate) channel: Vec<usize>,

    #[clap(long, default_value_t = 1000)]
    pub(crate) capture_chunk: usize,

    /// Defaults to infinity
    #[clap(short, long)]
    pub(crate) num_captures: Option<usize>,
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
    /// Set device to AWG mode before running any other command
    #[clap(short, long)]
    pub(crate) force_mode: bool,

    #[clap(short, long, arg_enum)]
    pub(crate) r#type: Option<AwgType>,

    #[clap(long)]
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
