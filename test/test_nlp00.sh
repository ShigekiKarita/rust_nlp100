#!/usr/bin/env bash

if [ $(cargo run --bin nlp00) = desserts ]; then
    echo "[OK] $0"
else
    echo "[ERROR] $0"
    exit 1;
fi
