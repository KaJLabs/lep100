#!/usr/bin/env bash
set -euo pipefail
# Requires lithc in PATH
lithc test ./tests/lep100-tests --network local --report junit
