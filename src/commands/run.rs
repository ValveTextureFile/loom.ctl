use tracing::{info, warn, debug};

use crate::{ctx::Context, interface::args::RunArgs};

pub fn handle(a: RunArgs, ctx: &mut Context) -> anyhow::Result<()> {
    // if ctx.verbose > 0 {
    //     eprintln!("[run] {a:#?}");
    // } else {
    //     println!(
    //         "[run] profile={:?} engine={:?} iwad={:?} mods={} passthrough={}",
    //         a.profile,
    //         a.engine,
    //         a.iwad,
    //         a.mods.len(),
    //         a.passthrough.len()
    //     );
    // }
    info!("[run] {a:#?}");
    debug!("[run] profile={:?} engine={:?} iwad={:?} mods={} passthrough={}",
            a.profile,
            a.engine,
            a.iwad,
            a.mods.len(),
            a.passthrough.len());

    Ok(())
}
