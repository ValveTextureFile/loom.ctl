use anyhow::Ok;
use tracing::{info, warn, debug};
use crate::{ctx::Context, interface::args::{ListArgs, ListArgsWhat}};

pub fn handle(a: ListArgs, ctx: &mut Context) -> anyhow::Result<()> {
    match a.what {
        ListArgsWhat::Engines => ls_engines(),
        ListArgsWhat::Iwads => ls_iwads(),
        ListArgsWhat::Mods => ls_mods(),
        ListArgsWhat::Profiles => ls_profiles(),
    }

    Ok(())
}

fn ls_engines() {
    debug!("[list] engines");
}

fn ls_iwads() {

}

fn ls_mods() {

}

fn ls_profiles() {

}