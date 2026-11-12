use dioxus::prelude::*;

use crate::client::components::Button;

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
pub(crate) fn ShortuctSettingsView() -> Element {
    let shortcuts = use_signal(|| {
        vec![ShortcutBinding {
            keys: ShortcutKeys {
                code: Code::Space,
                modifiers: Modifiers::empty(),
            },
            action: ShortcutAction::ToggleScript,
        }]
    });

    let mut captured_keys = use_signal(|| None::<ShortcutKeys>);
    rsx! {
        div {
            class: "flex flex-col gap-1 w-64",
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
            ShortcutsTable { shortcuts }
            input {
                class: "shortcut-input",
                style: "width: 220px; padding: 8px 10px; border: 1px solid #cbd5e1; border-radius: 6px; margin-right: 10px; box-sizing: border-box;",
                placeholder: "Enter shortcut",
                value: match captured_keys() {
                    Some(keys) => keys.to_label(),
                    None => String::new(),
                },
                readonly: true,
                onkeydown: move |event| capture_key(event, captured_keys)
            }
            Button {
                onclick: move |_| captured_keys.set(None),
                "Clear"
            }
            Button {
                onclick: move |_| add_shortcut(shortcuts, captured_keys),
                "Add Shortcut"
            }
        }
    }
}

#[component]
fn ShortcutsTable(mut shortcuts: Signal<Vec<ShortcutBinding>>) -> Element {
    rsx! {
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
                            Button {
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
    }
}

fn capture_key(event: KeyboardEvent, mut captured: Signal<Option<ShortcutKeys>>) {
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
    if recognized {
        let modifiers =
            event.modifiers() & (Modifiers::CONTROL | Modifiers::SHIFT | Modifiers::ALT);
        captured.set(Some(ShortcutKeys { code, modifiers }));
    }
}

fn add_shortcut(
    mut shortcuts: Signal<Vec<ShortcutBinding>>,
    mut captured: Signal<Option<ShortcutKeys>>,
) {
    if let Some(keys) = captured()
        && !shortcuts.read().iter().any(|binding| binding.keys == keys)
    {
        shortcuts.write().push(ShortcutBinding {
            keys,
            action: ShortcutAction::ToggleScript,
        });
        captured.set(None);
    }
}
