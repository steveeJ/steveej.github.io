#!/usr/bin/env bash 
set -x 
pushd gopath
gcloud app deploy --stop-previous-version --promote
