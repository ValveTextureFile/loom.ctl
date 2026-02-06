use crate::{
    ctx::Context, interface::args::ProfileFind, profilet::path::profile_path,
};
use anyhow::ensure;
use tracing::{debug, info};

pub fn handle(a: ProfileFind, ctx: &Context) -> anyhow::Result<()> {
    debug!("looking for {}", a.name);
    let path = profile_path(ctx, &a.name);

    ensure!(path.exists(), "there is no profile called {}", a.name);

    info!("there is an associated path:\n{}", path.display());
    Ok(())
}