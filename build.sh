#!/usr/bin/env bash

PROJECT=$1

python3 lib/rust/bundler.py -i $PROJECT/src/main.rs -o $PROJECT/codingame.rs