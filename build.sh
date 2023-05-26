#!/usr/bin/env bash

PROJECT=$1

python3 lib/rust/bundler.py -i games/$PROJECT/src/main.rs -o games/$PROJECT/codingame.rs
cat games/$PROJECT/codingame.rs | xclip -selection clipboard