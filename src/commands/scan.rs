use tracing::{info, warn, debug};

use crate::{
    ctx::Context, interface::args::ScanArgs
};

pub fn handle(a: ScanArgs, ctx: &Context) -> anyhow::Result<()> {
    debug!("[scan] called");
    Ok(())
}
