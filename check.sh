#!/usr/bin/env bash
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.
#
# Copyright (c) 2026 c1ph3rC4t

# Config
set -euo pipefail

PUSH_CHECK=false
STRICT=false
CI_FLAG=false
QUIET=false
FULL_QUIET=false
OPEN_DOCS=false

for arg in "$@"; do
    case $arg in
        -p | --push-check) PUSH_CHECK=true ;;
        -s | --strict) STRICT=true ;;
        -c | --ci) CI_FLAG=true ;;
        -q | --quiet) QUIET=true ;;
        -Q | --full-quiet) QUIET=true; FULL_QUIET=true ;;
        -o | --open-docs) OPEN_DOCS=true ;;
        *)
            echo -e "\n \x1b[1m\x1b[31m::\x1b[0m\x1b[1m Unknown argument \"$arg\"\x1b[0m\n"
            exit 1
            ;;
    esac
done

if $PUSH_CHECK; then
    TOTAL_CHECKS=4
else
    TOTAL_CHECKS=5
fi

DONE_CHECKS=0

# Functions
function begin_check {
    if $FULL_QUIET; then
        return 0
    fi
    echo -e " \x1b[1m\x1b[34m::\x1b[0m\x1b[1m [$DONE_CHECKS/$TOTAL_CHECKS] $@\x1b[0m"
    CURRENT_CHECK=$@
}

function end_check {
    ((++DONE_CHECKS))
    if $FULL_QUIET; then
        return 0
    fi
    echo -e " \x1b[1m\x1b[34m::\x1b[0m\x1b[1m $CURRENT_CHECK done\x1b[0m\n"
}

function success {
    if $FULL_QUIET; then
        return 0
    fi
    echo -e " \x1b[1m\x1b[32m::\x1b[0m\x1b[1m [$DONE_CHECKS/$TOTAL_CHECKS] All checks passed\x1b[0m"
}

function run_checks {
    trap 'handle_error' ERR

    # Formatting
    begin_check Formatting
        if $PUSH_CHECK; then
            cargo fmt --check
        else
            cargo fmt
        fi
    end_check

    # Linting
    begin_check Linting
        if $PUSH_CHECK; then
            cargo clippy --all-targets --all-features -- -D warnings -A missing_docs
        elif $STRICT; then
            cargo clippy --all-targets --all-features -- -D warnings
        else
            cargo clippy --all-targets --all-features
        fi
    end_check

    # Testing
    begin_check Testing
        cargo test
    end_check

    # Security scan
    begin_check Security scan
        trufflehog git file://.
        gitleaks detect --source . -v
    end_check

    # Docs
    if ! $PUSH_CHECK; then
        if $OPEN_DOCS; then
            begin_check Opening docs
                cargo doc --open
            end_check
        else
            begin_check Generating docs
                cargo doc
            end_check
        fi
    fi

    # Success
    success
}

function handle_error {
    echo -e "\n \x1b[1m\x1b[31m::\x1b[0m\x1b[1m [$DONE_CHECKS/$TOTAL_CHECKS] $CURRENT_CHECK failed\x1b[0m\n"
}

trap 'handle_error' ERR

if [ "${CI:-}" = "true" ] || $CI_FLAG; then
    run_checks > /dev/stderr
else
    if $QUIET; then
        if $FULL_QUIET; then
            run_checks > /dev/null 2>&1
        else
            run_checks > /dev/null
        fi
    else
        run_checks
    fi
fi
