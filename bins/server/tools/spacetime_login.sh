#!/usr/bin/env bash

set -euo pipefail

cd "$(dirname "$0")/.."

if [ ! -f .env ]; then
  echo ".env not found in bins/server" >&2
  exit 1
fi

# shellcheck disable=SC1091
. ./.env

if [ -z "${spacetimedb_token:-}" ]; then
  echo "spacetimedb_token not set in bins/server/.env" >&2
  exit 1
fi

spacetime login --token "$spacetimedb_token"
