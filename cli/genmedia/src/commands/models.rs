use anyhow::Result;

use crate::{client::ApiClient, config::Config};

pub async fn list(json: bool) -> Result<()> {
    let config = Config::load()?;
    let client = ApiClient::from_config(&config)?;
    let models = client.models().await?;

    if json {
        println!("{}", serde_json::to_string_pretty(&models)?);
        return Ok(());
    }

    println!("Videos");
    for item in models.videos {
        println!("- {} ({})", item.label, item.name);
    }
    println!();
    println!("Images");
    for item in models.images {
        println!("- {} ({})", item.label, item.name);
    }

    Ok(())
}
