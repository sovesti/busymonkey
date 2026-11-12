pub(crate) mod engine;
pub(crate) mod model;
pub(crate) mod script;

#[cfg(any(feature = "server", feature = "desktop"))]
pub(crate) use tokio::time::{Instant, sleep_until};
#[cfg(feature = "web")]
pub(crate) use wasmtimer::std::Instant;
#[cfg(feature = "web")]
pub(crate) use wasmtimer::tokio::sleep_until;
