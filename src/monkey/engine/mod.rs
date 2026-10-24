#[cfg(feature = "web")]
mod web;

// first to change
pub use enigo::{Keyboard, Mouse, agent::Agent};

// first to change
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
