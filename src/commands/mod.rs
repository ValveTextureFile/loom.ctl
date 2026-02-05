pub mod run;
pub mod list;
pub mod profile;

use crate::cli::*;
use crate::interface::*;

pub fn dispatch(cmd: cli::Command, ctx: &mut ctx::Context) -> anyhow::Result<()> {
    match cmd {
        Command::Run(a) => run::handle(a, ctx),
        Command::List(a) => list::handle(a, ctx),
        Command::Doctor(a) => doctor::handle(a, ctx),
        Command::PrintCmd(a) => printcmd
    }
}