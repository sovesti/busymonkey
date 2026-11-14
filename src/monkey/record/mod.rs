mod keycodes;
#[cfg(feature = "desktop")]
pub(crate) mod rdev;
pub(crate) mod storage;
#[cfg(test)]
mod test;

use std::pin::Pin;
use std::sync::Arc;

use dioxus::hooks::use_context;
use dioxus::prelude::*;
use tokio_stream::{Stream, StreamExt};

use crate::monkey::model::TimedToken;
use crate::monkey::record::storage::RecordingStorage;

pub fn provide_input_channel() {
    desktop! {
        rdev::provide_rdev();
    }
}

pub fn use_input_channel() -> DynInputChannel {
    use_context()
}

pub type DynInputChannel = Arc<dyn InputChannel>;
pub type DynTokenStream = Pin<Box<dyn Stream<Item = anyhow::Result<TimedToken>>>>;

pub trait InputChannel {
    fn subscribe(&self) -> DynTokenStream;
}

pub fn record_to(storage: impl RecordingStorage + Clone) -> UseFuture {
    let channel = use_input_channel();
    use_future(move || record_channel(storage.clone(), channel.clone()))
}

pub async fn record_channel(mut storage: impl RecordingStorage, channel: DynInputChannel) {
    let mut subscription = channel.subscribe();
    while storage.active()
        && let Some(next) = subscription.next().await
    {
        storage.add(next);
    }
    storage.finish();
}
