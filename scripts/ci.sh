#!/usr/bin/env bash
set -euo pipefail

# Local/CI quality gate for craole.cc.
# Run from a Nix shell with:
#   nix develop --command bash scripts/ci.sh

ROOT=$(git rev-parse --show-toplevel 2>/dev/null || pwd)
cd "$ROOT"

export DATABASE_URL="${DATABASE_URL:-sqlite://database/data/portfolio.db}"
export SQLX_OFFLINE="${SQLX_OFFLINE:-false}"

# cargo may be configured with a shared target directory (for example
# ~/.cargo/global_target). Resolve the actual directory instead of assuming
# target/ at the repository root. Respect an explicit override for CI or
# shared build caches.
CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$(cargo metadata --format-version 1 --no-deps \
  | python3 -c 'import json,sys; print(json.load(sys.stdin)["target_directory"])')}"
export CARGO_TARGET_DIR

need() {
	if ! command -v "$1" >/dev/null 2>&1; then
		printf 'error: required command not found: %s\n' "$1" >&2
		exit 127
	fi
}

need cargo
need rustc
need cargo-leptos
need wasm-bindgen
need wasm-opt
need sass
need tailwindcss
need sqlx
need sqlite3

printf '==> Toolchain\n'
rustc --version
cargo --version
cargo leptos --version
wasm-bindgen --version
wasm-opt --version | head -n 1
sass --version
tailwindcss --help | head -n 1
sqlx --version
sqlite3 --version | head -n 1

printf '\n==> Preparing SQLite database at %s\n' "$DATABASE_URL"
mkdir -p database/data
sqlx database create --database-url "$DATABASE_URL"
sqlx migrate run --source database/migrations --database-url "$DATABASE_URL"

printf '\n==> Content validation\n'
cargo run -p content -- validate .

printf '\n==> Content database seed smoke test\n'
cargo run -p content -- export-sql . | sqlite3 database/data/portfolio.db
PROJECT_COUNT=$(sqlite3 database/data/portfolio.db 'SELECT COUNT(*) FROM projects;')
POST_COUNT=$(sqlite3 database/data/portfolio.db 'SELECT COUNT(*) FROM posts;')
printf 'Seeded projects=%s posts=%s\n' "$PROJECT_COUNT" "$POST_COUNT"
if [ "$PROJECT_COUNT" -lt 1 ] || [ "$POST_COUNT" -lt 1 ]; then
	printf 'error: content seed produced an empty required table\n' >&2
	exit 1
fi

printf '\n==> Content static export smoke test\n'
./scripts/sync-media-assets.sh
if [ "${GENERATE_TTS:-0}" = "1" ]; then
	printf 'Generating local neural TTS audio with %s\n' "${TTS_PROVIDER:-piper}"
	python3 scripts/generate_tts.py --provider "${TTS_PROVIDER:-piper}"
else
	printf 'Skipping paid TTS generation. Set GENERATE_TTS=1 to enable it.\n'
fi
STATIC_DIST=$(mktemp -d)
cargo run -p content -- export-json . "$STATIC_DIST/data"
cargo run -p content -- export-static . "$STATIC_DIST/site"
test -f "$STATIC_DIST/data/manifest.json"
test -f "$STATIC_DIST/site/index.html"
test -f "$STATIC_DIST/site/data/manifest.json"
STATIC_PROJECT_COUNT=$(sqlite3 database/data/portfolio.db 'SELECT COUNT(*) FROM projects;')
printf 'Static export generated JSON manifest and site index; projects=%s\n' "$STATIC_PROJECT_COUNT"
rm -rf "$STATIC_DIST"

printf '\n==> Leptos full-stack build smoke test\n'
cargo leptos build
test -f target/site/pkg/craole-cc.js
test -f target/site/pkg/craole-cc.wasm
test -x "$CARGO_TARGET_DIR/debug/backend"
printf 'Leptos build generated server binary and WASM package.\n'

printf '\n==> Checking SQLx offline metadata\n'
cargo sqlx prepare --workspace --check --database-url "$DATABASE_URL"

if [ "${STRICT_FORMAT:-0}" = "1" ]; then
	printf '\n==> Rust formatting\n'
	cargo fmt --all --check

	if command -v treefmt >/dev/null 2>&1; then
		printf '\n==> Repository formatting\n'
		treefmt --fail-on-change
	fi
else
	printf '\n==> Formatting\n'
	printf 'Skipping formatting gate by default because the current tree is not rustfmt-clean. Set STRICT_FORMAT=1 to enforce it.\n'
fi

printf '\n==> Cargo check\n'
cargo check --workspace

printf '\n==> Clippy\n'
cargo clippy --workspace --all-targets --all-features -- -D warnings

printf '\n==> Tests\n'
if cargo nextest --version >/dev/null 2>&1; then
	cargo nextest run --workspace --no-tests pass
else
	cargo test --workspace
fi

printf '\nCI checks completed successfully.\n'
