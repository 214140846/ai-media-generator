use std::{fs, path::PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

const DEFAULT_BASE_URL: &str = "http://127.0.0.1:3000";

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub base_url: Option<String>,
    pub api_key: Option<String>,
}

impl Config {
    pub fn load() -> Result<Self> {
        let path = config_path()?;
        if !path.exists() {
            return Ok(Self::default());
        }

        let raw = fs::read_to_string(&path)
            .with_context(|| format!("failed to read config: {}", path.display()))?;
        let config = serde_json::from_str::<Self>(&raw)
            .with_context(|| format!("failed to parse config: {}", path.display()))?;
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let path = config_path()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("failed to create config dir: {}", parent.display()))?;
        }
        let raw = serde_json::to_string_pretty(self)?;
        fs::write(&path, raw)
            .with_context(|| format!("failed to write config: {}", path.display()))?;
        Ok(())
    }

    pub fn resolved_base_url(&self) -> String {
        std::env::var("AI_MEDIA_BASE_URL")
            .ok()
            .filter(|value| !value.trim().is_empty())
            .or_else(|| self.base_url.clone())
            .unwrap_or_else(|| DEFAULT_BASE_URL.to_string())
            .trim_end_matches('/')
            .to_string()
    }

    pub fn resolved_api_key(&self) -> Option<String> {
        std::env::var("AI_MEDIA_API_KEY")
            .ok()
            .filter(|value| !value.trim().is_empty())
            .or_else(|| self.api_key.clone())
    }
}

pub fn config_path() -> Result<PathBuf> {
    let base = dirs::config_dir().context("could not resolve config directory")?;
    Ok(base.join("ai-media").join("config.json"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Mutex, OnceLock};

    fn env_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

    #[test]
    fn resolved_base_url_prefers_ai_media_env() {
        let _guard = env_lock().lock().unwrap_or_else(|error| error.into_inner());
        unsafe {
            std::env::set_var("AI_MEDIA_BASE_URL", "https://example.com/");
        }

        let config = Config {
            base_url: Some("https://config.example.com".to_string()),
            api_key: None,
        };

        assert_eq!(config.resolved_base_url(), "https://example.com");

        unsafe {
            std::env::remove_var("AI_MEDIA_BASE_URL");
        }
    }

    #[test]
    fn resolved_api_key_prefers_ai_media_env() {
        let _guard = env_lock().lock().unwrap_or_else(|error| error.into_inner());
        unsafe {
            std::env::set_var("AI_MEDIA_API_KEY", "gm_env");
        }

        let config = Config {
            base_url: None,
            api_key: Some("gm_config".to_string()),
        };

        assert_eq!(config.resolved_api_key().as_deref(), Some("gm_env"));

        unsafe {
            std::env::remove_var("AI_MEDIA_API_KEY");
        }
    }

    #[test]
    fn config_path_uses_ai_media_directory() {
        let _guard = env_lock().lock().unwrap_or_else(|error| error.into_inner());
        let path = config_path().expect("config path");
        assert_eq!(path.file_name().and_then(|value| value.to_str()), Some("config.json"));
        assert_eq!(
            path.parent()
                .and_then(|value| value.file_name())
                .and_then(|value| value.to_str()),
            Some("ai-media")
        );
    }
}
