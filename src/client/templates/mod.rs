pub(crate) mod persistence;
pub(crate) mod preview;
pub(crate) mod script;

use dioxus::prelude::*;
use dioxus_icons::lucide::{Keyboard, Mouse, MousePointer2, MousePointerClick};

use crate::client::components::{Button, ButtonVariant};
use crate::client::settings::persistence::{use_save_settings, use_settings};
use crate::client::templates::persistence::{
    PersistentRecordedScript, PersistentRecordedScriptStoreExt,
};
use crate::monkey::model::{TimedToken, Token};
use crate::monkey::record::storage::linear::{Linear, LinearStoreExt};
use crate::monkey::record::storage::stoppable::{Stoppable, StoppableStoreExt};
use crate::monkey::record::storage::truncate::{TruncateMoves, TruncateMovesStoreExt};
use crate::monkey::record::use_recording;

#[component]
pub fn TemplatesView() -> Element {
    let mut running = use_signal(|| true);
    let storage = use_storage(running.into());
    let mut recorded = use_settings::<PersistentRecordedScript>();
    let mut save = use_save_settings::<PersistentRecordedScript>();
    use_recording(storage);
    rsx! {
        div { class: "flex flex-col min-h-0 items-center m-1",
            div {
                class: "flex flex-row justify-end m-1 gap-4",
                Button {
                    variant: ButtonVariant::Ghost,
                    disabled: !running(),
                    onclick: move |_| running.set(false),
                    "Stop"
                }
                Button {
                    variant: ButtonVariant::Solid,
                    disabled: running(),
                    onclick: move |_| {
                        if let Some(recorded) = recorded() {
                            recorded.tokens().write().as_mut().extend_from_slice(storage.delegate().delegate().all()().into_iter().map(|t| t.token().clone()).collect::<Vec<_>>().as_slice());
                            save.call();
                        }
                    },
                    "Save"
                }
            }
            TokensTable { store: storage.delegate().delegate().all() }
        }
    }
}

#[component]
fn TokensTable(store: Store<Vec<TimedToken>>) -> Element {
    rsx! {
        div { class: "grow p-2 min-h-0 w-3/4 overflow-auto",
            table { class: "min-h-0 w-full table-fixed",
                thead { class: "w-full",
                    tr { class: "w-full",
                        th { class: "w-1/8", "" }
                        th { class: "w-1/2 p-1 text-overlay0 text-md text-left", "Action" }
                        th { class: "p-1 text-overlay0 text-md text-left", "Time" }
                    }
                }
                tbody {
                    for token in store.iter() {
                        tr { class: "border-t-1 border-crust",
                            TokenRow { token }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn TokenRow(token: ReadSignal<TimedToken>) -> Element {
    rsx! {
        td { class: "m-1 p-1 text-overlay0",
            TokenIcon { token: token.map(TimedToken::token) }
        }
        td { class: "m-1 p-1", "{token}" }
        td { class: "m-1 p-1 text-left", {token.read().format_time()} }
    }
}

fn use_storage(stopped: ReadSignal<bool>) -> Store<Stoppable<TruncateMoves<Linear>>> {
    use_store(move || Stoppable::new(TruncateMoves::new(Linear::new()), stopped))
}

#[component]
pub fn TokenIcon(token: ReadSignal<Token>) -> Element {
    match token() {
        Token::Key(_, _) => rsx! {
            Keyboard { size: 20 }
        },
        Token::Button(_, _) => rsx! {
            MousePointerClick { size: 20 }
        },
        Token::MoveMouse(_, _, _) => rsx! {
            MousePointer2 { size: 20 }
        },
        Token::Scroll(_, _) => rsx! {
            Mouse { size: 20 }
        },
        _ => rsx! {},
    }
}
