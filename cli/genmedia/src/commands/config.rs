use anyhow::Result;

use crate::config::Config;

pub async fn set_key(key: String) -> Result<()> {
    let mut config = Config::load()?;
    config.api_key = Some(key);
    config.save()?;
    println!("saved API key");
    Ok(())
}

pub async fn set_base_url(base_url: String) -> Result<()> {
    let mut config = Config::load()?;
    config.base_url = Some(base_url.trim_end_matches('/').to_string());
    config.save()?;
    println!("saved base url");
    Ok(())
}

pub async fn show() -> Result<()> {
    let config = Config::load()?;
    println!("{}", serde_json::to_string_pretty(&config)?);
    Ok(())
}
