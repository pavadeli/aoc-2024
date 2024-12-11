#!/bin/bash

#
# Script to add another day to the workspace, usage:
#
#     ./prepare.sh dayXX
#

set -euo pipefail

echo Preparing ${1:?"Please provide the name of the day to add, example: ./prepare.sh day11"}
cargo new $1
cargo add common -p $1

cat >$1/src/main.rs <<EOF
use common::boilerplate;

boilerplate! {
    part1 => {}
    part2 => {}
}
EOF

code $1/src/main.rs
