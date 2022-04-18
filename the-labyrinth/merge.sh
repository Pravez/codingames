#!/usr/bin/env bash

find ./src -name "*.rs" -exec cat {} + > _main.rs
gsed -i '/mod */d' _main.rs
gsed -i '/use crate::*/d' _main.rs