use dioxus::prelude::*;

use crate::client::desktop::shortcut::ShortcutSettings;
use crate::client::settings::Settings;
use crate::client::settings::persistence::{provide_settings, use_settings};
use crate::client::settings::registry::ScriptSettingsRegistry;
use crate::monkey::script::start_script;

#[component]
pub fn ScriptsView() -> Element {
    provide_settings::<ScriptSettingsRegistry>();
    #[cfg(feature = "desktop")]
    use_shortcut_on_desktop(use_start_script());

    rsx! {
        Settings {},
        ShortcutSettings {},
        p {
            "Press space to toggle a script"
        },
        if let Some(Err(err)) = use_start_script().value() {
            p {
                class: "text-red",
                {err.to_string()}
            }
        }
    }
}

fn use_start_script() -> Action<(), ()> {
    use_action(move || async move {
        if let Some(registry) = *use_settings::<ScriptSettingsRegistry>().read() {
            start_script(registry.read().create_with_settings()).await
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
