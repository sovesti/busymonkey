use crate::monkey::Instant;
use crate::monkey::model::{Token, parameters::DynScript};

mod command;
mod state;
mod user;
pub mod util;

#[cfg(any(feature = "desktop", feature = "web"))]
pub(crate) mod dummy;
#[cfg(any(feature = "desktop", feature = "web"))]
pub(crate) mod smooth;

pub trait MonkeyScript {
    fn poll_action(&mut self) -> Option<Token>;

    fn poll_timeout(&mut self) -> Option<Instant>;

    fn handle_timeout(&mut self, now: Instant);
}

pub async fn start_script(script: anyhow::Result<DynScript>) -> anyhow::Result<()> {
    #[cfg(any(feature = "desktop", feature = "web"))]
    smooth::run_smooth_wander().await?;
    Ok(())
}
