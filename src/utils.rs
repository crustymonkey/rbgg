use anyhow::{anyhow, Result};
use reqwest;
use serde_json::Value;
use std::collections::HashMap;
use tokio::time::{self, Duration};
use std::thread;
use urlencoding::encode;
use xmltojson::to_json;


// Convenience type
pub type Params = HashMap<String, String>;

pub async fn get_json_resp(url: &str) -> Result<Value> {
    let mut resp;

    // Sometimes, when a large request, often for a user's collection,
    // is made, we'll get a 202 response and we have to request this again
    // after the server has cached it on their side
    loop {
        resp = reqwest::get(url).await?;
        if resp.status() == 202 {
            // We're going to sleep here and try again
            time::sleep(Duration::from_secs(1)).await;
        } else {
            // We should be good to process the response now
            break;
        }
    }

    let data = resp.text().await?;

    let ret = match to_json(&data) {
        Ok(res) => res,
        Err(_) => {
            return Err(anyhow!("Failed to convert to JSON"))
        },
    };

    return Ok(ret);
}

pub fn get_json_resp_b(url: &str) -> Result<Value> {
    let mut resp;

    // Sometimes, when a large request, often for a user's collection,
    // is made, we'll get a 202 response and we have to request this again
    // after the server has cached it on their side
    loop {
        resp = reqwest::blocking::get(url)?;
        if resp.status() == 202 {
            // We're going to sleep here and try again
            thread::sleep(Duration::from_secs(1));
        } else {
            // We should be good to process the response now
            break;
        }
    }

    let data = resp.text()?;

    let ret = match to_json(&data) {
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

/// A convenience function to return params, empty or not from an option
pub fn get_opts(options: Option<Params>) -> Params {
        let mut opts = Params::new();

        if let Some(o) = options {
            opts = o;
        }

        return opts;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_opts() {
        let res = get_opts(None);

        assert_eq!(res, Params::new());

        let p = Params::from([
            ("key".into(), "value".into()),
        ]);
        let res = get_opts(Some(p));

        assert_eq!(res.len(), 1);
        assert!(res.contains_key("key".into()));
    }
}