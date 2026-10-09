use sqlx::{Executor, PgPool};

use crate::user::server::users::Users;

pub async fn initialize_postgres(pool: &PgPool) -> anyhow::Result<()> {
    pool.execute(include_str!("initialize.sql")).await?;
    let _ = pool
        .sign_up("admin".to_owned(), "password".to_owned())
        .await;
    Ok(())
}
