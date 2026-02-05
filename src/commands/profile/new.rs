use tracing::{info, warn, debug};

use crate::{
    ctx::Context, interface::args::ProfileNew, profilet::{Profile, pathbuf::profile_path},
};

pub fn handle(a: ProfileNew, ctx: &Context) -> anyhow::Result<()> {
    debug!("#[profile::new] called");
    let path = profile_path(ctx, &a.name);

    if path.exists() {
        warn!("[profile::new] profile {} already exists", a.name);
        anyhow::bail!("profile '{}' already exists", a.name);
    }

    debug!("#[profile::new] making new profile.");
    let p = Profile::new(a.name.clone());
    debug!("#[profile::new] saving...");
    p.save(&path)?;

    info!("[profile::new] new profile @ {}", a.name);
    Ok(())
}
