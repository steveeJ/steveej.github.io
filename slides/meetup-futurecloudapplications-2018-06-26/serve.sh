#!/usr/bin/env bash
set -x
browser-sync start --server --port 2999 --files index.html,slides.md,js/fonts.css --watch true --no-ui 
