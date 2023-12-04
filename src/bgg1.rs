/*!
This is the standard async client for accessing boardgamegeek.com's
Version 1 XML API here: https://boardgamegeek.com/wiki/page/BGG_XML_API

For the given pieces of functionality, you should use the name
corresponding to the given name.  All optional key/value items are
passed in as a HashMap

For blocking (non-async) variants of functions, append "_b" to the name.
e.g. To call a blocking "search", instead call "search_b"

Generally speaking, all of the API calls also take options.  You can supply
these via utils::Params as noted below (with a blocking call).

```ignore,rust
use rbgg::{utils::Params, bgg1};

let cl = bgg1::Client1::new(None, None);
let opts = Params::from([("exact".to_string(), "1".to_string())]);
let resp = cl.search_b("bruges", Some(opts)).unwrap();
```
*/

use anyhow::Result;
use serde_json::Value;
use crate::utils::{self, Params};

/// A representation of a client to hold the url info for accessing the API
pub struct Client1 {
    pub url_base: String,
    pub api_prefix: String,
}

impl Client1 {
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
            prefix = "xmlapi".to_string();
        }

        return Self {
            url_base: ub,
            api_prefix: prefix,
        };
    }

    /// Search for a game on BGG and return the JSON response
    pub async fn search(
        &self,
        search: &str,
        options: Option<Params>,
    ) -> Result<Value> {
        let url = self.get_full_url(
            "search".into(),
            options,
            Some(Params::from([
                ("search".into(), search.into()),
            ])),
            None
        );
        let data = utils::get_json_resp(&url).await?;

        return Ok(data);
    }

    /// (blocking) Search for a game on BGG and return the JSON response
    pub fn search_b(
        &self,
        search: &str,
        options: Option<Params>,
    ) -> Result<Value> {
        let url = self.get_full_url(
            "search".into(),
            options,
            Some(Params::from([
                ("search".into(), search.into()),
            ])),
            None
        );
        let data = utils::get_json_resp_b(&url)?;

        return Ok(data);
    }

    /// Async retrieve information about a particular game given its game ID(s).
    /// Note that you pass in a vec of game IDs here as you can get info on
    /// more than 1 game in a single call
    pub async fn boardgame(
        &self,
        game_ids: &Vec<usize>,
        options: Option<Params>,
    ) -> Result<Value> {
        // Convert the int vec to Vec<&str>
        let ids: Vec<String> = game_ids.iter().map(|i| i.to_string()).collect();
        let url = self.get_full_url(
            "boardgame".into(),
            options,
            None,
            Some(&ids)
        );
        let data = utils::get_json_resp(&url).await?;

        return Ok(data);
    }

    /// Retrieve information about a particular game given its game ID(s).
    /// Note that you pass in a vec of game IDs here as you can get info on
    /// more than 1 game in a single call
    pub fn boardgame_b(&self, game_ids: &Vec<usize>, options: Option<Params>) -> Result<Value> {
        // Convert the int vec to Vec<&str>
        let ids: Vec<String> = game_ids.iter().map(|i| i.to_string()).collect();
        let url = self.get_full_url(
            "boardgame".into(),
            options,
            None,
            Some(&ids),
        );
        let data = utils::get_json_resp_b(&url)?;

        return Ok(data);
    }

    /// Async retrieve a user's collection.  Note that there are a variety of
    /// different parameters that can be used here.
    pub async fn collection(
        &self,
        username: &str,
        options: Option<Params>,
    ) -> Result<Value> {
        let addons = vec![username.to_string()];
        let url = self.get_full_url(
            "collection".into(),
            options,
            None,
            Some(&addons),
        );
        let data = utils::get_json_resp(&url).await?;

        return Ok(data);
    }

    /// Retrieve a user's collection.  Note that there are a variety of
    /// different parameters that can be used here.
    pub fn collection_b(
        &self,
        username: &str,
        options: Option<Params>,
    ) -> Result<Value> {
        let addons = vec![username.to_string()];
        let url = self.get_full_url(
            "collection".into(),
            options,
            None,
            Some(&addons),
        );
        let data = utils::get_json_resp_b(&url)?;

        return Ok(data);
    }

    /// Async get a forum/game thread.  Note that the thread ID is an int
    pub async fn thread(
        &self,
        thread_id: usize,
        options: Option<Params>,
    ) -> Result<Value> {
        let addons = vec![thread_id.to_string()];
        let url = self.get_full_url(
            "thread".into(),
            options,
            None,
            Some(&addons),
        );
        let data = utils::get_json_resp(&url).await?;

        return Ok(data);
    }

    /// Get a forum/game thread.  Note that the thread ID is an int
    pub fn thread_b(
        &self,
        thread_id: usize,
        options: Option<Params>,
    ) -> Result<Value> {
        let addons = vec![thread_id.to_string()];
        let url = self.get_full_url(
            "thread".into(),
            options,
            None,
            Some(&addons),
        );
        let data = utils::get_json_resp_b(&url)?;

        return Ok(data);
    }

    /// Async get a geeklist.  Note that the list ID is an int
    pub async fn geeklist(
        &self,
        list_id: usize,
        options: Option<Params>,
    ) -> Result<Value> {
        let addons = vec![list_id.to_string()];
        let url = self.get_full_url(
            "thread".into(),
            options,
            None,
            Some(&addons),
        );
        let data = utils::get_json_resp(&url).await?;

        return Ok(data);
    }

    /// Get a geeklist.  Note that the list ID is an int
    pub fn geeklist_b(
        &self,
        list_id: usize,
        options: Option<Params>,
    ) -> Result<Value> {
        let addons = vec![list_id.to_string()];
        let url = self.get_full_url(
            "thread".into(),
            options,
            None,
            Some(&addons),
        );
        let data = utils::get_json_resp_b(&url)?;

        return Ok(data);
    }

    /* Begin private functions */

    /// A private function for building a URL given the action that is being
    /// called (like "search"). `uri_addons` are items to be appended to the
    /// url *before* the query string.
    fn gen_url(
        &self,
        path: &str,
        options: Option<Params>,
        uri_addons: Option<&Vec<String>>,
    ) -> String {
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

    /// Another simple private function to get the full url for the purposes
    /// of deduping code between the sync and async functionality
    fn get_full_url(
        &self,
        path: String,
        params: Option<Params>,
        default_params: Option<Params>,
        uri_addons: Option<&Vec<String>>,
    ) -> String {
        let mut opts = utils::get_opts(params);
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
    use serde_json::to_string_pretty;
    use tokio;

    #[test]
    fn test_client() {
        let cl = Client1::new(None, None);

        assert_eq!(cl.url_base, "https://boardgamegeek.com".to_string());
        assert_eq!(cl.api_prefix, "xmlapi".to_string());

        let base = "https://example.com";
        let prefix = "/blah";
        let cl = Client1::new(Some(base.to_string()), Some(prefix.to_string()));

        assert_eq!(cl.url_base, base.to_string());
        assert_eq!(cl.api_prefix, "blah");
    }

    #[test]
    fn test_gen_url() {
        let cl = Client1::new(None, None);
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
    fn test_get_full_url() {
        let cl = Client1::new(None, None);
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
        let cl = Client1::new(None, None);
        let resp = cl.search("bruges", None).await;

        assert!(resp.is_ok());
        println!("{}", to_string_pretty(&resp.unwrap()).unwrap());
    }
}
