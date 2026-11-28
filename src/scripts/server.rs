use crate::user::server::permission::Permission;
use async_trait::async_trait;
use sqlx::{PgPool, query_scalar};
use std::sync::Arc;

#[async_trait]
pub trait Scripts: Send + Sync {
    async fn get_available_scripts(&self, permission: &Permission) -> anyhow::Result<Vec<String>>;
}

pub type DynScripts = Arc<dyn Scripts>;
#[async_trait]
impl Scripts for PgPool {
    async fn get_available_scripts(&self, permission: &Permission) -> anyhow::Result<Vec<String>> {
        query_scalar::<_, String>("select name from script where permission = 'basic' or (permission = 'licensed' and $1 = 'licensed')")
            .bind(permission.permission.clone()).fetch_all(self).await.map_err(anyhow::Error::from)
    }
}
