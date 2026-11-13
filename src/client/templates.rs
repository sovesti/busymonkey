use dioxus::prelude::*;
use dioxus_icons::lucide::{Keyboard, Mouse, MousePointer2, MousePointerClick};

use crate::client::components::Button;
use crate::monkey::model::{TimedToken, Token};
use crate::monkey::record::record_to;
use crate::monkey::record::storage::linear::{Linear, LinearStoreExt};
use crate::monkey::record::storage::stoppable::{Stoppable, StoppableStoreExt};
use crate::monkey::record::storage::truncate::{TruncateMoves, TruncateMovesStoreExt};

#[component]
pub fn TemplatesView() -> Element {
    let mut running = use_signal(|| true);
    let storage = use_storage(running.into());
    record_to(storage);
    rsx! {
        div {
            class: "flex flex-col min-h-0 items-center m-1",
            Button {
                disabled: !running(),
                onclick: move |_| running.set(false),
                "Stop"
            }
            TokensTable { store: storage.delegate().delegate().all() }
        }
    }
}

#[component]
fn TokensTable(store: Store<Vec<TimedToken>>) -> Element {
    rsx! {
        div {
            class: "grow p-2 min-h-0 w-3/4 overflow-auto",
            table {
                class: "min-h-0 w-full table-fixed",
                thead {
                    class: "w-full",
                    tr {
                        class: "w-full",
                        th {
                            class: "w-1/8",
                            ""
                        }
                        th {
                            class: "w-1/2",
                            "Action"
                        }
                        th {
                            "Time"
                        }
                    }
                }
                tbody {
                    for token in store.iter() {
                        tr { TokenRow { token } }
                    }
                }
            }
        }
    }
}

#[component]
fn TokenRow(token: ReadSignal<TimedToken>) -> Element {
    rsx! {
        td {
            class: "m-1 p-1",
            TokenIcon { token }
        }
        td {
            class: "m-1 p-1",
            "{token}"
        }
        td {
            class: "m-1 p-1 text-center",
            {token.read().format_time()}
        }
    }
}

fn use_storage(stopped: ReadSignal<bool>) -> Store<Stoppable<TruncateMoves<Linear>>> {
    use_store(move || Stoppable::new(TruncateMoves::new(Linear::new()), stopped))
}

#[component]
fn TokenIcon(token: ReadSignal<TimedToken>) -> Element {
    match token.read().token() {
        Token::Key(_, _) => rsx! { Keyboard { size: 20 } },
        Token::Button(_, _) => rsx! { MousePointerClick { size: 20 } },
        Token::MoveMouse(_, _, _) => rsx! { MousePointer2 { size: 20 } },
        Token::Scroll(_, _) => rsx! { Mouse { size: 20 } },
        _ => rsx! {},
    }
}
