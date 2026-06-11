/*!
This has some low level conveience functions, but what you will want to use
here is the `Params` type, which is just a shorthand for
HashMap<String, String>.
*/
use anyhow::{anyhow, Result};
use reqwest;
use serde_json::Value;
use std::collections::HashMap;
use std::thread;
use tokio::time::{self, Duration};
use urlencoding::encode;
use xmltojson::to_json;

/// Convenience type that is just a shorthand for a HashMap
pub type Params = HashMap<String, String>;

/// This performs the actual request to the BGG API and returns the JSON
/// response.
pub async fn get_json_resp(url: &str, api_key: &str) -> Result<Value> {
    let mut resp;

    // Sometimes, when a large request, often for a user's collection, // is made, we'll get a 202 response and we have to request this again
    // after the server has cached it on their side
    loop {
        let client = get_req_builder(url, api_key);
        resp = client.send().await?;
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
        Err(_) => return Err(anyhow!("Failed to convert to JSON")),
    };

    return Ok(ret);
}

/// This performs the request to the BGG API and returns the JSON response,
/// but in a blocking way.
pub fn get_json_resp_b(url: &str, api_key: &str) -> Result<Value> {
    let mut resp;

    // Sometimes, when a large request, often for a user's collection,
    // is made, we'll get a 202 response and we have to request this again
    // after the server has cached it on their side
    loop {
        let client = get_req_builder_b(url, api_key);
        resp = client.send()?;
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
        }
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

/// Get the request builder with the auth header set
pub fn get_req_builder(url: &str, api_key: &str) -> reqwest::RequestBuilder {
    let client = reqwest::Client::new();

    return client
        .get(url)
        .bearer_auth(api_key);
}

/// Get the blocking variant of the request builder with the auth header set
pub fn get_req_builder_b(url: &str, api_key: &str) -> reqwest::blocking::RequestBuilder {
    let client = reqwest::blocking::Client::new();

    return client
        .get(url)
        .bearer_auth(api_key);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_opts() {
        let res = get_opts(None);

        assert_eq!(res, Params::new());

        let p = Params::from([("key".into(), "value".into())]);
        let res = get_opts(Some(p));

        assert_eq!(res.len(), 1);
        assert!(res.contains_key("key".into()));
    }
    #[test]
    fn test_params_encoding() {
        // Basic test
        let p = Params::from([("key".into(), "value".into())]);
        assert_eq!(params2qs(&p), "key=value");

        // Test for the url encoding
        let p = Params::from([("key=".into(), "value".into())]);
        assert_eq!(params2qs(&p), "key%3D=value");

        // Due to the lack of stable ordering in a hash map, I have to do some
        // different style of checks here.
        let p = Params::from([
            ("key1".into(), "value1".into()),
            ("key2".into(), "value2".into()),
        ]);
        let res = params2qs(&p);
        let amp_count = res.chars().filter(|c| *c == '&').count();
        assert_eq!(amp_count, 1);
        assert!(res.contains("key1=value1"));
        assert!(res.contains("key2=value2"));
    }
}
