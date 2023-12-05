/*!
# rbgg
This is a library that allows you to conveniently use the boardgamegeek.com
APIs. This is a pretty thin wrapper over the APIs, so with the documentation
on BGG's site and the docs here, you should be able to get up and running
quickly.

API version 1: https://boardgamegeek.com/wiki/page/BGG_XML_API  
API version 2: https://boardgamegeek.com/wiki/page/BGG_XML_API2

The basics of this library are pretty simple. You make a call that matches up
with the API, like [search](https://t.brk.io/2I) after creating a client and
you get a
[serde_json::Value](https://docs.rs/serde_json/latest/serde_json/#operating-on-untyped-json-values)
response that contains the converted XML.

## Blocking and Async are supported
The other items to be aware of in this library is that `async` calls are the
default, but blocking calls are supported by simply appending "_b" to the end
of the method name.

For example, if you want to call the `search()` method, here are the ways
in which you would do this.

```rust
use rbgg::bgg2::*;

// There's also a Client2::new() that allows you to change root url and
// API path, but unless you have some specfic use case, you want to use
// the defaults.
let client = Client2::new_from_defaults();
// Calling the search function async. I'll note that all results, both async
// and sync, will be `Result<Value>`
let result = client.search("bruges", &vec![Search::BoardGame], None).await?;

// Similarly, calling it using a blocking call
let result = client.search_b("bruges", &vec![Search::BoardGame], None)?;
```

## API v2
While API v1 tracks pretty much exactly the [documentation on BGG's
site](https://boardgamegeek.com/wiki/page/BGG_XML_API) and, technically, so
does [API v2](https://boardgamegeek.com/wiki/page/BGG_XML_API), but with
some additional convenience methods.

For example, the [thing](https://t.brk.io/WW) API allows you to specify IDs
and one or more things to get.  While you're welcome to use that raw API, each
of the "things" has it's own direct call.  Here are a couple of examples,
first using the direct thing API, then the `boardgame()` convenience method.

```rust
use rbgg::{bgg2::*, utils::Params};

let client = Client2::new_from_defaults();
// You can set any of the parameters for the call using the `Params` in the
// utils lib.
let params = Params::from([
  ("comments".into(), "1".into()),
  ("stats".into(), "1".into()),
]);
// You can retrieve more than 1 item at a time
let game_ids = vec![136888, 133473];
let ttypes = vec![Thing::BoardGame];

// We'll use the blocking call in this example
let res = client.thing_b(&game_ids, &ttypes, Some(params));

// Alternatively, you can implicitly just use the "thing" type of boardgame.
// Here is the same call with the convenience function.
let res = client.boardgame_b(&game_ids, Some(params));
```

There are similar methods for all of the [family items](https://t.brk.io/j4) as
well.

Beyond that, you are pretty much just following what the docs say on BGG's
site as that's what the library implements.  Happy gaming!

## Caveats to Be Aware Of
* The library doesn't do things like automatic pagination
  collection.  So, if there is more than 1 page of results, it is up to you
  to handle this.  The upside is that you have easy access to this data.
* If there is an error in the response itself, it is up to you to handle that
  in the JSON response. It will look something like this:

```json
{
  "error": {
    "message": "Rate limit exceeded."
  }
}
```
 */
extern crate xmltojson;
extern crate reqwest;
extern crate serde_json;
extern crate urlencoding;

pub mod bgg1;
pub mod bgg2;
pub mod utils;
