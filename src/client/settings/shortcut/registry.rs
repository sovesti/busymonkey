use std::collections::HashMap;
use std::fmt::{self, Display};

use dioxus::prelude::*;

use crate::client::settings::persistence::{SettingsCategory, use_settings};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ShortcutAction {
    ToggleScript,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ShortcutKeys {
    code: Code,
    modifiers: Modifiers,
}

impl ShortcutKeys {
    pub fn new(code: Code, modifiers: Modifiers) -> Self {
        Self { code, modifiers }
    }
}

impl Display for ShortcutKeys {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.modifiers.contains(Modifiers::CONTROL) {
            write!(f, "Ctrl + ")?;
        }
        if self.modifiers.contains(Modifiers::SHIFT) {
            write!(f, "Shift + ")?;
        }
        if self.modifiers.contains(Modifiers::ALT) {
            write!(f, "Alt + ")?;
        }
        let code = self.code.to_string();
        let key = code
            .strip_prefix("Key")
            .or(code.strip_prefix("Digit"))
            .unwrap_or(code.as_str());
        write!(f, "{key}")
    }
}

#[derive(Clone, Debug, Store)]
pub struct ShortcutsRegistry {
    known: HashMap<ShortcutAction, ShortcutKeys>,
}

impl ShortcutsRegistry {
    pub fn new() -> Self {
        Self::default()
    }
}

#[store(pub)]
impl<Lens> Store<ShortcutsRegistry, Lens> {
    fn load_or_default(&self, action: ShortcutAction) -> Option<ShortcutKeys> {
        self.known()
            .get(action)
            .map(|store| store.cloned())
            .or_else(|| ShortcutsRegistry::default().known.get(&action).copied())
    }
}

impl Default for ShortcutsRegistry {
    fn default() -> Self {
        Self {
            known: HashMap::from([(
                ShortcutAction::ToggleScript,
                ShortcutKeys::new(
                    Code::Space,
                    Modifiers::empty() | Modifiers::CONTROL | Modifiers::SHIFT,
                ),
            )]),
        }
    }
}

// impl SettingsCategory for ShortcutsRegistry {
//     fn name() -> String {
//         "shortcuts".to_owned()
//     }
// }

pub fn use_shortcut_keys(action: ShortcutAction) -> Memo<Option<ShortcutKeys>> {
    // let settings = use_settings::<ShortcutsRegistry>();
    let settings = use_resource(|| async { use_store(ShortcutsRegistry::default) });
    use_memo(move || (*settings.read())?.load_or_default(action))
}
