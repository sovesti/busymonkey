use dioxus::prelude::*;

use crate::monkey::script::dummy::start_script;

#[component]
pub fn ScriptsView() -> Element {
    let script = use_action(start_script);
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

fn use_shortcut_on_desktop(script: Action<(), ()>) {
    #[cfg(feature = "desktop")]
    dioxus::desktop::use_global_shortcut(KeyCode::Space, move |st| {
        if st == dioxus::desktop::HotKeyState::Pressed {
            toggle_script(script)
        }
    })
    .unwrap();
}

fn toggle_script(mut script: Action<(), ()>) {
    if script.pending() {
        script.cancel();
    } else {
        script.call();
    }
}
