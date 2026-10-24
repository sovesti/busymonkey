use std::{
    io,
    path::{Path, PathBuf},
};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use tokio::fs;

pub trait SettingsCategory: Serialize + DeserializeOwned + Default {
    const NAME: &'static str;
}

pub fn default_dir() -> Option<PathBuf> {
    ProjectDirs::from("ru", "mkn", "busymonkey").map(|dirs| dirs.config_dir().to_owned())
}

pub async fn load<T: SettingsCategory>(dir: &Path) -> anyhow::Result<T> {
    match fs::read_to_string(file::<T>(dir)).await {
        Ok(text) => Ok(toml::from_str(&text)?),
        Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(T::default()),
        Err(err) => Err(err.into()),
    }
}

pub async fn save<T: SettingsCategory>(dir: &Path, value: &T) -> anyhow::Result<()> {
    fs::create_dir_all(dir).await?;
    fs::write(file::<T>(dir), toml::to_string(value)?).await?;
    Ok(())
}

fn file<T: SettingsCategory>(dir: &Path) -> PathBuf {
    dir.join(format!("{}.toml", T::NAME))
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

impl SettingsCategory for ScriptSettings {
    const NAME: &'static str = "script";
}

#[derive(Default)]
pub struct Settings {
    pub script: ScriptSettings,
}

impl Settings {
    pub async fn load_all(dir: &Path) -> anyhow::Result<Self> {
        Ok(Self {
            script: load(dir).await?,
        })
    }

    pub async fn save_all(&self, dir: &Path) -> anyhow::Result<()> {
        save(dir, &self.script).await
    }
}
