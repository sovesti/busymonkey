use dioxus::prelude::{desktop, web};
use anyhow::bail;
use crate::monkey::Instant;
use crate::monkey::model::{Token, parameters::DynScript};

mod command;
pub(crate) mod jitter;
#[cfg(any(feature = "desktop"))]
mod state;
mod user;
pub mod util;

#[cfg(any(feature = "desktop"))]
pub(crate) mod dummy;
#[cfg(any(feature = "desktop"))]
pub(crate) mod smooth;

pub trait MonkeyScript {
    fn poll_action(&mut self) -> Option<Token>;

    fn poll_timeout(&mut self) -> Option<Instant>;

    fn handle_timeout(&mut self, now: Instant);
}

pub async fn start_script(script: anyhow::Result<DynScript>) -> anyhow::Result<()> {
    desktop! {
        smooth::run_smooth_wander().await?;
    }
    web! {
        bail!("Running scripts form web is not supported")
    }
    Ok(())
}
