#!/usr/bin/env bash

execute_test() {
    for day in "$@"; do
        echo "Testing $day"
        echo "# Test results for $day" >> $GITHUB_STEP_SUMMARY
        cargo test --bin $day --no-fail-fast >> $GITHUB_STEP_SUMMARY
    done
}

while IFS= read -r day; do
    execute_test "$day";
done < <(
    (for filename in src/bin/*.rs; do
        filename=${filename#*/*/} # Remove the directory prefix
        filename=${filename%.rs}  # Remove the extension suffix
        echo ${filename:3} $filename
    done) | sort -n | cut -d' ' -f 2
)
