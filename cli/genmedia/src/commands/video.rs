use anyhow::{Result, bail};
use serde_json::{Map, Value};
use tokio::time::{Duration, sleep};

use crate::{
    client::ApiClient,
    config::Config,
    request_params::{merge_object, parse_params, reserved_video_keys},
    types::VideoTaskResponse,
};

pub async fn generate(
    model: String,
    prompt: String,
    aspect_ratio: Option<String>,
    duration: Option<u64>,
    images: Vec<String>,
    params: Vec<String>,
    wait: bool,
    poll_interval: u64,
) -> Result<()> {
    let config = Config::load()?;
    let client = ApiClient::from_config(&config)?;
    let mut body = Map::new();
    body.insert("model".to_string(), Value::from(model));
    body.insert("prompt".to_string(), Value::from(prompt));
    if let Some(aspect_ratio) = aspect_ratio {
        body.insert("aspect_ratio".to_string(), Value::from(aspect_ratio));
    }
    if let Some(duration) = duration {
        body.insert("duration".to_string(), Value::from(duration));
    }
    if !images.is_empty() {
        body.insert(
            "images".to_string(),
            Value::Array(images.into_iter().map(Value::from).collect()),
        );
    }

    let extras = parse_params(&params)?;
    let body = merge_object(body, extras, reserved_video_keys())?;
    let response = client.create_video(&body).await?;

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
