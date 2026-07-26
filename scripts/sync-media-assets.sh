#!/usr/bin/env bash
set -euo pipefail

ROOT=$(git rev-parse --show-toplevel 2>/dev/null || pwd)
cd "$ROOT"

# `assets/media/images` is the canonical image source. `public/media/images`
# is a generated delivery copy consumed by cargo-leptos and the runtime.
rm -rf public/media/images
mkdir -p public/media/images
cp -a assets/media/images/. public/media/images/

printf 'synced %s image assets to public/media/images\n' "$(find assets/media/images -maxdepth 1 -type f | wc -l)"
