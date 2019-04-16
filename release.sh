#!/bin/sh
set -e
cargo build
cargo test

# Backup manually-placed files.
mkdir -p docs_tmp
cp docs/404.html docs_tmp
cp docs/_config.yml docs_tmp

# Generate docs.
rm -rf docs
cargo doc --no-deps
cp -r target/doc docs

# Restore manually-placed files.
cp docs_tmp/* docs
rm -rf docs_tmp

# Site is published at BASE/accumulator.
# Docs are at BASE/accumulator/docs.
mv docs/accumulator docs/docs

# Add Jekyll redirect for BASE/accumulator.
echo '0a
---
redirect_from: "/index.html"
---
.
w' | ed docs/docs/index.html