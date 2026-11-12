use dioxus::CapturedError;
use dioxus::prelude::*;

use super::components::Button;
use crate::client::components::TabContent;
use crate::client::components::TabList;
use crate::client::components::TabTrigger;
use crate::client::components::Tabs;
use crate::user::api::{sign_in, sign_up};

#[derive(Clone, Copy)]
pub struct AuthenticationState {
    authorized: Signal<bool>,
}

/// Renders an action's `CapturedError` as a plain message for the user, without
/// the "error running server function: ... (details: ...)" wrapping.
fn readable_error(err: &CapturedError) -> String {
    match err.0.downcast_ref::<ServerFnError>() {
        Some(ServerFnError::ServerError { message, .. }) => message.clone(),
        _ => "Something went wrong, please try again".to_owned(),
    }
}

#[component]
pub fn AuthView(authorized: Signal<bool>) -> Element {
    let mut tab = use_signal(|| Tab::SignIn);
    rsx! {
        div { class: "flex flex-col w-1/3 min-w-60 justify-self-center",
            div { class: "flex flex-row",
                button {
                    class: "flex-1 rounded-m m-1 px-1 py-2 hover:bg-sky-100 cursor-pointer transition-colors",
                    class: if *tab.read() == Tab::SignIn { "bg-sky-200" } else { "" },
                    onclick: move |_| tab.set(Tab::SignIn),
                    "Sign in"
                }
                button {
                    class: "flex-1 rounded-m m-1 px-1 py-2 hover:bg-sky-100 cursor-pointer transition-colors",
                    class: if *tab.read() == Tab::SignUp { "bg-sky-200" } else { "" },
                    onclick: move |_| tab.set(Tab::SignUp),
                    "Sign up"
                }
            }
            match *tab.read() {
                Tab::SignIn => rsx! {
                    SignInForm { authorized }
                },
                Tab::SignUp => rsx! {
                    SignUpForm { authorized }
                },
            }
        }
    }
}

#[component]
fn SignInForm() -> Element {
    let mut username = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut signin = use_action(move || try_sign_in(username, password, authorized));
    rsx! {
        div { class: "flex flex-col",
            input {
                class: "flex-1 m-1 p-2",
                oninput: move |e: FormEvent| username.set(e.value()),
                placeholder: "Name",
                value: username,
            }
            input {
                class: "flex-1 m-1 p-2",
                oninput: move |e: FormEvent| password.set(e.value()),
                placeholder: "Password",
                r#type: "password",
                value: password,
            }
            button {
                class: "w-1/2 rounded-xl m-1 px-4 py-2 bg-sky-600 text-white hover:bg-sky-700 active:bg-sky-800 cursor-pointer transition-colors self-center",
                onclick: move |_| signin.call(),
                disabled: signin.pending(),
                if signin.pending() {
                    "Checking..."
                } else {
                    "Sign in"
                }
            }
            if let Some(Err(err)) = signin.value() {
                p { "{readable_error(&err)}" }
            }
        }
    }
}

async fn try_sign_in(username: Signal<String>, password: Signal<String>) -> Result<()> {
    sign_in(username.to_string(), password.to_string()).await?;
    use_context::<AuthenticationState>().authorized.set(true);
    Ok(())
}

#[component]
fn SignUpForm() -> Element {
    let mut username = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut signup = use_action(move || try_sign_up(username, password, authorized));
    rsx! {
        div { class: "flex flex-col",
            input {
                class: "flex-1 m-1 p-2",
                oninput: move |e: FormEvent| username.set(e.value()),
                placeholder: "Name",
                value: username,
            }
            input {
                class: "flex-1 m-1 p-2",
                oninput: move |e: FormEvent| password.set(e.value()),
                placeholder: "Password",
                r#type: "password",
                value: password,
            }
            button {
                class: "w-1/2 rounded-xl m-1 px-4 py-2 bg-sky-600 text-white hover:bg-sky-700 active:bg-sky-800 cursor-pointer transition-colors self-center",
                onclick: move |_| signup.call(),
                disabled: signup.pending(),
                if signup.pending() {
                    "Registering..."
                } else {
                    "Sign up"
                }
            }
            if let Some(Err(err)) = signup.value() {
                p { "{readable_error(&err)}" }
            }
        }
    }
}

async fn try_sign_up(username: Signal<String>, password: Signal<String>) -> Result<()> {
    sign_up(username.to_string(), password.to_string()).await?;
    try_sign_in(username, password).await
}
