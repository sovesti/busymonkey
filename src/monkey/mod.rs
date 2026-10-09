pub(crate) mod engine;
pub(crate) mod model;
pub(crate) mod script;

#[cfg(any(feature = "server", feature = "desktop"))]
pub(crate) use tokio::time::*;
#[cfg(feature = "web")]
pub(crate) use wasmtimer::std::*;
#[cfg(feature = "web")]
pub(crate) use wasmtimer::tokio::*;
