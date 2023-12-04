# rbgg (WIP)
This is a library that allows you to conveniently use the boardgamegeek.com
APIs. This is a pretty thin wrapper over the APIs, so with the documentation
on BGG's site and the docs here, you should be able to get up and running
quickly.

API version 1: https://boardgamegeek.com/wiki/page/BGG_XML_API  
API version 2: https://boardgamegeek.com/wiki/page/BGG_XML_API2

# Caveats to Be Aware Of
As of this writing, the library doesn't do things like automatic pagination
collection.  So, if there is more than 1 page of results, it is up to you
to handle this.  The upside is that you have easy access to this data.
