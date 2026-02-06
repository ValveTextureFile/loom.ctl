use crate::{ctx::Context, interface::args};

pub mod new;
pub mod delete;
pub mod set;
pub mod find;

pub mod add_mod;
pub mod remove_mod;

pub fn handle(a: args::ProfileArgs, ctx: &Context) -> anyhow::Result<()> {
    match a.cmd {
        args::ProfileCmd::New(x) => new::handle(x, ctx),
        args::ProfileCmd::Delete(x) => delete::handle(x, ctx),
        args::ProfileCmd::Set(x) => set::handle(x, ctx),
        args::ProfileCmd::AddMod(x) => add_mod::handle(x, ctx),
        args::ProfileCmd::RemoveMod(x) => remove_mod::handle(x, ctx),
        args::ProfileCmd::Find(x) => find::handle(x, ctx)
    }
}
