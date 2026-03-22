use anyhow::{Result, bail};
use serde_json::{Map, Value};

const RESERVED_IMAGE_KEYS: &[&str] = &[
    "model",
    "prompt",
    "aspectRatio",
    "responseFormat",
    "images",
    "metadata",
];
const RESERVED_VIDEO_KEYS: &[&str] = &["model", "prompt", "aspect_ratio", "duration", "images"];

pub fn parse_key_value(input: &str) -> Result<(String, Value)> {
    let (key, raw_value) = input
        .split_once('=')
        .ok_or_else(|| anyhow::anyhow!("invalid --param value `{input}`, expected KEY=VALUE"))?;

    let key = key.trim();
    if key.is_empty() {
        bail!("invalid --param value `{input}`, key cannot be empty");
    }

    let value = parse_value(raw_value.trim());
    Ok((key.to_string(), value))
}

pub fn parse_params(values: &[String]) -> Result<Map<String, Value>> {
    let mut params = Map::new();
    for raw in values {
        let (key, value) = parse_key_value(raw)?;
        if params.contains_key(&key) {
            bail!("duplicate --param key `{key}`");
        }
        params.insert(key, value);
    }
    Ok(params)
}

pub fn merge_object(
    mut base: Map<String, Value>,
    extras: Map<String, Value>,
    reserved_keys: &[&str],
) -> Result<Value> {
    for key in extras.keys() {
        if reserved_keys.contains(&key.as_str()) {
            bail!("`--param {key}=...` conflicts with a built-in CLI field");
        }
    }

    for (key, value) in extras {
        if base.contains_key(&key) {
            bail!("`--param {key}=...` conflicts with a built-in CLI field");
        }
        base.insert(key, value);
    }

    Ok(Value::Object(base))
}

pub fn reserved_image_keys() -> &'static [&'static str] {
    RESERVED_IMAGE_KEYS
}

pub fn reserved_video_keys() -> &'static [&'static str] {
    RESERVED_VIDEO_KEYS
}

fn parse_value(raw: &str) -> Value {
    serde_json::from_str(raw).unwrap_or_else(|_| Value::String(raw.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_numbers_booleans_and_strings() {
        let (key, value) = parse_key_value("duration=8").expect("parse");
        assert_eq!(key, "duration");
        assert_eq!(value, Value::from(8));

        let (_, value) = parse_key_value("enhance_prompt=true").expect("parse");
        assert_eq!(value, Value::from(true));

        let (_, value) = parse_key_value("size=1024x1024").expect("parse");
        assert_eq!(value, Value::from("1024x1024"));
    }

    #[test]
    fn parses_json_values() {
        let (_, value) = parse_key_value(r#"images=["a","b"]"#).expect("parse");
        assert_eq!(value, Value::from(vec!["a", "b"]));
    }

    #[test]
    fn merges_params_and_rejects_reserved_fields() {
        let mut base = Map::new();
        base.insert("model".to_string(), Value::from("veo3"));

        let extras = parse_params(&["enhance_prompt=true".to_string()]).expect("params");
        let merged = merge_object(base, extras, reserved_video_keys()).expect("merge");
        assert_eq!(merged["enhance_prompt"], Value::from(true));

        let extras = parse_params(&["model=bad".to_string()]).expect("params");
        let err = merge_object(Map::new(), extras, reserved_video_keys()).unwrap_err();
        assert!(err.to_string().contains("conflicts"));
    }
}
