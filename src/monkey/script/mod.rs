use crate::monkey::{Instant, model::Token};

mod command;
#[cfg(feature = "desktop")]
mod state;
#[cfg(feature = "desktop")]
mod user;
#[cfg(feature = "desktop")]
mod util;
#[cfg(feature = "desktop")]
mod smooth_wander;
#[cfg(feature = "desktop")]
mod run;

#[cfg(feature = "desktop")]
pub use run::start_script;
#[cfg(not(feature = "desktop"))]
pub async fn start_script(script: anyhow::Result<DynScript>) -> anyhow::Result<()> {
    anyhow::bail!("Can only run scripts on desktop")
}
pub use command::DynScript;

pub trait TokenProto {
    fn poll_action(&mut self) -> Option<Token>;

    fn poll_timeout(&mut self) -> Option<Instant>;

    fn handle_timeout(&mut self, now: Instant);

    fn is_finished(&mut self) -> bool;
}
