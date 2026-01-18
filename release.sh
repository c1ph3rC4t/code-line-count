#!/usr/bin/env bash
#\___________________,
# Cross platform bash

set -euo pipefail
#\____________________________,
# -e          => exit on error
# -u          => undefined var errors
# -o pipefail => fail if any pipeline command fails

cargo fmt
#\___________,
# Format code

cargo clippy -- -D warnings
#\____,
# Lint

cargo test
#\________________,
# Test using cargo

trufflehog git file://.
gitleaks detect --source . -v
#\_________________________,
# Scan for and detect leaks

cargo build --release
#\_____,
# Build

VERSION=$(cargo pkgid | cut -d'@' -f2)
mkdir -p ./builds
cp ./target/release/clc "./builds/clc-${VERSION}"
#\______________,
# Copy to builds

"./builds/clc-${VERSION}" --help
"./builds/clc-${VERSION}" --version
#\____________,
# Sanity check
