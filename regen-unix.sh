#!/bin/sh

set -euxo pipefail

svdtools patch patch/MCXA256.yaml
svd2rust -i svd/MCXA256.svd.patched --reexport-interrupt --ignore-groups --impl-defmt defmt --impl-debug --impl-debug-feature debug
rm -r src/*
form -i lib.rs -o src
rm lib.rs
cargo fmt
