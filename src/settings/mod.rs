use std::{
    io,
    path::{Path, PathBuf},
};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use tokio::fs;

pub fn default_dir() -> Option<PathBuf> {
    ProjectDirs::from("ru", "mkn", "busymonkey").map(|dirs| dirs.config_dir().to_owned())
}

pub async fn load<T: DeserializeOwned + Default>(dir: &Path, name: &str) -> anyhow::Result<T> {
    match fs::read_to_string(file(dir, name)).await {
        Ok(text) => Ok(toml::from_str(&text)?),
        Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(T::default()),
        Err(err) => Err(err.into()),
    }
}

pub async fn save<T: Serialize>(dir: &Path, name: &str, value: &T) -> anyhow::Result<()> {
    fs::create_dir_all(dir).await?;
    fs::write(file(dir, name), toml::to_string(value)?).await?;
    Ok(())
}

fn file(dir: &Path, name: &str) -> PathBuf {
    dir.join(format!("{name}.toml"))
}

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct ScriptSettings {
    pub step_ms: u64,
    pub radius: i32,
}

impl Default for ScriptSettings {
    fn default() -> Self {
        Self {
            step_ms: 20,
            radius: 10,
        }
    }
}

#[derive(Default)]
pub struct Settings {
    pub script: ScriptSettings,
}

impl Settings {
    pub async fn load_all(dir: &Path) -> anyhow::Result<Self> {
        Ok(Self {
            script: load(dir, "script").await?,
        })
    }

    pub async fn save_all(&self, dir: &Path) -> anyhow::Result<()> {
        save(dir, "script", &self.script).await
    }
}
