use chrono::{DateTime, Local};
use dioxus::prelude::*;
use dioxus_icons::lucide::{Keyboard, Mouse, MousePointer2, MousePointerClick};

use crate::client::components::Button;
use crate::monkey::model::{Direction, TimedToken, Token};
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
            class: "flex flex-col",
            Button {
                class: "flex-1",
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
        table {
            class: "flex-4",
            thead {
                tr {
                    th { "Action" }
                    th { "Time" }
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

#[component]
fn TokenRow(token: ReadSignal<TimedToken>) -> Element {
    rsx! {
        td {
            class: "flex flex-row",
            TokenIcon { token }
            {format_token(token)}
        }
        td {
            {format_time(token)}
        }
    }
}

fn use_storage(stopped: ReadSignal<bool>) -> Store<Stoppable<TruncateMoves<Linear>>> {
    use_store(move || Stoppable::new(TruncateMoves::new(Linear::new()), stopped))
}

#[component]
fn TokenIcon(token: ReadSignal<TimedToken>) -> Element {
    match token.read().token() {
        Token::Key(_, _) => rsx! { Keyboard {} },
        Token::Button(_, _) => rsx! { MousePointerClick {} },
        Token::MoveMouse(_, _, _) => rsx! { MousePointer2 {} },
        Token::Scroll(_, _) => rsx! { Mouse {} },
        _ => rsx! {},
    }
}

fn format_token(token: ReadSignal<TimedToken>) -> String {
    match token.read().token() {
        Token::Key(key, Direction::Press) => format!("Press {key:?}"),
        Token::Key(key, _) => format!("Release {key:?}"),
        Token::Button(button, _) => format!("Click {button:?} mouse button"),
        Token::MoveMouse(x, y, _) => format!("Move mouse to ({x}, {y})"),
        Token::Scroll(x, _) => format!(
            "Scroll {} pixels {}",
            x.abs(),
            if x.is_positive() { "down" } else { "up" }
        ),
        _ => String::new(),
    }
}

fn format_time(token: ReadSignal<TimedToken>) -> String {
    let local: DateTime<Local> = (DateTime::UNIX_EPOCH + token.read().timestamp()).into();
    local.format("%H:%M:%S").to_string()
}
