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

let cl = bgg2::Client2::new(None, None);
let opts = Params::from([("exact".to_string(), "1".to_string())]);
let resp = cl.search_b("bruges", Some(opts)).unwrap();
```
*/

use anyhow::Result;
use serde_json::Value;
use crate::{utils::{self, Params}, errors::InvalidBGGType};
use std::collections::HashSet;

lazy_static! {
    static ref THINGS: HashSet<&'static str> = HashSet::from([
        "boardgame",
        "boardgameexpansion",
        "boardgameaccessory",
        "videogame",
        "rpgitem",
        "rpgissue",
    ]);

    static ref FAMILIES: HashSet<&'static str> = HashSet::from([
        "rpg",
        "rpgperiodical",
        "boardgamefamily",
    ]);
}

/// A representation of a client to hold the url info for accessing the API
pub struct Client2 {
    pub url_base: String,
    pub api_prefix: String,
}

impl Client2 {
    /// If the url_base or api_prefix are not supplied, the defaults will be
    /// used instead ("https://boardgamegeek.com" and "xmlapi2", respectively)
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

    /* Begin "thing"s */

    /// This is the core (async) function for getting various "things" as
    /// described by the BGG API.  It's also possible to use the convenience
    /// functions like `boardgame()` instead, which will set the thing type
    /// for you.
    pub async fn thing(&self, ids: &Vec<usize>, ttypes: &Vec<String>, options: Option<Params>) -> Result<Value> {
        self.validate_types(ttypes, &THINGS)?;

        // Convert the numeric ids to strings
        let sids: Vec<String> = ids.iter().map(|i| i.to_string()).collect();
        let params = Params::from([
            ("id".into(), sids.join(",")),
            ("type".into(), ttypes.join(",")),
        ]);
        let url = self.get_full_url("thing".into(), options, Some(params));

        let data = utils::get_json_resp(&url).await?;

        return Ok(data);
    }

    /// This is the core (async) function for getting various "things" as
    /// described by the BGG API.  It's also possible to use the convenience
    /// functions like `boardgame()` instead, which will set the thing type
    /// for you.
    pub fn thing_b(&self, ids: &Vec<usize>, ttypes: &Vec<String>, options: Option<Params>) -> Result<Value> {
        self.validate_types(ttypes, &THINGS)?;

        // Convert the numeric ids to strings
        let sids: Vec<String> = ids.iter().map(|i| i.to_string()).collect();
        let params = Params::from([
            ("id".into(), sids.join(",")),
            ("type".into(), ttypes.join(",")),
        ]);
        let url = self.get_full_url("thing".into(), options, Some(params));

        let data = utils::get_json_resp_b(&url)?;

        return Ok(data);
    }

    /// A (async) convenience function for getting the info for a board game
    pub async fn boardgame(&self, ids: &Vec<usize>, options: Option<Params>) -> Result<Value> {
        return self.thing(ids, &vec!["boardgame".into()], options).await;
    }

    /// A (sync) convenience function for getting the info for a board game
    pub fn boardgame_b(&self, ids: &Vec<usize>, options: Option<Params>) -> Result<Value> {
        return self.thing_b(ids, &vec!["boardgame".into()], options);
    }

    /* End "thing"s */

    /* Begin private functions */

    fn validate_types(&self, ttypes: &Vec<String>, valid: &HashSet<&'static str>) -> Result<(), InvalidBGGType> {
        for t in ttypes {
            if !valid.contains(t.as_str()) {
                return Err(InvalidBGGType::new(t, "Invalid thing type"));
            }
        }

        return Ok(());
    }

    /// A private function for building a URL given the action that is being
    /// called (like "search"). `uri_addons` are items to be appended to the
    /// url *before* the query string.
    fn gen_url(&self, path: &str, options: Option<Params>) -> String {
        let mut ret = String::new();
        ret = ret + &self.url_base + "/" + &self.api_prefix + "/" + path + "?";

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
    ) -> String {
        let mut opts = utils::get_opts(params);
        // Add the default options
        if let Some(def_params) = default_params {
            for (k, v) in &def_params {
                opts.insert(k.into(), v.into());
            }
        }

        let url = self.gen_url(&path, Some(opts));

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
        let cl = Client2::new(None, None);

        assert_eq!(cl.url_base, "https://boardgamegeek.com".to_string());
        assert_eq!(cl.api_prefix, "xmlapi2".to_string());

        let base = "https://example.com";
        let prefix = "/blah";
        let cl = Client2::new(Some(base.to_string()), Some(prefix.to_string()));

        assert_eq!(cl.url_base, base.to_string());
        assert_eq!(cl.api_prefix, "blah");
    }

    #[test]
    fn test_gen_url() {
        let cl = Client2::new(None, None);
        let params = Params::from([
            ("search".to_string(), "this is a search".to_string()),
            ("exact".to_string(), "1".to_string()),
        ]);

        let res = cl.gen_url("search", Some(params), None);
        println!("{}", res);
        // Since a Hashmap isn't ordered, the order of the params and how they
        // end up in the url is not deterministic.  We just have to test
        // that parts of the url are correct
        assert!(res.starts_with("https://boardgamegeek.com/xmlapi2/search?"));
        assert!(res.contains("search=this%20is%20a%20search"));
        assert!(res.contains("&"));
        assert!(res.contains("exact=1"));

        let res = cl.gen_url("boardgame", None, Some(&vec!["1".into(), "2".into()]));

        assert_eq!(res, "https://boardgamegeek.com/xmlapi2/boardgame/1,2?".to_string());
    }

    #[test]
    fn test_get_full_url() {
        let cl = Client2::new(None, None);
        let url = cl.get_full_url(
            "search".to_string(),
            None,
            Some(Params::from([
                ("search".into(), "this is a search".into()),
            ]), ),
            None,
        );

        assert_eq!(url, "https://boardgamegeek.com/xmlapi2/search?search=this%20is%20a%20search");

        let url = cl.get_full_url(
            "boardgame".into(),
            Some(Params::from([("comments".into(), "1".into())])),
            None,
            Some(&vec!["a".into(), "b".into(), "c".into()]),
        );

        assert_eq!(url, "https://boardgamegeek.com/xmlapi2/boardgame/a,b,c?comments=1".to_string());
    }

    #[test]
    fn test_validate_types() {
        let cl = Client2::new(None, None);
        let ttypes = vec!["boardgame".into(), "bogus".into()];

        let res = cl.validate_types(&ttypes, &THINGS);
        if let Err(e) = &res {
            println!("Error: {}", e);
        }
        assert!(res.is_err());

        let ttypes = vec!["boardgame".into(), "rpgitem".into()];

        let res = cl.validate_types(&ttypes, &THINGS);
        assert!(res.is_ok());
    }
}
