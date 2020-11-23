#!/usr/bin/env bash
set -x
export GOPATH=$PWD/gopath

# export DEBUG=0
export API_HOST="${API_HOST:-localhost:8888}"
export API_KEY="techtalkdemosecret"
export API_MSG="Boomerang"
go run gopath/src/secure-echo-client/cmd/secure-echo-client-client/main.go
