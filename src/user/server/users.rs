use std::{fmt::Debug, sync::Arc};

use anyhow::{Context, anyhow, ensure};
use async_trait::async_trait;
use dioxus::logger::tracing::debug;
use sqlx::{PgPool, query, query_as};

use super::{User, password::Password, permission::Permission};

#[derive(Clone, Default, Debug)]
pub struct UserSession {
    permission: Option<Permission>,
}

impl UserSession {
    pub async fn load(id: i32, users: &DynUsers) -> anyhow::Result<Self> {
        let perms = users.permission(id).await?;
        Ok(Self {
            permission: Some(perms),
        })
    }

    pub fn permission(&self) -> Option<&Permission> {
        self.permission.as_ref()
    }
}

#[async_trait]
pub trait Users: Debug + Send + Sync + 'static {
    async fn sign_up(&self, name: String, password: String) -> anyhow::Result<User>;
    async fn sign_in(&self, name: String, password: String) -> anyhow::Result<User>;
    async fn find(&self, id: i32) -> anyhow::Result<User>;
    async fn permission(&self, id: i32) -> anyhow::Result<Permission>;
}

pub type DynUsers = Arc<dyn Users>;

#[async_trait]
impl Users for PgPool {
    async fn sign_up(&self, name: String, password: String) -> anyhow::Result<User> {
        ensure!(
            query::<_>("SELECT 0 FROM account WHERE name = $1 LIMIT 1")
                .bind(&name)
                .fetch_optional(self)
                .await?
                .is_none(),
            "name is occupied"
        );
        Ok(
            query_as::<_, User>("INSERT INTO account (name, password) VALUES ($1, $2) RETURNING *")
                .bind(name)
                .bind(Password::new(password).hash())
                .fetch_one(self)
                .await?,
        )
    }

    async fn sign_in(&self, name: String, password: String) -> anyhow::Result<User> {
        let user = query_as::<_, User>("SELECT * FROM account WHERE name = $1")
            .bind(name)
            .fetch_optional(self)
            .await?
            .ok_or(anyhow!("wrong login or password"))?;
        Password::new(password)
            .verify(&user.password)
            .context(anyhow!("wrong login or password"))?;
        debug!("user signed in: {user:?}");
        Ok(user)
    }

    async fn find(&self, id: i32) -> anyhow::Result<User> {
        query_as::<_, User>("SELECT * FROM account WHERE id = $1")
            .bind(id)
            .fetch_optional(self)
            .await?
            .ok_or(anyhow!("no such user"))
    }

    async fn permission(&self, id: i32) -> anyhow::Result<Permission> {
        if let Some(existing) =
            query_as::<_, Permission>("SELECT * FROM account_permission WHERE account = $1")
                .bind(id)
                .fetch_optional(self)
                .await?
        {
            return Ok(existing);
        }
        Ok(query_as::<_, Permission>(
            "INSERT INTO account_permission (account, permission) VALUES ($1, $2) RETURNING *",
        )
        .bind(id)
        .bind(Permission::default().to_string())
        .fetch_one(self)
        .await?)
    }
}

#[cfg(test)]
pub mod test {
    use std::sync::Arc;

    use anyhow::anyhow;
    use async_trait::async_trait;
    use tokio::sync::Mutex;

    use super::{super::permission::Permission, User, Users};

    #[derive(Default, Debug)]
    pub struct FakeUsers {
        pub users: Vec<User>,
        pub permissions: Vec<(i32, Permission)>,
    }

    #[async_trait]
    impl Users for Arc<Mutex<FakeUsers>> {
        async fn sign_up(&self, name: String, password: String) -> anyhow::Result<User> {
            let id = self.lock().await.users.len() as i32;
            let value = User { id, name, password };
            self.lock().await.users.push(value.clone());
            Ok(value)
        }

        async fn sign_in(&self, name: String, password: String) -> anyhow::Result<User> {
            self.lock()
                .await
                .users
                .iter()
                .find(|u| u.name == name && u.password == password)
                .cloned()
                .ok_or(anyhow!("no user"))
        }

        async fn find(&self, id: i32) -> anyhow::Result<User> {
            self.lock()
                .await
                .users
                .iter()
                .find(|u| u.id == id)
                .cloned()
                .ok_or(anyhow!("no user"))
        }

        async fn permission(&self, id: i32) -> anyhow::Result<Permission> {
            let permissions = &mut self.lock().await.permissions;
            permissions.push((id, Permission::default()));
            Ok(permissions
                .iter()
                .find(|&(user, _)| id == *user)
                .cloned()
                .unwrap()
                .1)
        }
    }
}
