use anyhow::{Result, bail};

use crate::{
    client::ApiClient,
    config::Config,
    types::{ImageModelItem, ModelParameter, VideoModelItem},
};

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
        for parameter in item.parameters {
            print_parameter_summary(&parameter);
        }
    }
    println!();
    println!("Images");
    for item in models.images {
        println!("- {} ({})", item.label, item.name);
        for parameter in item.parameters {
            print_parameter_summary(&parameter);
        }
    }

    Ok(())
}

pub async fn show(model: String, json: bool) -> Result<()> {
    let config = Config::load()?;
    let client = ApiClient::from_config(&config)?;
    let models = client.models().await?;

    if let Some(item) = models.videos.iter().find(|item| item.name == model) {
        return print_video(item, json);
    }
    if let Some(item) = models.images.iter().find(|item| item.name == model) {
        return print_image(item, json);
    }

    bail!("model `{model}` not found");
}

fn print_video(item: &VideoModelItem, json: bool) -> Result<()> {
    if json {
        println!("{}", serde_json::to_string_pretty(item)?);
        return Ok(());
    }

    println!("{} ({})", item.label, item.name);
    println!("kind: {}", item.kind);
    if !item.marks.is_empty() {
        println!("marks: {}", item.marks.join(", "));
    }
    println!("parameters:");
    for parameter in &item.parameters {
        print_parameter_detail(parameter);
    }
    Ok(())
}

fn print_image(item: &ImageModelItem, json: bool) -> Result<()> {
    if json {
        println!("{}", serde_json::to_string_pretty(item)?);
        return Ok(());
    }

    println!("{} ({})", item.label, item.name);
    println!("kind: {}", item.kind.join(", "));
    println!("parameters:");
    for parameter in &item.parameters {
        print_parameter_detail(parameter);
    }
    Ok(())
}

fn print_parameter_summary(parameter: &ModelParameter) {
    let required = if parameter.required.unwrap_or(false) {
        "required"
    } else {
        "optional"
    };
    println!("  - {} [{} {}]", parameter.key, parameter.parameter_type, required);
}

fn print_parameter_detail(parameter: &ModelParameter) {
    let required = if parameter.required.unwrap_or(false) {
        "required"
    } else {
        "optional"
    };
    println!("- {} [{} {}]", parameter.key, parameter.parameter_type, required);
    if let Some(description) = &parameter.description {
        println!("  {}", description);
    }
    if let Some(values) = &parameter.enum_values {
        println!("  enum: {}", serde_json::to_string(values).unwrap_or_default());
    }
    if let Some(default_value) = &parameter.default_value {
        println!("  default: {}", default_value);
    }
    match (parameter.min, parameter.max) {
        (Some(min), Some(max)) => println!("  range: {} - {}", min, max),
        (Some(min), None) => println!("  min: {}", min),
        (None, Some(max)) => println!("  max: {}", max),
        (None, None) => {}
    }
}
