#[cfg(feature = "web")]
mod web;

pub use enigo::{Keyboard, Mouse, agent::Agent};

pub trait MonkeyEngine: Agent {}

impl<T: Agent> MonkeyEngine for T {}

#[cfg(any(feature = "desktop", feature = "web"))]
pub fn default_engine() -> anyhow::Result<impl MonkeyEngine> {
    #[cfg(feature = "desktop")]
    {
        Ok(enigo::Enigo::new(&enigo::Settings::default())?)
    }
    #[cfg(feature = "web")]
    {
        Ok(web::BrowserMonkeyEngine {})
    }
}
