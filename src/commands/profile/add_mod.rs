use tracing::{info};

use crate:: {
    ctx::Context, interface::args::ProfileAddMod, profilet::{Profile, path::profile_path}
};

pub fn handle(a: ProfileAddMod, ctx: & Context) -> anyhow::Result<()> {

    let path = profile_path(ctx, &a.name);
    let mut p = Profile::load(&path)?;

    p.mods.extend(a.mods);

    p.save(&path)?;

    info!(profile = %p.name, "mods added");

    Ok(())
}