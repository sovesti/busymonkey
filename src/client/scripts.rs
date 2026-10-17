use dioxus::prelude::*;

use crate::monkey::script::start_script;

#[derive(Clone)]
enum ShortcutAction {
    ToggleScript,
}
#[derive(Clone)]
struct ShortcutBinding {
    keys: String,
    action: ShortcutAction,
}

#[component]
pub fn ScriptsView() -> Element {
    let script = use_action(start_script);
    #[cfg(feature = "desktop")]
    use_shortcut_on_desktop(script);

    let mut shortcuts = use_signal(|| {
        vec![
            ShortcutBinding {
                keys: String::from("Space"),
                action: ShortcutAction::ToggleScript,
            },
        ]
    });

    let mut new_shortcut = use_signal(|| String::new());

    rsx! {
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
                        td { "{binding.keys}" }

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
            style: "width: 220px; padding: 8px 10px; border: 1px solid #cbd5e1; border-radius: 6px; margin-right: 10px; box-sizing: border-box;",

            placeholder: "Enter shortcut",
            value: new_shortcut(),
            readonly: true,
            onkeydown: move |event| {
                let modifiers = event.modifiers();

                let ctrl = modifiers.contains(Modifiers::CONTROL);
                let shift = modifiers.contains(Modifiers::SHIFT);
                let alt = modifiers.contains(Modifiers::ALT);
                let mut combination = String::new();

                if ctrl {
                    combination.push_str("Ctrl + ");
                }

                if shift {
                    combination.push_str("Shift + ");
                }

                if alt {
                    combination.push_str("Alt + ");
                }

                let code = event.code().to_string();

                let symbol = code
                    .strip_prefix("Key")
                    .or(code.strip_prefix("Digit"));

                let mut recognized = false;

                if let Some(symbol) = symbol {
                    if symbol.len() == 1 {
                        combination.push_str(symbol);
                        recognized = true;
                    }
                } else if code == "Enter" {
                    combination.push_str("Enter");
                    recognized = true;
                } else if code == "Space" {
                    combination.push_str("Space");
                    recognized = true;
                } else if code == "Escape" {
                    combination.push_str("Escape");
                    recognized = true;
                } else if let Some(number) = code.strip_prefix("F") {
                    if let Ok(n) = number.parse::<u8>() {
                        if (1..=12).contains(&n) {
                            combination.push_str(&code);
                            recognized = true;
                        }
                    }
                }

                if recognized {
                    new_shortcut.set(combination);
                }

            }
        }
        button {
            style: "background-color: #f1f5f9; color: #475569; border: 1px solid #cbd5e1; border-radius: 6px; padding: 8px 12px; cursor: pointer; margin-right: 10px;",

            onclick: move |_| {
                new_shortcut.set(String::new());
            },

            "Clear"
        }

        button {
            style: "background-color: #2563eb; color: white; border: none; border-radius: 6px; padding: 8px 14px; cursor: pointer;",

            onclick: move |_| {
            let text = new_shortcut();
            let cleaned = text.trim();

            let exists = shortcuts
                .read()
                .iter()
                .any(|binding| binding.keys.as_str() == cleaned);

            if !cleaned.is_empty() && !exists {
                shortcuts.write().push(ShortcutBinding {
                    keys: cleaned.to_string(),
                    action: ShortcutAction::ToggleScript,
                });

                new_shortcut.set(String::new());
            }
        },
            "Add Shortcut"
        }

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
