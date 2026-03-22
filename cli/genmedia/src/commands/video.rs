use anyhow::{Result, bail};
use serde_json::json;
use tokio::time::{Duration, sleep};

use crate::{client::ApiClient, config::Config, types::VideoTaskResponse};

pub async fn generate(
    model: String,
    prompt: String,
    aspect_ratio: Option<String>,
    duration: Option<u64>,
    wait: bool,
    poll_interval: u64,
) -> Result<()> {
    let config = Config::load()?;
    let client = ApiClient::from_config(&config)?;
    let response = client
        .create_video(&json!({
            "model": model,
            "prompt": prompt,
            "aspect_ratio": aspect_ratio,
            "duration": duration
        }))
        .await?;

    if !wait {
        println!("{}", serde_json::to_string_pretty(&response)?);
        return Ok(());
    }

    let task = poll_video_task(&client, &response.task_id, poll_interval).await?;
    println!("{}", serde_json::to_string_pretty(&task)?);
    Ok(())
}

pub async fn get(task_id: String, wait: bool, poll_interval: u64) -> Result<()> {
    let config = Config::load()?;
    let client = ApiClient::from_config(&config)?;
    let task = if wait {
        poll_video_task(&client, &task_id, poll_interval).await?
    } else {
        client.get_video_task(&task_id).await?
    };
    println!("{}", serde_json::to_string_pretty(&task)?);
    Ok(())
}

async fn poll_video_task(
    client: &ApiClient,
    task_id: &str,
    poll_interval: u64,
) -> Result<VideoTaskResponse> {
    loop {
        let task = client.get_video_task(task_id).await?;
        match task.status.as_str() {
            "SUCCESS" => return Ok(task),
            "FAILURE" => bail!(
                "{}",
                if task.fail_reason.is_empty() {
                    "video task failed"
                } else {
                    &task.fail_reason
                }
            ),
            _ => sleep(Duration::from_secs(poll_interval.max(1))).await,
        }
    }
}
