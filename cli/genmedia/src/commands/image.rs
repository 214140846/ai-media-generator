use anyhow::{Result, bail};
use serde_json::json;
use tokio::time::{Duration, sleep};

use crate::{
    client::ApiClient,
    config::Config,
    types::{ImageGenerateResponse, ImageTaskResponse},
};

pub async fn generate(
    model: String,
    prompt: String,
    aspect_ratio: Option<String>,
    wait: bool,
    poll_interval: u64,
) -> Result<()> {
    let config = Config::load()?;
    let client = ApiClient::from_config(&config)?;
    let response = client
        .create_image(&json!({
            "model": model,
            "prompt": prompt,
            "aspectRatio": aspect_ratio,
            "responseFormat": "url"
        }))
        .await?;

    if !wait || response.status.as_deref() != Some("processing") {
        println!("{}", serde_json::to_string_pretty(&response)?);
        return Ok(());
    }

    let task = poll_image_task(&client, &response.task_id, poll_interval).await?;
    println!("{}", serde_json::to_string_pretty(&task)?);
    Ok(())
}

pub async fn get(task_id: String, wait: bool, poll_interval: u64) -> Result<()> {
    let config = Config::load()?;
    let client = ApiClient::from_config(&config)?;
    let task = if wait {
        poll_image_task(&client, &task_id, poll_interval).await?
    } else {
        client.get_image_task(&task_id).await?
    };
    println!("{}", serde_json::to_string_pretty(&task)?);
    Ok(())
}

async fn poll_image_task(
    client: &ApiClient,
    task_id: &str,
    poll_interval: u64,
) -> Result<ImageTaskResponse> {
    loop {
        let task = client.get_image_task(task_id).await?;
        match task.status.as_str() {
            "completed" => return Ok(task),
            "failed" => bail!(
                "{}",
                task.error
                    .unwrap_or_else(|| "image task failed".to_string())
            ),
            _ => sleep(Duration::from_secs(poll_interval.max(1))).await,
        }
    }
}

#[allow(dead_code)]
fn _print_initial(response: &ImageGenerateResponse) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(response)?);
    Ok(())
}
