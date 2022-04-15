#!/usr/bin/env bash

find ./src -name "*.rs" -exec cat {} + > _main.rs
sed -i '/mod */d' _main.rs
sed -i '/use crate::*/d' _main.rs