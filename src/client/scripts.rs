use dioxus::prelude::*;

use crate::client::settings::persistence::{provide_settings, use_settings};
use crate::client::settings::{Settings, registry::ScriptSettingsRegistry};
use crate::monkey::script::start_script;

#[derive(Clone)]
enum ShortcutAction {
    ToggleScript,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct ShortcutKeys {
    code: Code,
    modifiers: Modifiers,
}

impl ShortcutKeys {
    fn to_label(&self) -> String {
        let mut label = String::new();

        if self.modifiers.contains(Modifiers::CONTROL) {
            label.push_str("Ctrl + ");
        }

        if self.modifiers.contains(Modifiers::SHIFT) {
            label.push_str("Shift + ");
        }

        if self.modifiers.contains(Modifiers::ALT) {
            label.push_str("Alt + ");
        }

        let code = self.code.to_string();

        let key = code
            .strip_prefix("Key")
            .or(code.strip_prefix("Digit"))
            .unwrap_or(code.as_str());

        label.push_str(key);

        label
    }
}

#[derive(Clone)]
struct ShortcutBinding {
    keys: ShortcutKeys,
    action: ShortcutAction,
}

#[component]
pub fn ScriptsView() -> Element {
    let mut shortcuts = use_signal(|| {
        vec![ShortcutBinding {
            keys: ShortcutKeys {
                code: Code::Space,
                modifiers: Modifiers::empty(),
            },
            action: ShortcutAction::ToggleScript,
        }]
    });

    let mut captured_keys = use_signal(|| None::<ShortcutKeys>);
    provide_settings::<ScriptSettingsRegistry>();
    #[cfg(feature = "desktop")]
    use_shortcut_on_desktop(use_start_script());

    rsx! {
        Settings {},
        style {
            ".shortcut-input:focus {{
                outline: 2px solid #2563eb;
                outline-offset: 2px;
                border-color: #2563eb;
            }}"
        }

        h2 {
            "Keyboard Shortcuts"
        }

        table {
            style: "border-collapse: separate; border-spacing: 24px 10px;",

            thead {
                tr {
                    th { "Shortcut" }
                    th { "Action"}
                    th { "Delete" }
                }
            }
            tbody {
                for (index, binding) in shortcuts().into_iter().enumerate() {
                    tr {
                        td {
                            {binding.keys.to_label()}
                        }

                        td {
                            select {
                                value: match &binding.action {
                                    ShortcutAction::ToggleScript => "toggle_script",
                                },

                                onchange: move |event| {
                                    if event.value() == "toggle_script" {
                                        if let Some(item) = shortcuts.write().get_mut(index) {
                                            item.action = ShortcutAction::ToggleScript;
                                        }
                                    }
                                },

                                option {
                                    value: "toggle_script",
                                    "Start/Stop script"
                                }
                            }
                        }

                        td {
                            button {
                                style: "background-color: #fee2e2; color: #b91c1c; border: none; border-radius: 6px; padding: 6px 10px; cursor: pointer;",

                                onclick: move |_| {
                                    shortcuts.write().remove(index);
                                },
                                "Delete"
                            }
                        }
                    }
                }
            }
        }

        input {
            class: "shortcut-input",

            style: "width: 220px; padding: 8px 10px; border: 1px solid #cbd5e1; border-radius: 6px; margin-right: 10px; box-sizing: border-box;",

            placeholder: "Enter shortcut",
            value: match captured_keys() {
                Some(keys) => keys.to_label(),
                None => String::new(),
            },
            readonly: true,
            onkeydown: move |event| {
                let code = event.code();
                let code_text = code.to_string();

                let symbol = code_text
                    .strip_prefix("Key")
                    .or(code_text.strip_prefix("Digit"));

                let mut recognized = false;

                if let Some(symbol) = symbol {
                    if symbol.len() == 1 {
                        recognized = true;
                    }
                } else if code_text == "Enter"
                    || code_text == "Space"
                    || code_text == "Escape"
                {
                    recognized = true;
                } else if let Some(number) = code_text.strip_prefix("F") {
                    if let Ok(n) = number.parse::<u8>() {
                        if (1..=12).contains(&n) {
                            recognized = true;
                        }
                    }
                }

                if recognized {
                    let modifiers = event.modifiers()
                        & (Modifiers::CONTROL | Modifiers::SHIFT | Modifiers::ALT);

                    let keys = ShortcutKeys {
                        code,
                        modifiers,
                    };

                    captured_keys.set(Some(keys));
                }
            }
        }
        button {
            style: "background-color: #f1f5f9; color: #475569; border: 1px solid #cbd5e1; border-radius: 6px; padding: 8px 12px; cursor: pointer; margin-right: 10px;",

            onclick: move |_| {
                captured_keys.set(None);
            },

            "Clear"
        }

        button {
            style: "background-color: #2563eb; color: white; border: none; border-radius: 6px; padding: 8px 14px; cursor: pointer;",

            onclick: move |_| {
                if let Some(keys) = captured_keys() {
                    let exists = shortcuts
                        .read()
                        .iter()
                        .any(|binding| binding.keys == keys);

                    if !exists {
                        shortcuts.write().push(ShortcutBinding {
                            keys,
                            action: ShortcutAction::ToggleScript,
                        });

                        captured_keys.set(None);
                    }
                }
            },
            "Add Shortcut"
        }

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
