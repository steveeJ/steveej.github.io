#!/usr/bin/env bash
set -xe
export GOPATH=$PWD/gopath

export API_KEY="techtalkdemo"
gopath/bin/secure-echo-server --port 8081
