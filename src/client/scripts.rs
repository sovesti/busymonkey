use dioxus::prelude::*;

use crate::client::components::Button;
use crate::client::settings::persistence::{use_save_settings, use_settings};
use crate::client::settings::registry::ScriptSettingsRegistry;
use crate::client::settings::scripts::ScriptSettingsView;
use crate::monkey::script::start_script;
use crate::scripts::api::available_scripts;

#[component]
pub fn ScriptsView() -> Element {
    let available = use_resource(available_scripts);
    let settings = use_settings::<ScriptSettingsRegistry>();
    let mut save = use_save_settings::<ScriptSettingsRegistry>();
    let mut access_denied = use_signal(|| false);
    let start = use_start_script(access_denied);

    #[cfg(feature = "desktop")]
    use_shortcut_on_desktop(start);

    rsx! {
        match &*available.read() {
            None => rsx! {
                p { "Loading scripts..." }
            },
            Some(Err(err)) => rsx! {
                p { class: "text-red", "{err}" }
            },
            Some(Ok(scripts)) => rsx! {
                if scripts.is_empty() {
                    p { "No scripts available for your account." }
                } else if let Some(mut registry) = *settings.read() {
                    label {
                        "Script"
                        select {
                            class: "m-2 p-2 bg-inherit",
                            value: if scripts.contains(&registry.read().chosen()) { registry.read().chosen() } else { String::new() },
                            onchange: move |evt: FormEvent| {
                                registry.write().select(evt.value());
                                save.call();
                            },
                            option { value: "", disabled: true, "Choose a script" }
                            for script in scripts {
                                option { key: "{script}", value: "{script}", "{script}" }
                            }
                        }
                    }
                }
            },
        }
        if let Some(registry) = *settings.read() {
            ScriptSettingsView { key: "{registry.read().chosen()}" }
        }
        if let Some(Err(err)) = save.value() {
            p { class: "text-red", "{err}" }
        }
        p { "Press space to toggle a script" }
        if access_denied() {
            dialog { open: true, class: "fixed inset-0 m-auto p-4",
                p { "This script is not available for your account." }
                Button { onclick: move |_| access_denied.set(false), "OK" }
            }
        }
        if let Some(Err(err)) = start.value() {
            p { class: "text-red", {err.to_string()} }
        }
    }
}

fn use_start_script(mut access_denied: Signal<bool>) -> Action<(), ()> {
    let settings = use_settings::<ScriptSettingsRegistry>();
    use_action(move || async move {
        access_denied.set(false);
        let registry = settings
            .read()
            .as_ref()
            .map(|registry| registry.read().clone());
        if let Some(registry) = registry {
            if !available_scripts().await?.contains(&registry.chosen()) {
                access_denied.set(true);
                return Ok(());
            }
            start_script(registry.create_with_settings()).await
        } else {
            Ok(())
        }
    })
}

#[cfg(feature = "desktop")]
fn toggle_script(script: &mut Action<(), ()>) {
    if script.pending() {
        script.cancel();
    } else {
        script.call();
    }
}

#[cfg(feature = "desktop")]
fn use_shortcut_on_desktop(mut script: Action<(), ()>) {
    dioxus::desktop::use_global_shortcut(KeyCode::Space, move |st| {
        if st == dioxus::desktop::HotKeyState::Released {
            return;
        }

        toggle_script(&mut script);
    })
    .unwrap();
}
