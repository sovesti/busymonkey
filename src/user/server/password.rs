pub struct Password {
    raw: String,
}

impl Password {
    pub fn new(raw: String) -> Self {
        Self { raw }
    }

    pub fn hash(&self) -> String {
        password_auth::generate_hash(&self.raw)
    }

    pub fn verify(&self, hash: &str) -> anyhow::Result<()> {
        Ok(password_auth::verify_password(&self.raw, hash)?)
    }
}
