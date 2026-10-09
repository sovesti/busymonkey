use std::{env, fs::exists};

use anyhow::{Context, anyhow, ensure};
use serde::Deserialize;
use sqlx::{PgPool, postgres::PgPoolOptions};

#[derive(Deserialize)]
pub struct Config {
    postgres: PostgresConfig,
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        Ok(config::Config::builder()
            .add_source(config::File::with_name(&config_path()?))
            .build()?
            .try_deserialize()?)
    }

    pub fn postgres(&self) -> &PostgresConfig {
        &self.postgres
    }
}

fn config_path() -> anyhow::Result<String> {
    let path = env::args().nth(1).unwrap_or("config.toml".to_owned());
    ensure!(
        exists(&path).is_ok_and(|exists| exists),
        "Config file not found at {path}"
    );
    Ok(path)
}

#[derive(Deserialize)]
pub struct PostgresConfig {
    user: String,
    password: String,
    database: String,
    host: String,
    port: u16,
}

impl PostgresConfig {
    pub fn url(&self) -> String {
        format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.database
        )
    }

    pub async fn connect(&self) -> anyhow::Result<PgPool> {
        self.try_connect()
            .await
            .with_context(|| anyhow!("Failed to connect to Postgres: {}", self.url()))
    }

    async fn try_connect(&self) -> anyhow::Result<PgPool> {
        Ok(PgPoolOptions::new()
            .connect_with(self.url().parse()?)
            .await?)
    }
}
