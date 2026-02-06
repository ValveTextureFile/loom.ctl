use anyhow::Ok;
use tracing::{debug, info};

use crate::{ctx::Context, interface::args::ProfileDelete, profilet::path::profile_path};

pub fn handle(a: ProfileDelete, ctx: &Context) -> anyhow::Result<()> {
    debug!("called");
    let path = profile_path(&ctx, &a.name);
    anyhow::ensure!(path.exists() == true, "path does not exist");
    debug!("removing {}", a.name);
    std::fs::remove_file(path.to_path_buf())?;
    info!("profile removed.");

    Ok(())
}
