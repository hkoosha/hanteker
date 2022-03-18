#![cfg_attr(not(debug_assertions), deny(warnings))]

use std::time::Duration;

use pretty_env_logger::formatted_builder;

use hanteker_lib::models::hantek2d42::Hantek2D42;

use crate::cli::{Cli, cli_parse, Commands};
use crate::handler::{
    handle_awg, handle_capture, handle_channel, handle_device, handle_print, handle_scope,
    handle_shell,
};

mod cli;
mod handler;

fn init_log(silent: usize, verbose: usize) {
    let filter = match (silent, verbose) {
        (1, _) => "WARN",
        (2, _) => "ERROR",
        (s, _) if s > 2 => "",
        (_, 1) => "DEBUG",
        (_, v) if v >= 2 => "TRACE",
        _ => "INFO",
    };

    let mut builder = formatted_builder();
    builder.parse_filters(filter);
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
        hantek.usb.claim()?;
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
