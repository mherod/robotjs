#!/bin/bash

# Backup the original Cargo.toml
cp Cargo.toml Cargo.toml.bak

# Fix the lto line and remove any trailing whitespace
sed -i '' 's/lto = true-e/lto = true/' Cargo.toml
sed -i '' 's/[[:space:]]*$//' Cargo.toml

# Remove any duplicate entries
awk '!seen[$0]++' Cargo.toml > Cargo.toml.tmp && mv Cargo.toml.tmp Cargo.toml

echo "Cargo.toml has been fixed. Please review the changes."
diff Cargo.toml.bak Cargo.toml