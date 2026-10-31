use std::fmt::{self, Display};

use dioxus::prelude::*;
use dioxus_icons::lucide::{CircleUser, Cog, ScrollText};

use crate::client::components::{
    Button, ButtonAlignment, ButtonSize, ButtonVariant, SidebarContent, use_sidebar,
};

use super::auth::AuthView;
use super::components::{Sidebar, SidebarHeader, SidebarInset, SidebarProvider, SidebarTrigger};
use super::scripts::ScriptsView;
use super::settings::SettingsView;

#[component]
pub fn ViewsInSidebar() -> Element {
    let current = use_signal(|| View::Scripts);
    rsx! {
        SidebarProvider {
            default_open: false,
            Sidebar {
                SidebarHeader { SidebarTrigger { class: "self-end" } },
                SidebarContent { for view in views() { ViewButton { view, current } } }
            },
            SidebarInset {
                CurrentView { current }
            }
        }
    }
}

#[component]
fn ViewButton(view: View, mut current: Signal<View>) -> Element {
    let ctx = use_sidebar();
    rsx! {
        Button {
            variant: ButtonVariant::Ghost,
            alignment: ButtonAlignment::Left,
            size: if ctx.open() { ButtonSize::Default } else { ButtonSize::Icon },
            onclick: move |_| current.set(view),
            Icon { view },
            if ctx.open() { span { "{view}" } }
        }
    }
}

#[component]
fn CurrentView(current: Signal<View>) -> Element {
    match current() {
        View::Account => rsx! { AuthView {} },
        View::Scripts => rsx! { ScriptsView {} },
        View::Settings => rsx! { SettingsView {} },
    }
}

fn views() -> Vec<View> {
    vec![View::Account, View::Scripts, View::Settings]
}

#[derive(Clone, Copy, PartialEq)]
enum View {
    Account,
    Scripts,
    Settings,
}

impl Display for View {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                View::Account => "Account",
                View::Scripts => "Scripts",
                View::Settings => "Settings",
            }
        )
    }
}

#[component]
fn Icon(view: View) -> Element {
    match view {
        View::Account => rsx! { CircleUser {} },
        View::Scripts => rsx! { ScrollText {} },
        View::Settings => rsx! { Cog {} },
    }
}
