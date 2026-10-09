pub(crate) mod config;
mod db;

use std::sync::Arc;

use dioxus::prelude::*;
use dioxus_fullstack::routing::Router;
use dioxus_server::axum::Extension;

use crate::{
    server::config::Config,
    ui::app,
    user::server::{auth, users::DynUsers},
};

pub async fn router() -> anyhow::Result<Router> {
    let config = Config::load()?;
    let db = config.postgres().connect().await?;
    db::initialize_postgres(&db).await?;
    let arc = Arc::new(db.clone());
    Ok(dioxus::server::router(app)
        .layer(auth::auth_layer(arc.clone()))
        .layer(auth::auth_session_layer(db.clone()).await?)
        .layer(Extension(arc.clone() as DynUsers)))
}
