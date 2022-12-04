#!/usr/bin/env bash
set -euo pipefail

day="$1"

tinput="$(curl --silent https://adventofcode.com/2022/day/$((10#$day)) | htmlq --text 'article.day-desc pre:first-of-type code')"
env DAY="$day" tinput="$tinput" envsubst < templates/day.rs > "src/bin/$day.rs"
