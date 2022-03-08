use std::time::Duration;
use anyhow::bail;

use hanteker_lib::models::hantek2d42::Hantek2D42;
use pretty_env_logger::formatted_builder;

use crate::cli::{cli_parse, Commands};
use crate::handler::{handle_awg, handle_device, handle_print, handle_scope, handle_shell};

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

    builder.try_init().unwrap();
}

fn main() -> anyhow::Result<()> {
    let cli = cli_parse();

    init_log(cli.silent, cli.verbose);

    if let Commands::Shell(sub) = &cli.sub_commands {
        handle_shell(&cli, sub);
    } else {
        let context = libusb::Context::new()?;
        let mut hantek = Hantek2D42::open(&context, Duration::from_millis(cli.timeout))?;
        match &cli.sub_commands {
            Commands::Awg(sub) => handle_awg(&cli, sub, &mut hantek)?,
            Commands::Device(sub) => handle_device(&cli, sub, &mut hantek)?,
            Commands::Scope(sub) => handle_scope(&cli, sub, &mut hantek)?,
            Commands::Print(_) => handle_print(&cli, &mut hantek)?,
            _ => unreachable!(),
        }
    }

    Ok(())
}
