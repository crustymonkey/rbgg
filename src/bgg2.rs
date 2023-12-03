/*!
This is the standard async client for accessing boardgamegeek.com's
Version 2 XML API here: https://boardgamegeek.com/wiki/page/BGG_XML_API2

For the given pieces of functionality, you should use the name
corresponding to the given name.

For blocking (non-async) variants of functions, append "_b" to the name.
e.g. To call a blocking "search", instead call "search_b"

Generally speaking, all of the API calls also take options.  You can supply
these via utils::Params (Hashmap) as noted below (with a blocking call).

```ignore,rust
use bgg::{utils::Params, bgg1};

let cl = bgg1::Client::new(None, None);
let opts = Params::from([("exact".to_string(), "1".to_string())]);
let resp = cl.search_b("bruges", Some(opts)).unwrap();
```
*/

use anyhow::Result;
use serde_json::Value;
use crate::utils::{self, Params};

/// A representation of a client to hold the url info for accessing the API
pub struct Client2 {
    pub url_base: String,
    pub api_prefix: String,
}

impl Client2 {
    /// If the url_base or api_prefix are not supplied, the defaults will be
    /// used instead ("https://boardgamegeek.com" and "xmlapi", respectively)
    pub fn new(url_base: Option<String>, api_prefix: Option<String>) -> Self {
        let ub;
        let prefix;

        if let Some(u) = url_base {
            ub = match u.strip_suffix('/') {
                Some(stripped) => stripped.to_string(),
                None => u,
            };
        } else {
            ub = "https://boardgamegeek.com".to_string();
        }

        if let Some(p) = api_prefix {
            prefix = p.as_str().trim_matches('/').to_string();
        } else {
            prefix = "xmlapi2".to_string();
        }

        return Self {
            url_base: ub,
            api_prefix: prefix,
        };
    }

    /* Begin private functions */

    /// A private function for building a URL given the action that is being
    /// called (like "search"). `uri_addons` are items to be appended to the
    /// url *before* the query string.
    fn gen_url(&self, path: &str, options: Option<Params>, uri_addons: Option<&Vec<String>>) -> String {
        let mut ret = String::new();
        ret = ret + &self.url_base + "/" + &self.api_prefix + "/" + path;

        if let Some(addons) = uri_addons {
            ret = ret + "/" + &addons.join(",");
        }
        ret += "?";
        

        if let Some(opts) = options {
            let qs = utils::params2qs(&opts);
            ret.push_str(&qs);
        }

        return ret;
    }

    /// A simple private function that returns a Params instance regardless of
    /// whether any were passed in.
    fn get_opts(&self, options: Option<Params>) -> Params {
        let mut opts = Params::new();

        if let Some(o) = options {
            opts = o;
        }

        return opts;
    }

    /// Another simple private function to get the full url for the purposes
    /// of deduping code between the sync and async functionality
    fn get_full_url(
        &self,
        path: String,
        params: Option<Params>,
        default_params: Option<Params>,
        uri_addons: Option<&Vec<String>>,
    ) -> String {
        let mut opts = self.get_opts(params);
        // Add the default options
        if let Some(def_params) = default_params {
            for (k, v) in &def_params {
                opts.insert(k.into(), v.into());
            }
        }

        let url = self.gen_url(&path, Some(opts), uri_addons);

        return url;
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
        assert_eq!(cl.api_prefix, "xmlapi".to_string());

        let base = "https://example.com";
        let prefix = "/blah";
        let cl = Client::new(Some(base.to_string()), Some(prefix.to_string()));

        assert_eq!(cl.url_base, base.to_string());
        assert_eq!(cl.api_prefix, "blah");
    }

    #[test]
    fn test_gen_url() {
        let cl = Client::new(None, None);
        let params = Params::from([
            ("search".to_string(), "this is a search".to_string()),
            ("exact".to_string(), "1".to_string()),
        ]);

        let res = cl.gen_url("search", Some(params), None);
        println!("{}", res);
        // Since a Hashmap isn't ordered, the order of the params and how they
        // end up in the url is not deterministic.  We just have to test
        // that parts of the url are correct
        assert!(res.starts_with("https://boardgamegeek.com/xmlapi/search?"));
        assert!(res.contains("search=this%20is%20a%20search"));
        assert!(res.contains("&"));
        assert!(res.contains("exact=1"));

        let res = cl.gen_url("boardgame", None, Some(&vec!["1".into(), "2".into()]));

        assert_eq!(res, "https://boardgamegeek.com/xmlapi/boardgame/1,2?".to_string());
    }
    #[test]
    fn test_get_opts() {
        let cl = Client::new(None, None);
        let res = cl.get_opts(None);

        assert!(res.is_empty());

        let p = Params::from([
            ("key".into(), "value".into()),
        ]);
        let res = cl.get_opts(Some(p.clone()));

        assert_eq!(res.len(), 1);
        assert!(res.contains_key("key".into()));
    }

    #[test]
    fn test_get_full_url() {
        let cl = Client::new(None, None);
        let url = cl.get_full_url(
            "search".to_string(),
            None,
            Some(Params::from([
                ("search".into(), "this is a search".into()),
            ]), ),
            None,
        );

        assert_eq!(url, "https://boardgamegeek.com/xmlapi/search?search=this%20is%20a%20search");

        let url = cl.get_full_url(
            "boardgame".into(),
            Some(Params::from([("comments".into(), "1".into())])),
            None,
            Some(&vec!["a".into(), "b".into(), "c".into()]),
        );

        assert_eq!(url, "https://boardgamegeek.com/xmlapi/boardgame/a,b,c?comments=1".to_string());
    }

    #[tokio::test]
    async fn test_search() {
        let cl = Client::new(None, None);
        let resp = cl.search("bruges", None).await;

        assert!(resp.is_ok());
        println!("{}", to_string(&resp.unwrap()).unwrap());
    }
}
