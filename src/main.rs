mod client;
mod monkey;
mod scripts;
#[cfg(feature = "server")]
mod server;
mod user;

fn main() {
    #[cfg(not(feature = "server"))]
    {
        if let Some(host) = option_env!("BUSYMONKEY_HOST") {
            dioxus::fullstack::set_server_url(host);
        }
        dioxus::launch(crate::client::app);
    }
    #[cfg(feature = "server")]
    dioxus::serve(crate::server::router);
}
