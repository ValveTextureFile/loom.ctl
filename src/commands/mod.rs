pub mod run;
pub mod list;


use crate::cli::*;
use crate::interface::*;

pub fn dispatch(cmd: cli::Command, ctx: &mut ctx::Context) -> anyhow::Result<()> {
    match cmd {
        Command::Run(a) => run::handle(a, ctx),
        Command::List(a) => list::handle(a, ctx)
    }
}