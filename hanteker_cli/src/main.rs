use std::time::Duration;

use pretty_env_logger::formatted_builder;

use hanteker_lib::models::hantek2d42::Hantek2D42;

use crate::cli::{cli_parse, Cli, Commands};
use crate::handler::{
    handle_awg, handle_capture, handle_channel, handle_device, handle_print, handle_scope,
    handle_shell,
};

mod cli;
mod handler;

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

    builder.init();
}

fn main() -> anyhow::Result<()> {
    let cli = cli_parse();

    init_log(cli.silent, cli.verbose);

    if let Commands::Shell(sub) = &cli.sub_commands {
        handle_shell(&cli, sub);
    } else {
        let context = libusb::Context::new()?;
        let mut hantek = Hantek2D42::open(&context, Duration::from_millis(cli.timeout))?;
        let cmd_result = handle_usb_command(&cli, &mut hantek);
        let release_result = hantek.usb.release();
        cmd_result?;
        release_result?;
    }

    Ok(())
}

fn handle_usb_command(cli: &Cli, hantek: &mut Hantek2D42) -> anyhow::Result<()> {
    match &cli.sub_commands {
        Commands::Awg(sub) => handle_awg(cli, sub, hantek)?,
        Commands::Device(sub) => handle_device(cli, sub, hantek)?,
        Commands::Scope(sub) => handle_scope(cli, sub, hantek)?,
        Commands::Print(_) => handle_print(cli, hantek)?,
        Commands::Channel(sub) => handle_channel(cli, sub, hantek)?,
        Commands::Capture(sub) => handle_capture(cli, sub, hantek)?,
        Commands::Shell(_) => unreachable!(),
    }

    Ok(())
}
