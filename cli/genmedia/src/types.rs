use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct ModelsResponse {
    pub videos: Vec<VideoModelItem>,
    pub images: Vec<ImageModelItem>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VideoModelItem {
    pub id: String,
    pub name: String,
    pub label: String,
    pub kind: String,
    #[serde(default)]
    pub marks: Vec<String>,
    #[serde(default)]
    pub credits: Vec<Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ImageModelItem {
    pub id: String,
    pub name: String,
    pub label: String,
    pub kind: Vec<String>,
    pub vendor: String,
    #[serde(rename = "priceTier")]
    pub price_tier: Option<String>,
    #[serde(rename = "creditCost")]
    pub credit_cost: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ImageGenerateResponse {
    #[serde(rename = "taskId")]
    pub task_id: String,
    pub status: Option<String>,
    pub model: Option<String>,
    pub vendor: Option<String>,
    #[serde(default)]
    pub results: Vec<ImageAsset>,
    pub metadata: Option<Value>,
    pub credits: Option<CreditUsage>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ImageAsset {
    pub url: Option<String>,
    #[serde(rename = "r2Url")]
    pub r2_url: Option<String>,
    #[serde(rename = "providerUrl")]
    pub provider_url: Option<String>,
    pub base64: Option<String>,
    #[serde(rename = "mimeType")]
    pub mime_type: Option<String>,
    pub width: Option<u64>,
    pub height: Option<u64>,
    pub index: Option<u64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreditUsage {
    pub required: u64,
    pub deducted: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ImageTaskResponse {
    #[serde(rename = "taskId")]
    pub task_id: String,
    pub status: String,
    #[serde(default)]
    pub results: Vec<ImageAsset>,
    pub error: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
    pub metadata: Option<Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VideoCreateResponse {
    pub task_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VideoTaskResponse {
    pub task_id: String,
    pub platform: String,
    pub action: String,
    pub status: String,
    pub fail_reason: String,
    pub submit_time: Option<Value>,
    pub start_time: Option<Value>,
    pub finish_time: Option<Value>,
    pub progress: String,
    pub data: VideoTaskData,
    pub search_item: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VideoTaskData {
    pub output: Option<String>,
}
