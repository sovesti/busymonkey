use crate::monkey::{Instant, model::Token};

mod command;
mod state;
mod user;
mod util;
mod smooth;
#[cfg(feature = "desktop")]
mod run;

#[cfg(feature = "desktop")]
pub use run::start_script;
#[cfg(not(feature = "desktop"))]
pub async fn start_script(script: anyhow::Result<DynScript>) -> anyhow::Result<()> {
    anyhow::bail!("Can only run scripts on desktop")
}
pub use user::DynScript;



pub trait TokenProto {
    fn poll_action(&mut self) -> Option<Token>;

    fn poll_timeout(&mut self) -> Option<Instant>;

    fn handle_timeout(&mut self, now: Instant);

    fn is_finished(&mut self) -> bool;
}
