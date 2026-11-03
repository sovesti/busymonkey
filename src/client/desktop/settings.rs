use std::path::PathBuf;
use std::{fmt::Debug, io};

use anyhow::anyhow;
use dioxus::prelude::*;
use directories::ProjectDirs;
use serde::{Serialize, de::DeserializeOwned};
use tokio::fs;

pub async fn load_or_default<T: DeserializeOwned + Default + Debug>(name: String) -> T {
    in_project_dir(|dir| load(dir, &name))
        .await
        .map_err(|err| warn!("failed to load settings: {err:?}"))
        .unwrap_or_default()
}

pub async fn try_save<T: Serialize>(name: String, value: &T) -> anyhow::Result<()> {
    in_project_dir(|dir| save(dir, &name, value)).await
}

async fn in_project_dir<T>(f: impl AsyncFnOnce(PathBuf) -> anyhow::Result<T>) -> anyhow::Result<T> {
    let dir = default_dir().ok_or(anyhow!("failed to obtain default app directory"))?;
    f(dir.clone()).await.with_context(|| format!("{dir:?}"))
}

fn default_dir() -> Option<PathBuf> {
    ProjectDirs::from("ru", "mkn", "busymonkey").map(|dirs| dirs.config_dir().to_owned())
}

async fn load<T: DeserializeOwned + Default>(dir: PathBuf, name: &str) -> anyhow::Result<T> {
    match fs::read_to_string(file(dir, name)).await {
        Ok(text) => Ok(toml::from_str(&text)?),
        Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(T::default()),
        Err(err) => Err(err.into()),
    }
}

async fn save<T: Serialize>(dir: PathBuf, name: &str, value: &T) -> anyhow::Result<()> {
    fs::create_dir_all(&dir).await?;
    fs::write(file(dir, name), toml::to_string(value)?).await?;
    Ok(())
}

fn file(dir: PathBuf, name: &str) -> PathBuf {
    dir.join(format!("{name}.toml"))
}
