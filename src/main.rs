mod interface;
mod commands;
mod profilet;

use clap::Parser;
use interface::cli;
use interface::ctx;
// loomctl - doom launcher w/ mod support

fn main() -> anyhow::Result<()> {
    let cli = cli::Cli::parse();
    let verbose;
    if cli.verbose < 1{
        verbose = 4;    
    } else {
        verbose = cli.verbose;
    }

    interface::logging::initlogging(verbose);

    let mut ctx = ctx::Context::new(verbose)?;
    commands::dispatch(cli.cmd, &mut ctx)?;
    Ok(())
}
