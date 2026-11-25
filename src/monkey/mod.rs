#[cfg(feature = "desktop")]
pub(crate) mod engine;
pub(crate) mod model;
pub(crate) mod record;
#[allow(clippy::all)]
#[allow(unused)]
pub(crate) mod script;

#[cfg(not(feature = "web"))]
pub(crate) use tokio::time::*;
#[cfg(feature = "web")]
pub(crate) use wasmtimer::std::*;
#[cfg(feature = "web")]
pub(crate) use wasmtimer::tokio::*;
