use std::fmt::{self, Display};

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct Permission {
    pub(crate) permission: String,
}

impl Default for Permission {
    fn default() -> Self {
        Self {
            permission: "basic".to_owned(),
        }
    }
}

impl Display for Permission {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.permission)
    }
}

impl From<Permission> for String {
    fn from(value: Permission) -> Self {
        value.to_string()
    }
}

impl Permission {
    pub fn has(&self, permission: &str) -> bool {
        self.permission.contains(permission)
    }
}
