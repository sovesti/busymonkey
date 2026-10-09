use dioxus::prelude::*;

use crate::monkey::script::start_script;

#[component]
pub fn ScriptsView() -> Element {
    let script = use_action(start_script);
    #[cfg(feature = "desktop")]
    use_shortcut_on_desktop(script);
    rsx! {
        p {
            "Press space to toggle a script"
        },
        if let Some(Err(err)) = script.value() {
            p {
                class: "text-red",
                {err.to_string()}
            }
        }
    }
}

#[cfg(feature = "desktop")]
fn use_shortcut_on_desktop(mut script: Action<(), ()>) {
    dioxus::desktop::use_global_shortcut(KeyCode::Space, move |st| {
        if st == dioxus::desktop::HotKeyState::Released {
            return;
        }
        if script.pending() {
            script.cancel();
        } else {
            script.call();
        }
    })
    .unwrap();
}
