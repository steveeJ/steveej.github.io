#!/usr/bin/env bash
set -xe
cg --debug --verbose go-server defs/3.0_secure-echo.yaml
go-swagger validate defs/2.0_secure-echo.yaml
cp out/go-server/api/swagger.yaml defs/2.0_secure-echo.yaml
