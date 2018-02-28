#!/usr/bin/env bash
set -x
curl -X POST --header 'Content-Type: application/json' -H 'X-API-Key: notthekey' -d '{ "msg": "We will not see this." }' 'http://localhost:8888/echo'
:
curl -X POST --header 'Content-Type: application/json' -H 'X-API-Key: techtalkdemosecret' -d '{ "msg": "We will see this again soon." }' 'http://localhost:8888/echo'
