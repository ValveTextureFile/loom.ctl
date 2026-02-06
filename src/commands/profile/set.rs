use tracing::{debug, info, warn};

use crate::{
    ctx::Context, interface::args::ProfileSet, profilet::{Profile, path::profile_path},
};

pub fn handle(a: ProfileSet, ctx: &Context) -> anyhow::Result<()> {
    let path = profile_path(&ctx, &a.name);
    anyhow::ensure!(path.exists() == true, "path does not exist");

    let mut p = Profile::load(path.as_path())?;

    if let Some(engine) = a.engine {
        p.engine = Some(engine);
    }

    if let Some(iwad) = a.iwad {
        p.iwad = Some(iwad);
    }

    p.save(&path)?;

    info!("updated profile {}", p.name);

    Ok(())
}
