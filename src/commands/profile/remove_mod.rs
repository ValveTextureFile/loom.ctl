use tracing::{debug, info, warn};

use crate::{
    interface::{args::ProfileRemoveMod, ctx::Context},
    profilet::{Profile, path::profile_path},
};

pub fn handle(a: ProfileRemoveMod, ctx: &Context) -> anyhow::Result<()> {
    debug!(profile = %a.name, target = %a.target, "profile::remove_mod");

    let path = profile_path(ctx, &a.name);
    let mut p = Profile::load(&path)?;

    if let Ok(idx1) = a.target.parse::<usize>() {
        if idx1 == 0 || idx1 > p.mods.len() {
            warn!("index out of range");
            anyhow::bail!("mod index out of range");
        }
        p.mods.remove(idx1 - 1);
    } else {
        let before = p.mods.len();
        p.mods.retain(|m| m != &a.target);

        if p.mods.len() == before {
            anyhow::bail!("mod not found");
        }
    }

    p.save(&path)?;
    info!(profile = %p.name, "mod removed");

    Ok(())
}
