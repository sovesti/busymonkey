use crate::monkey::Instant;

use super::model::Token;

#[cfg(any(feature = "desktop", feature = "web"))]
pub(crate) mod dummy;

pub trait MonkeyScript {
    fn poll_action(&mut self) -> Option<Token>;

    fn poll_timeout(&self) -> Option<Instant>;

    fn handle_timeout(&mut self, now: Instant);
}

pub async fn start_script() -> anyhow::Result<()> {
    #[cfg(any(feature = "desktop", feature = "web"))]
    dummy::start_dummy_script().await?;
    Ok(())
}
