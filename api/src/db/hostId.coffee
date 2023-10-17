#!/usr/bin/env coffee

> @w5/pg/PG.js > ONE0
  @w5/split > rsplit
  @w5/uintb64/uintBin.js
  ./KV.js

< (host)=>
  host = host.toLocaleLowerCase()
  [val,tld] = rsplit host,'.'
  if not tld
    return 0
  id = await ONE0"SELECT host_upsert(#{val},#{tld})"
  id
