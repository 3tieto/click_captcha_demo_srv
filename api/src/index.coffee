#!/usr/bin/env coffee

> @8n/srv
  ./db/mailId.js

srv {
  test: (body)=>
    console.log {body},'>>'
    console.log await mailId 'i18n.site@GMAIL.COM'
    # consolelog 'done'
    '123'

}
