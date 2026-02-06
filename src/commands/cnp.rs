use tracing::{info, warn, debug};

use crate::{
    ctx::Context, interface::args::XXXArgs
};

pub fn handle(a: ProfileNew, ctx: &Context) -> anyhow::Result<()> {
    info!("[cnp] hey, i shouldnt be called.");
    Ok(())
}
