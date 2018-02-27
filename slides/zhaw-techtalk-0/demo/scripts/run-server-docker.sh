#!/usr/bin/env bash
set -xe
docker build gopath --tag zhaw-tech-talk-demo:latest
docker run --rm --publish 127.0.0.1:8888:8080 --env API_KEY=techtalkdemosecret zhaw-tech-talk-demo:latest
