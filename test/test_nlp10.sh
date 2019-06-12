#!/usr/bin/env bash

f=data/hightemp.txt
expect=$(wc -l $f)
result=$(cargo run --bin nlp10 $f)

if [ "$expect" = "$result" ]; then
    echo "[OK] $0"
else
    echo "[ERROR] $0"
    echo "expect: $expect"
    echo "result: $result"
    exit 1
fi
