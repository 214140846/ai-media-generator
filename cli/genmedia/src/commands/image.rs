use anyhow::{Context, Result, bail};
use serde_json::{Map, Value};
use tokio::time::{Duration, sleep};

use crate::{
    client::ApiClient,
    config::Config,
    request_params::{merge_object, parse_params, reserved_image_keys},
    types::{ImageGenerateResponse, ImageTaskResponse},
};

pub async fn generate(
    model: String,
    prompt: String,
    aspect_ratio: Option<String>,
    response_format: Option<String>,
    images: Vec<String>,
    metadata_json: Option<String>,
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
        body.insert("aspectRatio".to_string(), Value::from(aspect_ratio));
    }
    body.insert(
        "responseFormat".to_string(),
        Value::from(response_format.unwrap_or_else(|| "url".to_string())),
    );
    if !images.is_empty() {
        body.insert(
            "images".to_string(),
            Value::Array(images.into_iter().map(Value::from).collect()),
        );
    }
    if let Some(raw) = metadata_json {
        let metadata: Value = serde_json::from_str(&raw)
            .with_context(|| "invalid --metadata-json, expected JSON value")?;
        body.insert("metadata".to_string(), metadata);
    }

    let extras = parse_params(&params)?;
    let body = merge_object(body, extras, reserved_image_keys())?;
    let response = client.create_image(&body).await?;

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
