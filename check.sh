#!/usr/bin/env bash
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.
#
# Copyright (c) 2026 c1ph3rC4t

set -euo pipefail

# Error handler
trap 'echo -e "\\n \\x1b[1m\\x1b[31m::\\x1b[0m\\x1b[1m Some check failed\\x1b[0m"' ERR

# Parse arguments
OPEN_DOCS=false
for arg in "$@"; do
    case $arg in
        --open-docs) OPEN_DOCS=true ;;
        *)
            echo "Unknown option: $arg"
            echo "Usage: $0 [--open-docs]"
            exit 1
            ;;
    esac
done
echo -e " \x1b[1m\x1b[34m::\x1b[0m\x1b[1m Formatting\x1b[0m"
cargo fmt

echo -e "\n \x1b[1m\x1b[34m::\x1b[0m\x1b[1m Linting"
cargo clippy -- -D warnings

echo -e "\n \x1b[1m\x1b[34m::\x1b[0m\x1b[1m Testing\x1b[0m"
cargo test

echo -e "\n \x1b[1m\x1b[34m::\x1b[0m\x1b[1m Security scan\x1b[0m"
trufflehog git file://.
gitleaks detect --source . -v

if $OPEN_DOCS; then
    echo -e "\n \x1b[1m\x1b[34m::\x1b[0m\x1b[1m Opening docs\x1b[0m"
    cargo doc --open
else
    echo -e "\n \x1b[1m\x1b[34m::\x1b[0m\x1b[1m Generating docs\x1b[0m"
    cargo doc
fi

echo -e "\n \x1b[1m\x1b[32m::\x1b[0m\x1b[1m All checks passed\x1b[0m"
