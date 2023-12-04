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

use anyhow::{anyhow, Result};
use serde_json::Value;
use crate::utils::{self, Params};
use std::fmt;

/// This is used mainly for raw thing() calls
pub enum Thing {
    BoardGame,
    BoardGameExpansion,
    BoardGameAccessory,
    VideoGame,
    RpgItem,
    RpgIssue,
}

impl Thing {
    pub fn as_str(&self) -> &'static str {
        return match self {
            Thing::BoardGame => "boardgame",
            Thing::BoardGameExpansion => "boardgameexpansion",
            Thing::BoardGameAccessory => "boardgameaccessory",
            Thing::VideoGame => "videogame",
            Thing::RpgItem => "rpgitem",
            Thing::RpgIssue => "rpgissue",
        };
    }

    pub fn to_string(&self) -> String {
        return self.as_str().to_string();
    }
}

impl fmt::Display for Thing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}", self.as_str());
    }
}

/// This is used for search() calls
pub enum Search {
    BoardGame,
    BoardGameExpansion,
    BoardGameAccessory,
    VideoGame,
    RpgItem,
}

impl Search {
    pub fn as_str(&self) -> &'static str {
        return match self {
            Search::BoardGame => "boardgame",
            Search::BoardGameExpansion => "boardgameexpansion",
            Search::BoardGameAccessory => "boardgameaccessory",
            Search::VideoGame => "videogame",
            Search::RpgItem => "rpgitem",
        };
    }

    pub fn to_string(&self) -> String {
        return self.as_str().to_string();
    }
}

impl fmt::Display for Search {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}", self.as_str());
    }
}

/// This is for use with the raw family() call
pub enum Family {
    Rpg,
    RpgPeriodical,
    BoardGameFamily,
}

impl Family {
    pub fn as_str(&self) -> &'static str {
        return match self {
            Family::Rpg => "rpg",
            Family::RpgPeriodical => "rpgperiodical",
            Family::BoardGameFamily => "boardgamefamily",
        };
    }

    pub fn to_string(&self) -> String {
        return self.as_str().to_string();
    }
}

impl fmt::Display for Family {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}", self.as_str());
    }
}

/// This is for use with some calls
pub enum ThingFamily {
    Thing,
    Family,
}

impl ThingFamily {
    pub fn as_str(&self) -> &'static str {
        return match self {
            ThingFamily::Thing => "thing",
            ThingFamily::Family => "family",
        };
    }

    pub fn to_string(&self) -> String {
        return self.as_str().to_string();
    }
}

impl fmt::Display for ThingFamily {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}", self.as_str());
    }
}

pub enum Hotness {
    BoardGame,
    Rpg,
    VideoGame,
    BoardGamePerson,
    RpgPerson,
    BoardGameCompany,
    RpgCompany,
    VideoGameCompany,
}

impl Hotness {
    pub fn as_str(&self) -> &'static str {
        return match self {
            Hotness::BoardGame => "boardgame",
            Hotness::Rpg => "rpg",
            Hotness::VideoGame => "videogame",
            Hotness::BoardGamePerson => "boardgameperson",
            Hotness::RpgPerson => "rpgperson",
            Hotness::BoardGameCompany => "boardgamecompany",
            Hotness::RpgCompany => "rpgcompany",
            Hotness::VideoGameCompany => "videogamecompany",
        };
    }

    pub fn to_string(&self) -> String {
        return self.as_str().to_string();
    }
}

