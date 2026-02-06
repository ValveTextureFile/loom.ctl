use tracing::{info, debug};

use crate::{
    ctx::Context, interface::args::ProfileNew, profilet::{Profile, path::profile_path},
};

pub fn handle(a: ProfileNew, ctx: &Context) -> anyhow::Result<()> {
    debug!("called");
    let path = profile_path(ctx, &a.name);

    anyhow::ensure!(path.exists() == false, "path already exists");

    debug!("making new profile.");
    let p = Profile::new(a.name.clone());
    debug!("saving...");
    p.save(&path)?;

    info!("new profile @ {}", a.name);
    Ok(())
}
