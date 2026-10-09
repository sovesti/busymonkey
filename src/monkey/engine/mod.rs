pub use enigo::agent::Agent;

pub trait MonkeyEngine: Agent {}

impl MonkeyEngine for enigo::Enigo {}

pub fn default_engine() -> anyhow::Result<impl MonkeyEngine> {
    Ok(enigo::Enigo::new(&enigo::Settings::default())?)
}
