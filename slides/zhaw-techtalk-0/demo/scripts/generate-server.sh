#!/usr/bin/env bash
set -x
export GOPATH=$PWD/gopath

rm -Rf src/secure-echo-server
mkdir -p src/secure-echo-server

go get github.com/docker/go-units 
go get github.com/go-openapi/loads
go get github.com/jessevdk/go-flags
go get github.com/docker/go-units

go-swagger generate server -f ../defs/2.0_secure-echo.yaml -A secure-echo-server -t src/secure-echo-server -P string
