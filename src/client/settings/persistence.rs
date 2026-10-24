use dioxus::prelude::*;
use serde::{Serialize, de::DeserializeOwned};

use crate::client;

pub fn provide_settings<T: SettingsCategory>() {
    provide_context(use_resource(T::load_or_default));
}

pub fn use_settings<T: SettingsCategory>() -> Resource<Signal<T>> {
    use_context::<Resource<Signal<T>>>()
}

pub fn use_save_settings<T: SettingsCategory>() -> Action<(), ()> {
    use_action(|| async {
        if let Some(ready) = &*use_settings::<T>().read() {
            ready.read().try_save().await?;
        }
        Ok::<(), anyhow::Error>(())
    })
}

pub trait SettingsCategory: DeserializeOwned + Serialize + Default + 'static {
    fn name() -> String;

    async fn load_or_default() -> Signal<Self> {
        desktop! {
            Signal::new(client::desktop::settings::load_or_default(Self::name()).await)
        }
        #[cfg(not(feature = "desktop"))]
        {
            Signal::default()
        }
    }

    async fn try_save(&self) -> anyhow::Result<()> {
        desktop! {
            client::desktop::settings::try_save(Self::name(), self).await
        }
        #[cfg(not(feature = "desktop"))]
        {
            Ok(())
        }
    }
}
