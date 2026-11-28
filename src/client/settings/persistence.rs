use std::{fmt::Debug, time::Duration};

use dioxus::{core::provide_root_context, prelude::*};
use dioxus_primitives::toast::{ToastOptions, consume_toast};
use serde::{Serialize, de::DeserializeOwned};

use crate::client;

pub fn use_settings<T: SettingsCategory>() -> Resource<Store<T>> {
    use_context::<Resource<Store<T>>>()
}

pub fn use_save_settings<T: SettingsCategory>() -> Action<(), ()> {
    let settings = use_settings::<T>();
    use_action(move || async move {
        if let Some(ready) = settings().map(|ready| ready()) {
            ready.try_save().await.map_err(|err| {
                consume_toast().error(format!("Problems with saving: {err}"), two_secs());
                err
            })?;
        }
        consume_toast().info("Saved successfully".to_owned(), two_secs());
        Ok::<(), anyhow::Error>(())
    })
}

fn two_secs() -> ToastOptions {
    ToastOptions::new().duration(Duration::from_secs(2))
}

pub trait SettingsCategory:
    DeserializeOwned + Serialize + Clone + Default + Debug + 'static
{
    fn name() -> String;

    async fn load_or_default() -> Store<Self> {
        desktop! {
            Store::new(client::desktop::settings::load_or_default(Self::name()).await)
        }
        #[cfg(not(feature = "desktop"))]
        {
            Store::new(Self::default())
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
