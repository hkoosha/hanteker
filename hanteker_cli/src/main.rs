use std::str::FromStr;
use std::time::Duration;
use std::{env, io};

use anyhow::bail;
use clap::{Args, CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};
use log::warn;
use pretty_env_logger::formatted_builder;

use hanteker_lib::device::cfg::{
    AwgType, Coupling, DeviceFunction, Probe, Scale, TimeScale, TriggerMode, TriggerSlope,
};
use hanteker_lib::models::hantek2d42::Hantek2D42;

/// A cli tool to interface with Hantek oscilloscope
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None, trailing_var_arg = true)]
struct Cli {
    #[clap(subcommand)]
    sub_commands: Commands,

    #[clap(long = "generate", arg_enum)]
    generator: Option<Shell>,

    /// USB timeout in milliseconds
    #[clap(long, default_value_t = 1000)]
    timeout: u64,

    /// Specify multiple time to increase log level from info
    #[clap(short, long, parse(from_occurrences))]
    verbose: usize,

    /// Specify multiple times to decrease log level from info, takes precedence over --verbose
    #[clap(short, long, parse(from_occurrences))]
    silent: usize,

    #[clap(long)]
    /// Suppress warnings about UI quirks
    no_quirks: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
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
struct DeviceCli {
    #[clap(long)]
    start: bool,

    #[clap(long)]
    stop: bool,

    #[clap(short, long, arg_enum)]
    mode: Option<DeviceFunction>,
}

#[derive(Args, Debug)]
struct ScopeCli {
    #[clap(short, long, validator = channel_no_validator)]
    channel: Vec<usize>,

    #[clap(long)]
    enable_channel: bool,

    #[clap(long)]
    disable_channel: bool,

    #[clap(long, arg_enum)]
    coupling: Option<Coupling>,

    #[clap(long, arg_enum)]
    probe: Option<Probe>,

    #[clap(long, arg_enum)]
    scale: Option<Scale>,

    #[clap(long)]
    offset: Option<f32>,

    #[clap(long)]
    enable_bandwidth_limit: bool,

    #[clap(long)]
    disable_bandwidth_limit: bool,

    #[clap(long, arg_enum)]
    time_scale: Option<TimeScale>,

    #[clap(long)]
    time_offset: Option<f32>,

    // TODO properly name arg in clap.
    /// Takes channel no
    #[clap(long)]
    trigger_source: Option<usize>,

    #[clap(long, arg_enum)]
    trigger_slope: Option<TriggerSlope>,

    #[clap(long, arg_enum)]
    trigger_mode: Option<TriggerMode>,

    #[clap(long)]
    trigger_level: Option<f32>,

    #[clap(short = 'k', long)]
    capture: bool,

    #[clap(long, default_value_t = 1000)]
    capture_chunk: usize,
}

#[derive(Args, Debug)]
struct PrintCli {}

#[derive(Args, Debug)]
struct ShellCli {
    #[clap(short, long)]
    name_override: Option<String>,
}

#[derive(Args, Debug)]
struct AwgCli {
    #[clap(short, long, arg_enum)]
    r#type: Option<AwgType>,

    #[clap(short, long)]
    frequency: Option<f32>,

    #[clap(short, long)]
    amplitude: Option<f32>,

    #[clap(short, long)]
    offset: Option<f32>,

    #[clap(long)]
    duty_square: Option<f32>,

    #[clap(long)]
    duty_ramp: Option<f32>,

    #[clap(long)]
    duty_trap_rise: Option<f32>,

    #[clap(long)]
    duty_trap_high: Option<f32>,

    #[clap(long)]
    duty_trap_low: Option<f32>,

    #[clap(long)]
    stop: bool,

    #[clap(long)]
    start: bool,
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

fn init_log(silent: usize, verbose: usize) {
    let mut builder = formatted_builder();

    if silent == 1 {
        builder.parse_filters("WARN");
    } else if silent == 2 {
        builder.parse_filters("ERROR");
    } else if silent > 2 {
        builder.parse_filters("");
    } else if verbose == 1 {
        builder.parse_filters("DEBUG");
    } else if verbose >= 2 {
        builder.parse_filters("TRACE");
    } else {
        builder.parse_filters("INFO");
    }

    builder.try_init().unwrap();
}

fn main() -> anyhow::Result<()> {
    let cli: Cli = Cli::parse();

    init_log(cli.silent, cli.verbose);

    match &cli.sub_commands {
        Commands::Awg(sub) => {
            let context = libusb::Context::new()?;
            let mut hantek = Hantek2D42::open(&context, Duration::from_millis(cli.timeout))?;
            handle_awg(&cli, sub, &mut hantek)?
        }
        Commands::Device(sub) => {
            let context = libusb::Context::new()?;
            let mut hantek = Hantek2D42::open(&context, Duration::from_millis(cli.timeout))?;
            handle_device(&cli, sub, &mut hantek)?
        }
        Commands::Scope(sub) => {
            let context = libusb::Context::new()?;
            let mut hantek = Hantek2D42::open(&context, Duration::from_millis(cli.timeout))?;
            handle_scope(&cli, sub, &mut hantek)?
        }
        Commands::Print(_) => {
            let context = libusb::Context::new()?;
            let mut hantek = Hantek2D42::open(&context, Duration::from_millis(cli.timeout))?;
            handle_print(&cli, &mut hantek)?
        }
        Commands::Shell(sub) => {
            handle_shell(cli.generator, sub);
        }
    }

    Ok(())
}

fn handle_shell(shell: Option<Shell>, s: &ShellCli) {
    let mut cmd = Cli::command();
    let name = match &s.name_override {
        Some(name) => name.clone(),
        None => env::args().into_iter().next().unwrap(),
    };
    generate(
        shell.expect("shell type not specified"),
        &mut cmd,
        name,
        &mut io::stdout(),
    );
}

fn handle_print(_parent: &Cli, hantek: &mut Hantek2D42) -> anyhow::Result<()> {
    println!("{}", hantek.usb.pretty_printed_device_info());
    Ok(())
}

fn handle_device(_parent: &Cli, cli: &DeviceCli, hantek: &mut Hantek2D42) -> anyhow::Result<()> {
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

fn handle_scope(_parent: &Cli, cli: &ScopeCli, hantek: &mut Hantek2D42) -> anyhow::Result<()> {
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
            hantek.set_channel_offset_with_auto_adjustment(*channel, cli.offset.unwrap())?;
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
        let lock = out.lock();
        hantek.capture_inf(&cli.channel, cli.capture_chunk, lock);
    }

    Ok(())
}

fn handle_awg(parent: &Cli, cli: &AwgCli, hantek: &mut Hantek2D42) -> anyhow::Result<()> {
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
