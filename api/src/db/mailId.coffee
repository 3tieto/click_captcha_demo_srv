#!/usr/bin/env coffee

> @w5/pg/PG.js > ONE0
  @w5/split > rsplit
  @w5/uintb64/uintBin.js
  @w5/uintb64/binUint.js
  @w5/redis/KV.js

MAIL_ID = 'mailId'

< default main = (mail)=>
  mail = mail.toLocaleLowerCase()
  id = await KV.hgetB MAIL_ID, mail
  if id
    console.log {id}
    return binUint id
  li = mail.split '@'
  if li.length != 2
    return 0
  [prefix, domain] = li
  [host,tld] = rsplit domain,'.'
  id = await ONE0"SELECT mail_upsert(#{prefix},#{host},#{tld})"
  await KV.hset(
    MAIL_ID, mail, uintBin id
  )
  id

