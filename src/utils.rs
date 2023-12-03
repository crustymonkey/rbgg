use anyhow::{anyhow, Result};
use reqwest;
use serde_json::Value;
use std::collections::HashMap;
use urlencoding::encode;
use xmltojson::to_json;


// Convenience type
pub type Params = HashMap<String, String>;

pub async fn get_json_resp(url: &str) -> Result<Value> {
    let resp = reqwest::get(url).await?.text().await?;
    let ret = match to_json(&resp) {
        Ok(res) => res,
        Err(_) => {
            return Err(anyhow!("Failed to convert to JSON"))
        },
    };

    return Ok(ret);
}

pub fn get_json_resp_b(url: &str) -> Result<Value> {
    let resp = reqwest::blocking::get(url)?.text()?;
    let ret = match to_json(&resp) {
        Ok(res) => res,
        Err(_) => {
            return Err(anyhow!("Failed to convert to JSON"));
        },
    };

    return Ok(ret);
}

/// Convert a set of Params into a query string
pub fn params2qs(params: &Params) -> String {
    let mut parts = vec![];

    for (k, v) in params {
        let mut tmp = String::new();

        tmp += &encode(k).to_owned();
        tmp.push_str("=");
        tmp += &encode(v).to_owned();

        parts.push(tmp);
    }

    return parts.join("&");
}