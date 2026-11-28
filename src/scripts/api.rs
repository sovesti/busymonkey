use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::{scripts::server::DynScripts, user::server::auth::AxumSession};
#[cfg(feature = "server")]
use dioxus::server::axum::Extension;

#[get("/scripts", auth: AxumSession, Extension(scripts): Extension<DynScripts>)]
pub async fn available_scripts() -> Result<Vec<String>, HttpError> {
    match auth.current_user.as_ref().and_then(|x| x.permission()) {
        Some(permission) => scripts
            .get_available_scripts(permission)
            .await
            .or_internal_server_error("ooops, try again"),
        None => HttpError::unauthorized("Sign in, please"),
    }
}
