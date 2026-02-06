use tracing::{info, warn, debug};

use crate::{
    ctx::Context, interface::args::DoctorArgs
};

pub fn handle(a: DoctorArgs, ctx: &Context) -> anyhow::Result<()> {
    info!("[doctor] i do absolutely nada");
    Ok(())
}
