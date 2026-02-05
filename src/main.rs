mod interface;
mod commands;
mod profilet;

use clap::Parser;
use interface::cli;
use interface::ctx;
// loomctl - doom launcher w/ mod support

fn main() -> anyhow::Result<()> {
    let cli = cli::Cli::parse();
    interface::logging::initlogging(cli.verbose);

    let mut ctx = ctx::Context::new(cli.verbose)?;
    commands::dispatch(cli.cmd, &mut ctx)?;
    Ok(())
}
