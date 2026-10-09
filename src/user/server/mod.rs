pub(crate) mod auth;
mod password;
pub(crate) mod permission;
pub(crate) mod users;

#[derive(Clone, Debug, sqlx::FromRow)]
pub struct User {
    id: i32,
    name: String,
    password: String,
}

impl User {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
