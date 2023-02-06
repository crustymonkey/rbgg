//! This is the standard async client for accessing boardgamegeek.com's
//! Version 1 XML API here: https://boardgamegeek.com/wiki/page/BGG_XML_API
//! 
//! For the given pieces of functionality, you should use the name
//! corresponding to the given name.  All optional key/value items are
//! passed in as a HashMap

use anyhow::Result;
use serde_json::Value;
use crate::utils::{self, Params};

pub struct Client {
    pub url_base: String,
    pub api_prefix: String,
}

impl Client {
    pub fn new(url_base: Option<String>, api_prefix: Option<String>) -> Self {
        let mut ub = String::new();
        let mut prefix = String::new();

        if let Some(u) = url_base {
            ub = match u.strip_suffix('/') {
                Some(stripped) => stripped.to_string(),
                None => u,
            };
        } else {
            ub = "https://boardgamegeek.com".to_string();
        }

        if let Some(p) = api_prefix {
            prefix = match p.strip_suffix('/') {
                Some(stripped) => stripped.to_string(),
                None => p,
            };
        } else {
            prefix = "/xmlapi".to_string();
        }

        return Self {
            url_base: ub,
            api_prefix: prefix,
        };
    }

    pub async fn search(&self, search: &str, options: Option<Params>) -> Result<Value> {
        let mut opts = Params::new();

        if let Some(o) = options {
            opts = o;
        }
        opts.insert("search".to_string(), search.to_string());

        let full_url = self.gen_url("search", Some(opts));
        let data = utils::get_json_resp(&full_url).await?;

        return Ok(data);
    }

    fn gen_url(&self, path: &str, options: Option<Params>) -> String {
        let mut ret = String::new();
        ret = ret + &self.url_base + "/" + &self.api_prefix + "/" + path + "?";

        if let Some(opts) = options {
            let qs = utils::params2qs(&opts);
            ret.push_str(&qs);
        }

        return ret;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::to_string;
    use tokio;

    #[test]
    fn test_client() {
        let cl = Client::new(None, None);

        assert_eq!(cl.url_base, "https://boardgamegeek.com".to_string());
        assert_eq!(cl.api_prefix, "/xmlapi".to_string());

        let base = "https://example.com";
        let prefix = "/blah";
        let cl = Client::new(Some(base.to_string()), Some(prefix.to_string()));

        assert_eq!(cl.url_base, base.to_string());
        assert_eq!(cl.api_prefix, prefix.to_string());
    }

    #[test]
    fn test_gen_url() {
        let cl = Client::new(None, None);
        let params = Params::from([
            ("search".to_string(), "this is a search".to_string()),
            ("exact".to_string(), "1".to_string()),
        ]);

        let res = cl.gen_url("search", Some(params));
        println!("{}", res);
        assert_eq!(
            res,
            "https://boardgamegeek.com//xmlapi/search?\
                search=this%20is%20a%20search&exact=1".to_string());
    }

    #[tokio::test]
    async fn test_search() {
        let cl = Client::new(None, None);
        let resp = cl.search("bruges", None).await;

        assert!(resp.is_ok());
        println!("{}", to_string(&resp.unwrap()).unwrap());
    }
}
