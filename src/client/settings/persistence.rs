use std::{fmt::Debug, marker::PhantomData};

use dioxus::{core::provide_root_context, prelude::*};
use serde::{Serialize, de::DeserializeOwned};

use crate::client;

pub fn use_settings<T: SettingsCategory>() -> Resource<Signal<T>> {
    use_context::<Resource<Signal<T>>>()
}

pub fn use_save_settings<T: SettingsCategory>() -> Action<(), ()> {
    let settings = use_settings::<T>();
    use_action(move || async move {
        if let Some(ready) = settings().map(|ready| ready()) {
            ready.try_save().await?;
        }
        Ok::<(), anyhow::Error>(())
    })
}

pub trait SettingsCategory:
    DeserializeOwned + Serialize + Clone + Default + Debug + 'static
{
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

    async fn try_save(self) -> anyhow::Result<()> {
        desktop! {
            client::desktop::settings::try_save(Self::name(), &self).await
        }
        #[cfg(not(feature = "desktop"))]
        {
            Ok(())
        }
    }
}
