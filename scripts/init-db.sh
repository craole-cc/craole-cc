#!/bin/sh
# init-db.sh — Initialize the local SQLite database for craole.cc
# Usage: ./scripts/init-db.sh [-f | --force | --reset]
#
# By default, skips initialization if the database already exists.
# Pass -f / --force / --reset to nuke and recreate it.
# shellcheck enable=all

set -e

# ── Config ────────────────────────────────────────────────────────────────────

DB_PATH="database/data/portfolio.db"
MIGRATIONS_DIR="database/migrations"
ENV_FILE=".env"

# ── Helpers ───────────────────────────────────────────────────────────────────

log() { printf '\033[1;34m[init-db]\033[0m %s\n' "${*:-}"; }
ok() { printf '\033[1;32m[init-db]\033[0m %s\n' "${*:-}"; }
warn() { printf '\033[1;33m[init-db]\033[0m %s\n' "${*:-}"; }
die() {
    printf '\033[1;31m[init-db]\033[0m %s\n' "${*:-}" >&2
    exit 1
}

#╔═══════════════════════════════════════════════════════════╗
#║ Aguments                                                  ║
#╚═══════════════════════════════════════════════════════════╝

FORCE=0
while [ $# -ge 1 ]; do
    case "${1:-}" in
    -f | --force | --reset) FORCE=1 ;;
    *) die "Unknown argument: $1" ;;
    esac
done

#╔═══════════════════════════════════════════════════════════╗
#║ Preflight                                                 ║
#╚═══════════════════════════════════════════════════════════╝

# Must be run from the workspace root
[ -f "Cargo.toml" ] || die "Run this script from the workspace root."

# Require sqlx-cli
command -v sqlx >/dev/null 2>&1 || die "sqlx-cli not found. Install with: cargo install sqlx-cli --no-default-features --features sqlite"

# Resolve DATABASE_URL: prefer env var, then .env file, then default
if [ -z "${DATABASE_URL:-}" ] && [ -f "${ENV_FILE:-}" ]; then
    DATABASE_URL=$(grep -E '^DATABASE_URL=' "${ENV_FILE}" | grep -v '^\s*#' | tail -1 | cut -d'=' -f2-)
fi
DATABASE_URL="${DATABASE_URL:-sqlite:./${DB_PATH}}"

log "Database URL: ${DATABASE_URL}"

#╔═══════════════════════════════════════════════════════════╗
#║ Database                                                  ║
#╚═══════════════════════════════════════════════════════════╝

if [ -f "${DB_PATH:-}" ]; then
    if [ "${FORCE:-}" -eq 1 ]; then
        warn "Removing existing database (--force)..."
        rm -f "${DB_PATH:-}"
    else
        ok "Database already exists. Skipping. Use -f / --force / --reset to reinitialize."
        exit 0
    fi
fi

log "Creating database directory..."
mkdir -p "$(dirname "${DB_PATH}")"
touch "${DB_PATH}"

#╔═══════════════════════════════════════════════════════════╗
#║ Migrations                                                ║
#╚═══════════════════════════════════════════════════════════╝

log "Running migrations from ${MIGRATIONS_DIR}..."
sqlx migrate run \
    --source "${MIGRATIONS_DIR}" \
    --database-url "${DATABASE_URL}"

ok "Done. Database initialized at ${DB_PATH}"
