#!/usr/bin/env bash
set -x
export GOPATH=$PWD

go get github.com/go-openapi/runtime
go get github.com/go-openapi/validate
go get golang.org/x/net/context
go get golang.org/x/net/context/ctxhttp
go get github.com/go-openapi/strfmt

rm -Rf src/secure-echo-client
mkdir -p src/secure-echo-client
go-swagger generate client -f ../defs/2.0_secure-echo.yaml -A secure-echo-client -t src/secure-echo-client -P string
