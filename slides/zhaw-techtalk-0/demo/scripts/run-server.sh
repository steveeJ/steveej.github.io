#!/usr/bin/env bash
set -xe
export GOPATH=$PWD/gopath

go build -o gopath/bin/secure-echo-server gopath/src/secure-echo-server/cmd/secure-echo-server-server/main.go

export API_KEY="techtalkdemo"
gopath/bin/secure-echo-server --port 8081
