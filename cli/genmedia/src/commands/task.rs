use anyhow::Result;

use crate::commands::{image, video};

pub async fn get(kind: String, task_id: String, wait: bool, poll_interval: u64) -> Result<()> {
    match kind.as_str() {
        "image" => image::get(task_id, wait, poll_interval).await,
        "video" => video::get(task_id, wait, poll_interval).await,
        _ => unreachable!("clap validates task kind"),
    }
}
