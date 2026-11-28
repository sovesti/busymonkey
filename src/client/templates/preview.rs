use std::time::Duration;

use dioxus::prelude::*;

use crate::client::settings::persistence::use_settings;
use crate::client::templates::TokenIcon;
use crate::client::templates::persistence::{
    PersistentRecordedScript, PersistentRecordedScriptStoreExt,
};
use crate::monkey::model::{TimedToken, Token};
use crate::monkey::record::script::RecordedScript;

#[component]
pub fn RecordedScriptPreview() -> Element {
    let stored = use_settings::<PersistentRecordedScript>();
    rsx! {
        if let Some(stored) = stored() {
            div { class: "grow p-2 min-h-0 w-3/4 overflow-auto",
                table { class: "min-h-0 w-full table-fixed",
                    thead { class: "w-full",
                        tr { class: "w-full",
                            th { class: "w-1/8", "" }
                            th { class: "w-7/8 p-1 text-overlay0 text-md text-left", "Action" }
                        }
                    }
                    tbody {
                        for token in stored.tokens().read().as_ref().clone() {
                            tr { class: "border-t-1 border-crust",
                                TokenRow { token }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn TokenRow(token: ReadSignal<Token>) -> Element {
    rsx! {
        td { class: "m-1 p-1 text-overlay0",
            TokenIcon { token }
        }
        td { class: "m-1 p-1", "{TimedToken::new(Duration::ZERO, token())}" }
    }
}
