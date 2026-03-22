use anyhow::{Context, Result, anyhow, bail};
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};
use serde::Serialize;

use crate::{
    config::Config,
    types::{
        ImageGenerateResponse, ImageTaskResponse, ModelsResponse, VideoCreateResponse,
        VideoTaskResponse,
    },
};

pub struct ApiClient {
    base_url: String,
    client: reqwest::Client,
}

impl ApiClient {
    pub fn from_config(config: &Config) -> Result<Self> {
        let base_url = config.resolved_base_url();
        let api_key = config.resolved_api_key().ok_or_else(|| {
            anyhow!("missing API key, run `ai-media config set-key <gm_key>` first")
        })?;

        let mut headers = HeaderMap::new();
        headers.insert(
            "x-api-key",
            HeaderValue::from_str(&api_key).context("invalid api key header")?,
        );
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {api_key}"))
                .context("invalid authorization header")?,
        );

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .context("failed to build HTTP client")?;

        Ok(Self { base_url, client })
    }

    pub async fn models(&self) -> Result<ModelsResponse> {
        self.get_json("/api/agent/models").await
    }

    pub async fn create_image<T: Serialize>(&self, body: &T) -> Result<ImageGenerateResponse> {
        self.post_json("/api/image/generate", body).await
    }

    pub async fn get_image_task(&self, task_id: &str) -> Result<ImageTaskResponse> {
        self.get_json(&format!("/api/image/tasks/{task_id}")).await
    }

    pub async fn create_video<T: Serialize>(&self, body: &T) -> Result<VideoCreateResponse> {
        self.post_json("/api/v2/videos/generations", body).await
    }

    pub async fn get_video_task(&self, task_id: &str) -> Result<VideoTaskResponse> {
        self.get_json(&format!("/api/v2/videos/generations/{task_id}"))
            .await
    }

    async fn get_json<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T> {
        let response = self
            .client
            .get(format!("{}{}", self.base_url, path))
            .send()
            .await
            .with_context(|| format!("GET {} failed", path))?;
        parse_json(response).await
    }

    async fn post_json<TReq: Serialize, TRes: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        body: &TReq,
    ) -> Result<TRes> {
        let response = self
            .client
            .post(format!("{}{}", self.base_url, path))
            .json(body)
            .send()
            .await
            .with_context(|| format!("POST {} failed", path))?;
        parse_json(response).await
    }
}

async fn parse_json<T: serde::de::DeserializeOwned>(response: reqwest::Response) -> Result<T> {
    let status = response.status();
    let text = response
        .text()
        .await
        .context("failed to read response body")?;
    if !status.is_success() {
        let message = serde_json::from_str::<serde_json::Value>(&text)
            .ok()
            .and_then(|json| {
                json.get("error")
                    .and_then(|value| value.as_str())
                    .map(ToString::to_string)
            })
            .unwrap_or_else(|| text.clone());
        bail!("request failed with {}: {}", status, message);
    }
    serde_json::from_str::<T>(&text)
        .with_context(|| format!("failed to parse JSON response: {text}"))
}
