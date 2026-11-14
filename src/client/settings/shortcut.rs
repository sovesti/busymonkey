use std::collections::HashMap;

use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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

#[component]
pub(crate) fn ShortuctSettingsView() -> Element {
    let mut shortcuts = use_signal(|| {
        HashMap::from([(
            ShortcutAction::ToggleScript,
            ShortcutKeys {
                code: Code::Space,
                modifiers: Modifiers::empty(),
            },
        )])
    });
    let mut editing = use_signal(|| false);

    let current_keys = shortcuts
        .read()
        .get(&ShortcutAction::ToggleScript)
        .copied();

    rsx! {
        div {
            class: "w-full max-w-5xl text-text",

            h2 {
                class: "text-xl font-normal mb-6",
                "Настройки горячих клавиш"
            }

            table {
                class: "w-full table-fixed border-collapse",

                thead {
                    tr {
                        th {
                            class: "text-left font-normal text-lg pb-3",
                            style: "width: 37%;",
                            "Action"
                        }
                        th {
                            class: "text-left font-normal text-lg pb-3",
                            "Shortcut"
                        }
                        th {
                            style: "width: 116px;",
                        }
                    }
                }

                tbody {
                    tr {
                        class: "border-t border-surface1",

                        td {
                            class: "py-3 pr-3",
                            "Start/Stop script"
                        }

                        td {
                            class: "py-3 pr-3",
                            if editing() {
                                input {
                                    class: "w-full max-w-56 rounded-md border border-surface2 bg-base px-3 py-2 text-text focus:outline-2 focus:outline-blue-500",
                                    r#type: "text",
                                    readonly: true,
                                    placeholder: "Press shortcut...",
                                    onmounted: move |element| async move {
                                        let _ = element.data().set_focus(true).await;
                                    },
                                    onkeydown: move |event| {
                                        if let Some(keys) = shortcut_from_event(&event) {
                                            event.prevent_default();
                                            shortcuts.write().insert(ShortcutAction::ToggleScript, keys);
                                            editing.set(false);
                                        }
                                    }
                                }
                            } else if let Some(keys) = current_keys {
                                "{keys.to_label()}"
                            } else {
                                "Not assigned"
                            }
                        }

                        td {
                            class: "py-3 text-right",
                            if editing() {
                                button {
                                    class: "rounded-full border border-surface2 px-4 py-1 hover:bg-surface0",
                                    onclick: move |_| editing.set(false),
                                    "Cancel"
                                }
                            } else {
                                button {
                                    class: "rounded-full border border-surface2 px-4 py-1 hover:bg-surface0",
                                    onclick: move |_| editing.set(true),
                                    "Change"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn shortcut_from_event(event: &KeyboardEvent) -> Option<ShortcutKeys> {
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
    } else if code_text == "Enter" || code_text == "Space" || code_text == "Escape" {
        recognized = true;
    } else if let Some(number) = code_text.strip_prefix("F") {
        if let Ok(n) = number.parse::<u8>() {
            if (1..=12).contains(&n) {
                recognized = true;
            }
        }
    }

    if !recognized {
        return None;
    }

    let modifiers =
        event.modifiers() & (Modifiers::CONTROL | Modifiers::SHIFT | Modifiers::ALT);

    Some(ShortcutKeys { code, modifiers })
}