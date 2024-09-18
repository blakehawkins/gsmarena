#!/usr/bin/env bash

set -euxo pipefail

rm cap.tsv
touch cap.tsv
cargo run -- -h >> cap.tsv
cargo run >> cap.tsv
cargo run -- -q "https://www.gsmarena.com/results.php3?nYearMin=2021&nYearMax=2021&nDisplayResMax=3686400&nChargingWMin=1&sMakers=107" >> cap.tsv