impl fmt::Display for Hotness {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}", self.as_str());
    }
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

    /// Search (async) the site for the given query and search types
    pub async fn search(
        &self,
        query: &str,
        stypes: &Vec<Search>,
        options: Option<Params>,
    ) -> Result<Value> {
        let params = Params::from([
            ("query".into(), query.into()),
            (
                "type".into(),
                stypes
                    .iter()
                    .map(|t| t.as_str())
                    .collect::<Vec<&'static str>>()
                    .join(","),
            ),
        ]);

        let url = self.get_full_url("search".into(), options, Some(params));

        let data = utils::get_json_resp(&url).await?;

        return Ok(data);
    }

    /// Search (async) the site for the given query and search types
    pub fn search_b(
        &self,
        query: &str,
        stypes: &Vec<Search>,
        options: Option<Params>,
    ) -> Result<Value> {
        let params = Params::from([
            ("query".into(), query.into()),
            (
                "type".into(),
                stypes
                    .iter()
                    .map(|t| t.as_str())
                    .collect::<Vec<&'static str>>()
                    .join(","),
            ),
        ]);

        let url = self.get_full_url("search".into(), options, Some(params));

        let data = utils::get_json_resp_b(&url)?;

        return Ok(data);
    }

    /* Begin "thing"s */

    /// This is the core (async) function for getting various "things" as
    /// described by the BGG API.  It's also possible to use the convenience
    /// functions like `boardgame()` instead, which will set the thing type
    /// for you.
    pub async fn thing(
        &self,
        ids: &Vec<usize>,
        ttypes: &Vec<Thing>,
        options: Option<Params>,
    ) -> Result<Value> {
        // Convert the numeric ids to strings
        let sids: Vec<String> = ids.iter().map(|i| i.to_string()).collect();
        let params = Params::from([
            ("id".into(), sids.join(",")),
            (
                "type".into(),
                ttypes
                    .iter()
                    .map(|t| t.as_str())
                    .collect::<Vec<&'static str>>()
                    .join(","),
            ),
        ]);
        let url = self.get_full_url("thing".into(), options, Some(params));

        let data = utils::get_json_resp(&url).await?;

        return Ok(data);
    }

    /// This is the core (sync) function for getting various "things" as
    /// described by the BGG API.  It's also possible to use the convenience
    /// functions like `boardgame()` instead, which will set the thing type
    /// for you.
    pub fn thing_b(
        &self,
        ids: &Vec<usize>,
        ttypes: &Vec<Thing>,
        options: Option<Params>,
    ) -> Result<Value> {
        // Convert the numeric ids to strings
        let sids: Vec<String> = ids.iter().map(|i| i.to_string()).collect();
        let params = Params::from([
            ("id".into(), sids.join(",")),
            (
                "type".into(),
                ttypes
                    .iter()
                    .map(|t| t.as_str())
                    .collect::<Vec<&'static str>>()
                    .join(","),
            ),
        ]);
        let url = self.get_full_url("thing".into(), options, Some(params));

        let data = utils::get_json_resp_b(&url)?;

        return Ok(data);
    }

    /// A (async) convenience function for getting the info for a board game
    pub async fn boardgame(
        &self,
        ids: &Vec<usize>,
        options: Option<Params>,
    ) -> Result<Value> {
        return self.thing(ids, &vec![Thing::BoardGame], options).await;
    }

    /// A (sync) convenience function for getting the info for a board game
    pub fn boardgame_b(
        &self,
        ids: &Vec<usize>,
        options: Option<Params>,
    ) -> Result<Value> {
        return self.thing_b(ids, &vec![Thing::BoardGame], options);
    }

    /// A (async) convenience function for getting the info for a board game
    /// expansion
    pub async fn boardgameexpansion(
        &self,
        ids: &Vec<usize>,
        options: Option<Params>,
    ) -> Result<Value> {
        return self.thing(ids, &vec![Thing::BoardGameExpansion], options).await;
    }

    /// A (sync) convenience function for getting the info for a board game
    /// expansion
    pub fn boardgameexpansion_b(
        &self,
        ids: &Vec<usize>,
        options: Option<Params>,
    ) -> Result<Value> {
        return self.thing_b(ids, &vec![Thing::BoardGameExpansion], options);
    }

    /// A (async) convenience function for getting the info for a board game
    /// accessory
    pub async fn boardgameaccessory(
        &self,
        ids: &Vec<usize>,
        options: Option<Params>,
    ) -> Result<Value> {
        return self.thing(ids, &vec![Thing::BoardGameAccessory], options).await;
    }

    /// A (sync) convenience function for getting the info for a board game
    /// accessory
    pub fn boardgameaccessory_b(
        &self,
        ids: &Vec<usize>,
        options: Option<Params>,
    ) -> Result<Value> {
        return self.thing_b(ids, &vec![Thing::BoardGameAccessory], options);
    }

    /// A (async) convenience function for getting the info for a video game
    pub async fn videogame(
        &self,
        ids: &Vec<usize>,
        options: Option<Params>,
    ) -> Result<Value> {
        return self.thing(ids, &vec![Thing::VideoGame], options).await;
    }

    /// A (sync) convenience function for getting the info for a video game
    pub fn videogame_b(
        &self,
        ids: &Vec<usize>,
        options: Option<Params>,
    ) -> Result<Value> {
        return self.thing_b(ids, &vec![Thing::VideoGame], options);
    }

    /// A (async) convenience function for getting the info for a rpg item
    pub async fn rpgitem(
        &self,
        ids: &Vec<usize>,
        options: Option<Params>,
    ) -> Result<Value> {
        return self.thing(ids, &vec![Thing::RpgItem], options).await;
    }

    /// A (sync) convenience function for getting the info for a rpg item
    pub fn rpgitem_b(
        &self,
        ids: &Vec<usize>,
        options: Option<Params>,
    ) -> Result<Value> {
        return self.thing_b(ids, &vec![Thing::RpgItem], options);
    }

    /// A (async) convenience function for getting the info for a rpg issue
    pub async fn rpgissue(
        &self,
        ids: &Vec<usize>,
        options: Option<Params>,
    ) -> Result<Value> {
        return self.thing(ids, &vec![Thing::RpgIssue], options).await;
    }

    /// A (sync) convenience function for getting the info for a rpg issue
    pub fn rpgissue_b(
        &self,
        ids: &Vec<usize>,
        options: Option<Params>,
    ) -> Result<Value> {
        return self.thing_b(ids, &vec![Thing::RpgIssue], options);
    }

    /* End "thing"s */

    /* Begin "family" items */

    /// This is the core (async) function for getting various "family" items as
    /// described by the BGG API.  It's also possible to use the convenience
    /// functions like `rpg()` instead, which will set the thing type
    /// for you.
    pub async fn family(&self, ids: &Vec<usize>, ttypes: &Vec<Family>) -> Result<Value> {
        // Convert the numeric ids to strings
        let sids: Vec<String> = ids
            .iter()
            .map(|i| i.to_string())
            .collect();
        let params = Params::from([
            ("id".into(), sids.join(",")),
            (
                "type".into(),
                ttypes
                    .iter()
                    .map(|t| t.as_str())
                    .collect::<Vec<&'static str>>()
                    .join(","),
            ),
        ]);
        let url = self.get_full_url("family".into(), None, Some(params));

        let data = utils::get_json_resp(&url).await?;

        return Ok(data);
    }

    /// This is the core (sync) function for getting various "family" items as
    /// described by the BGG API.  It's also possible to use the convenience
    /// functions like `rpg()` instead, which will set the thing type
    /// for you.
    pub fn family_b(&self, ids: &Vec<usize>, ttypes: &Vec<Family>) -> Result<Value> {
        // Convert the numeric ids to strings
        let sids: Vec<String> = ids
            .iter()
            .map(|i| i.to_string())
            .collect();
        let params = Params::from([
            ("id".into(), sids.join(",")),
            (
                "type".into(),
                ttypes
                    .iter()
                    .map(|t| t.as_str())
                    .collect::<Vec<&'static str>>()
                    .join(","),
            ),
        ]);
        let url = self.get_full_url("family".into(), None, Some(params));

        let data = utils::get_json_resp_b(&url)?;

        return Ok(data);
    }

    /// A (async) convenience function for getting the info for a rpg
    pub async fn rpg(&self, ids: &Vec<usize>) -> Result<Value> {
        return self.family(ids, &vec![Family::Rpg]).await;
    }

    /// A (sync) convenience function for getting the info for a rpg
    pub fn rpg_b(&self, ids: &Vec<usize>) -> Result<Value> {
        return self.family_b(ids, &vec![Family::Rpg]);
    }

    /// A (async) convenience function for getting the info for a rpg
    /// periodical
    pub async fn rpgperiodical(&self, ids: &Vec<usize>) -> Result<Value> {
        return self.family(ids, &vec![Family::RpgPeriodical]).await;
    }

    /// A (sync) convenience function for getting the info for a rpg
    /// periodical
    pub fn rpgperiodical_b(&self, ids: &Vec<usize>) -> Result<Value> {
        return self.family_b(ids, &vec![Family::RpgPeriodical]);
    }

    /// A (async) convenience function for getting the info for a board game
    /// family
    pub async fn boardgamefamily(&self, ids: &Vec<usize>) -> Result<Value> {
        return self.family(ids, &vec![Family::BoardGameFamily]).await;
    }

    /// A (sync) convenience function for getting the info for a board game
    /// family
    pub fn boardgamefamily_b(&self, ids: &Vec<usize>) -> Result<Value> {
        return self.family_b(ids, &vec![Family::BoardGameFamily]);
    }

    /* End "family" items */

    /// Get a (async) list of forums for a given game (by ID)
    pub async fn forumlist(
        &self,
        game_id: usize,
        ltype: ThingFamily,
    ) -> Result<Value> {
        let params = Params::from([
            ("id".into(), game_id.to_string()),
            ("type".into(), ltype.to_string()),
        ]);
        let url = self.get_full_url("forumlist".into(), None, Some(params));

        let data = utils::get_json_resp(&url).await?;

        return Ok(data);
    }

    /// Get a (sync) list of forums for a given game (by ID)
    pub fn forumlist_b(
        &self,
        game_id: usize,
        ltype: ThingFamily
    ) -> Result<Value> {
        let params = Params::from([
            ("id".into(), game_id.to_string()),
            ("type".into(), ltype.to_string()),
        ]);
        let url = self.get_full_url("forumlist".into(), None, Some(params));

        let data = utils::get_json_resp_b(&url)?;

        return Ok(data);
    }

    /// Get a (async) list of threads in a particular forum by forum ID
    pub async fn forum(
        &self,
        forum_id: usize,
        options: Option<Params>,
    ) -> Result<Value> {
        let params = Params::from([
            ("id".into(), forum_id.to_string()),
        ]);
        let url = self.get_full_url("forumlist".into(), options, Some(params));

        let data = utils::get_json_resp(&url).await?;

        return Ok(data);
    }

    /// Get a (sync) list of threads in a particular forum by forum ID
    pub async fn forum_b(
        &self,
        forum_id: usize,
        options: Option<Params>,
    ) -> Result<Value> {
        let params = Params::from([
            ("id".into(), forum_id.to_string()),
        ]);
        let url = self.get_full_url("forumlist".into(), options, Some(params));

        let data = utils::get_json_resp_b(&url)?;

        return Ok(data);
    }

    /// Get a (async) thread by ID
    pub async fn thread(
        &self,
        thread_id: usize,
        options: Option<Params>,
    ) -> Result<Value> {
        let params = Params::from([
            ("id".into(), thread_id.to_string()),
        ]);
        let url = self.get_full_url("thread".into(), options, Some(params));

        let data = utils::get_json_resp(&url).await?;

        return Ok(data);
    }

    /// Get a (sync) list of threads in a particular forum by forum ID
    pub fn thread_b(
        &self,
        thread_id: usize,
        options: Option<Params>,
    ) -> Result<Value> {
        let params = Params::from([
            ("id".into(), thread_id.to_string()),
        ]);
        let url = self.get_full_url("thread".into(), options, Some(params));

        let data = utils::get_json_resp_b(&url)?;

        return Ok(data);
    }

    /// Get a (async) user by their username
    pub async fn user(
        &self,
        username: &str,
        options: Option<Params>,
    ) -> Result<Value> {
        let params = Params::from([
            ("name".into(), username.into()),
        ]);
        let url = self.get_full_url("user".into(), options, Some(params));

        let data = utils::get_json_resp(&url).await?;

        return Ok(data);
    }

    /// Get a (sync) user by their username
    pub fn user_b(
        &self,
        username: &str,
        options: Option<Params>,
    ) -> Result<Value> {
        let params = Params::from([
            ("name".into(), username.into()),
        ]);
        let url = self.get_full_url("user".into(), options, Some(params));

        let data = utils::get_json_resp_b(&url)?;

        return Ok(data);
    }

    /// Get a (async) guild by ID
    pub async fn guild(
        &self,
        guild_id: usize,
        options: Option<Params>,
    ) -> Result<Value> {
        let params = Params::from([
            ("id".into(), guild_id.to_string()),
        ]);
        let url = self.get_full_url("guild".into(), options, Some(params));

        let data = utils::get_json_resp(&url).await?;

        return Ok(data);
    }

    /// Get a (async) guild by ID
    pub fn guild_b(
        &self,
        guild_id: usize,
        options: Option<Params>,
    ) -> Result<Value> {
        let params = Params::from([
            ("id".into(), guild_id.to_string()),
        ]);
        let url = self.get_full_url("guild".into(), options, Some(params));

        let data = utils::get_json_resp_b(&url)?;

        return Ok(data);
    }

    /// Get (async) plays for a user or for a particular item. Either a
    /// username or item ID + ttype MUST be supplied
    pub async fn plays(
        &self,
        username: Option<&str>,
        item_id: Option<usize>,
        ttype: Option<ThingFamily>,
        options: Option<Params>,
    ) -> Result<Value> {
        if username.is_none() && (item_id.is_none() || ttype.is_none()) {
            // TODO: Replace with custom error type
            return Err(anyhow!("You must supply either a username or item_id + ttype"));
        }

        if username.is_some() && item_id.is_some() {
            // TODO: Replace with custom error type
            return Err(anyhow!("You must supply either a username or item_id + ttype, not both"));
        }

        let params;
        if let Some(u) = username {
            params = Params::from([
                ("username".into(), u.into()),
            ]);
        } else if let Some(id) = item_id {
            params = Params::from([
                ("id".into(), id.to_string()),
                ("type".into(), ttype.unwrap().to_string()),
            ]);
        } else {
            // We should never get here
            return Err(anyhow!("We have a logic bug here as this should never happen"));
        }

        let url = self.get_full_url("plays".into(), options, Some(params));

        let data = utils::get_json_resp(&url).await?;

        return Ok(data);
    }

    /// Get (sync) plays for a user or for a particular item. Either a
    /// username or item ID + ttype MUST be supplied
    pub fn plays_b(
        &self,
        username: Option<&str>,
        item_id: Option<usize>,
        ttype: Option<ThingFamily>,
        options: Option<Params>,
    ) -> Result<Value> {
        if username.is_none() && (item_id.is_none() || ttype.is_none()) {
            // TODO: Replace with custom error type
            return Err(anyhow!("You must supply either a username or item_id + ttype"));
        }

        if username.is_some() && item_id.is_some() {
            // TODO: Replace with custom error type
            return Err(anyhow!("You must supply either a username or item_id + ttype, not both"));
        }

        let params;
        if let Some(u) = username {
            params = Params::from([
                ("username".into(), u.into()),
            ]);
        } else if let Some(id) = item_id {
            params = Params::from([
                ("id".into(), id.to_string()),
                ("type".into(), ttype.unwrap().to_string()),
            ]);
        } else {
            // We should never get here
            return Err(anyhow!("We have a logic bug here as this should never happen"));
        }

        let url = self.get_full_url("plays".into(), options, Some(params));

        let data = utils::get_json_resp_b(&url)?;

        return Ok(data);
    }

    /// Get a (async) user's collection by username
    pub async fn collection(&self, username: &str, options: Option<Params>) -> Result<Value> {
        let params = Params::from([
            ("username".into(), username.into()),
        ]);
        let url = self.get_full_url("collection".into(), options, Some(params));

        let data = utils::get_json_resp(&url).await?;

        return Ok(data);
    }

    /// Get a (sync) user's collection by username
    pub fn collection_b(&self, username: &str, options: Option<Params>) -> Result<Value> {
        let params = Params::from([
            ("username".into(), username.into()),
        ]);
        let url = self.get_full_url("collection".into(), options, Some(params));

        let data = utils::get_json_resp_b(&url)?;

        return Ok(data);
    }

    /// Get (async) the latest hotness on BGG
    pub async fn hot(&self, htype: Hotness) -> Result<Value> {
        let params = Params::from([
            ("type".into(), htype.to_string()),
        ]);
        let url = self.get_full_url("hot".into(), None, Some(params));

        let data = utils::get_json_resp(&url).await?;

        return Ok(data);
    }

    /// Get (sync) the latest hotness on BGG
    pub async fn hot_b(&self, htype: Hotness) -> Result<Value> {
        let params = Params::from([
            ("type".into(), htype.to_string()),
        ]);
        let url = self.get_full_url("hot".into(), None, Some(params));

        let data = utils::get_json_resp_b(&url)?;

        return Ok(data);
    }

    /* Begin private functions */

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
    use serde_json::to_string_pretty;

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

        let res = cl.gen_url("search", Some(params));
        println!("{}", res);
        // Since a Hashmap isn't ordered, the order of the params and how they
        // end up in the url is not deterministic.  We just have to test
        // that parts of the url are correct
        assert!(res.starts_with("https://boardgamegeek.com/xmlapi2/search?"));
        assert!(res.contains("search=this%20is%20a%20search"));
        assert!(res.contains("&"));
        assert!(res.contains("exact=1"));

        let res = cl.gen_url("boardgame", None);

        assert_eq!(res, "https://boardgamegeek.com/xmlapi2/boardgame?".to_string());
    }

    #[test]
    fn test_get_full_url() {
        let cl = Client2::new(None, None);
        let url = cl.get_full_url(
            "search".to_string(),
            None,
            Some(Params::from([
                ("search".into(), "this is a search".into()),
            ])),
        );

        assert_eq!(url, "https://boardgamegeek.com/xmlapi2/search?search=this%20is%20a%20search");

        let url = cl.get_full_url(
            "boardgame".into(),
            Some(Params::from([("comments".into(), "1".into())])),
            None,
        );

        assert_eq!(url, "https://boardgamegeek.com/xmlapi2/boardgame?comments=1".to_string());
    }

    #[tokio::test]
    async fn test_search() {
        let cl = Client2::new(None, None);
        let params = Params::from([("exact".into(), "1".into())]);
        let resp = cl.search("burges", &vec![Search::BoardGame], Some(params)).await;

        assert!(resp.is_ok());
        println!("{}", to_string_pretty(&resp.unwrap()).unwrap());
    }
}
