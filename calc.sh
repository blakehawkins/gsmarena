#!/usr/bin/env bash

set -euxo pipefail

rm cap.tsv
touch cap.tsv
cargo run -- -h >> cap.tsv
cargo run -- -o "Android One" >> cap.tsv
cargo run >> cap.tsv
cargo run -- -b 4300 >> cap.tsv
