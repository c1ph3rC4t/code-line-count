#!/usr/bin/env bash
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.
#
# Copyright (c) 2026 c1ph3rC4t

# Config
set -euo pipefail

TOTAL_CHECKS=7
DONE_CHECKS=0

PUSH_CHECK=false
STRICT=false
CI_FLAG=false
GEN_DOCS=false
OPEN_DOCS=false
BIN=false

for arg in "$@"; do
    case $arg in
        -p | --push-check) PUSH_CHECK=true ;;
        -s | --strict) STRICT=true ;;
        -c | --ci) CI_FLAG=true ;;
        -o | --open-docs) OPEN_DOCS=true; GEN_DOCS=true; ((++TOTAL_CHECKS)) ;;
        -d | --docs) GEN_DOCS=true; ((++TOTAL_CHECKS)) ;;
        -b | --bin) BIN=true ;;
        -*)
            echo -e "\n \x1b[1m\x1b[31m::\x1b[0m\x1b[1m Unknown argument \"$arg\"\x1b[0m\n"
            exit 1
            ;;
    esac
done

# Functions
function begin_check {
    echo -e " \x1b[1m\x1b[34m::\x1b[0m\x1b[1m [$DONE_CHECKS/$TOTAL_CHECKS] $*\x1b[0m"
    CURRENT_CHECK=$*
}

function end_check {
    ((++DONE_CHECKS))
    echo -e " \x1b[1m\x1b[34m::\x1b[0m\x1b[1m $CURRENT_CHECK done\x1b[0m\n"
}

function success {
    RAND=$(( $(od -An -tu4 -N4 /dev/urandom) % 10 ))
    if (( RAND <= 0 )); then
        echo -e " \x1b[1m\x1b[35m::\x1b[0m\x1b[1m [$DONE_CHECKS/$TOTAL_CHECKS] All checks passed! :3\x1b[0m"
    else
        echo -e " \x1b[1m\x1b[32m::\x1b[0m\x1b[1m [$DONE_CHECKS/$TOTAL_CHECKS] All checks passed\x1b[0m"
    fi
}

function run_checks {
    trap 'handle_error > /dev/stderr' ERR

    # Dependency check
    begin_check Dependency check
        cargo fmt --version &> /dev/null || cargo install rustfmt
        cargo sort --version &> /dev/null || cargo install cargo-sort
        cargo audit --version &> /dev/null || cargo install cargo-audit
        cargo nextest --version &> /dev/null || cargo install --locked cargo-nextest
    end_check

    # Formatting
    begin_check Formatting
        if $CI_FLAG; then
            cargo fmt --check
            cargo sort --check
        else
            cargo fmt
            cargo sort
        fi
    end_check

    # Linting
    begin_check Linting
        if $PUSH_CHECK; then
            cargo clippy --all-targets --all-features -- \
                -A clippy::all \
                -W missing_docs \
                -W clippy::missing_errors_doc \
                -W clippy::missing_safety_doc
            cargo clippy --all-targets --all-features -- \
                -D warnings \
                -W clippy::pedantic \
                -W clippy::nursery \
                -W clippy::perf \
                -W clippy::unwrap_used \
                -W clippy::expect_used \
                -W clippy::redundant_type_annotations \
                -A missing_docs \
                -A clippy::missing_errors_doc \
                -A clippy::missing_safety_doc
        elif $STRICT; then
            cargo clippy --all-targets --all-features -- \
                -D warnings \
                -W clippy::pedantic \
                -W clippy::nursery \
                -W clippy::perf \
                -W clippy::unwrap_used \
                -W clippy::expect_used \
                -W clippy::redundant_type_annotations \
                -W missing_docs \
                -W clippy::missing_errors_doc \
                -W clippy::missing_safety_doc
        else
            cargo clippy --all-targets --all-features -- \
                -W missing_docs \
                -W clippy::pedantic \
                -W clippy::nursery \
                -W clippy::perf \
                -W clippy::unwrap_used \
                -W clippy::expect_used \
                -W clippy::redundant_type_annotations
        fi
    end_check

    # Testing
    begin_check Testing
        cargo nextest run --no-tests=pass
        if ! $BIN; then
            cargo test --doc
        fi
    end_check

    # Audit scan
    begin_check Audit scan
        cargo audit
    end_check

    # Trufflehog scan
    begin_check Trufflehog scan
        trufflehog git file://.
    end_check

    # Gitleaks scan
    begin_check Gitleaks scan
        gitleaks detect --source . -v
    end_check

    # Docs
    if $OPEN_DOCS; then
        begin_check Opening docs
            cargo doc --open
        end_check
    elif $GEN_DOCS; then
        begin_check Generating docs
            cargo doc
        end_check
    fi

    # Success
    success
}

function handle_error {
    echo -e "\n \x1b[1m\x1b[31m::\x1b[0m\x1b[1m [$DONE_CHECKS/$TOTAL_CHECKS] $CURRENT_CHECK failed\x1b[0m\n"
}

trap 'handle_error > /dev/stderr' ERR

if [ "${CI:-}" = "true" ] || $CI_FLAG; then
    CI_FLAG=true
    PUSH_CHECK=true
    run_checks > /dev/stderr
else
    run_checks
fi
