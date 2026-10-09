mod monkey;
mod scripts;
#[cfg(feature = "server")]
mod server;
mod ui;
mod user;

fn main() {
    #[cfg(not(feature = "server"))]
    {
        if let Some(host) = option_env!("BUSYMONKEY_HOST") {
            dioxus::fullstack::set_server_url(host);
        }
        dioxus::launch(crate::ui::app);
    }
    #[cfg(feature = "server")]
    dioxus::serve(crate::server::router);
}
