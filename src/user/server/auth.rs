use anyhow::anyhow;
use async_trait::async_trait;
use axum_session::{SessionConfig, SessionLayer, SessionStore};
use axum_session_auth::*;
use axum_session_sqlx::SessionPgPool;
use dioxus::prelude::*;
use dioxus_server::http;
use sqlx::postgres::PgPool;

use super::{
    permission::Permission,
    users::{DynUsers, UserSession},
};

pub type AxumSession = axum_session_auth::AuthSession<UserSession, i32, SessionPgPool, DynUsers>;
pub type AxumAuth = axum_session_auth::AuthSessionLayer<UserSession, i32, SessionPgPool, DynUsers>;

pub fn auth_layer(pool: DynUsers) -> AxumAuth {
    AxumAuth::new(Some(pool))
        .with_config(AuthConfig::<i32>::default().with_anonymous_user_id(Some(ANONYMOUS_ID)))
}

pub async fn auth_session_layer(pool: PgPool) -> anyhow::Result<SessionLayer<SessionPgPool>> {
    Ok(SessionLayer::new(
        SessionStore::<SessionPgPool>::new(
            Some(pool.into()),
            SessionConfig::default().with_table_name("session"),
        )
        .await?,
    ))
}

pub async fn check_permission(
    users: &DynUsers,
    method: http::Method,
    permission: Permission,
    session: &AxumSession,
) -> anyhow::Result<()> {
    Auth::<UserSession, i32, DynUsers>::build([method.clone()], false)
        .requires(Rights::permission(permission))
        .validate(
            session
                .current_user
                .as_ref()
                .ok_or_else(permission_denied)?,
            &method,
            Some(users),
        )
        .await
        .ok_or_else(permission_denied)
}

fn permission_denied() -> anyhow::Error {
    anyhow!("Permission denied")
}

#[async_trait]
impl Authentication<UserSession, i32, DynUsers> for UserSession {
    async fn load_user(userid: i32, pool: Option<&DynUsers>) -> anyhow::Result<UserSession> {
        let pool = pool.ok_or(anyhow!("No conneciton witn DB"))?;
        if userid == ANONYMOUS_ID {
            Ok(UserSession::default())
        } else {
            UserSession::load(userid, pool).await
        }
    }

    fn is_authenticated(&self) -> bool {
        self.permission().is_some()
    }

    fn is_active(&self) -> bool {
        self.permission().is_some()
    }

    fn is_anonymous(&self) -> bool {
        self.permission().is_none()
    }
}

#[async_trait]
impl HasPermission<DynUsers> for UserSession {
    async fn has(&self, perm: &str, _pool: &Option<&DynUsers>) -> bool {
        debug!("checking {perm} of {self:?}");
        self.permission().is_some_and(|p| p.has(perm))
    }
}

pub const ANONYMOUS_ID: i32 = -1;
