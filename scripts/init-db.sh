#!/bin/sh
# init-db.sh — Initialize the local SQLite database for craole.cc
# Usage: ./scripts/init-db.sh [-f | --force | --reset]
#
# By default, skips initialization if the database already exists.
# Pass -f / --force / --reset to nuke and recreate it.
# shellcheck enable=all

set -e

#╔═══════════════════════════════════════════════════════════╗
#║ Configuration                                             ║
#╚═══════════════════════════════════════════════════════════╝
DB_PATH="database/data/portfolio.db"
MIGRATIONS_DIR="database/migrations"
ENV_FILE=".env"able=all
SCR_NAME="init-db"

#╔═══════════════════════════════════════════════════════════╗
#║ Color Output                                              ║
#╚═══════════════════════════════════════════════════════════╝
if [ -t 1 ]; then
  CLR_SUCCESS=$(tput setaf 2)
  CLR_FAILURE=$(tput setaf 1)
  CLR_INFO=$(tput setaf 6)
  CLR_WARNING=$(tput setaf 3)
  CLR_BOLD=$(tput bold)
  CLR_RESET=$(tput sgr0)
else
  CLR_SUCCESS="$(printf "\033[1;42m")"
  CLR_FAILURE="$(printf "\033[1;31m")"
  CLR_INFO="$(printf "\033[1;34m")"
  CLR_WARNING="$(printf "\033[1;33m")"
  CLR_BOLD=""
  CLR_RESET="$(printf "\033[0m")"
fi

#╔═══════════════════════════════════════════════════════════╗
#║ Helpers                                                   ║
#╚═══════════════════════════════════════════════════════════╝
log() { printf '%s[INFO]%s:%s %s\n' "${CLR_INFO}" "${SCR_NAME}" "${CLR_RESET}" "${*:-}"; }
ok() { printf '%s[INFO]%s:%s %s\n' "${CLR_SUCCESS}" "${SCR_NAME}" "${CLR_RESET}" "${*:-}"; }
warn() { printf '%s[WARN]%s:%s %s\n' "${CLR_WARNING}" "${SCR_NAME}" "${CLR_RESET}" "${*:-}"; }
die() {
  printf '%s[ERROR]%s:%s %s\n' "${CLR_FAILURE}" "${SCR_NAME}" "${CLR_RESET}" "${*:-}"
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
  shift
done

#╔═══════════════════════════════════════════════════════════╗
#║ Preflight                                                 ║
#╚═══════════════════════════════════════════════════════════╝

#> Ensure be run from the workspace root
[ -f "Cargo.toml" ] || die "Run this script from the workspace root."

#> Ensure sqlx-cli is available
command -v sqlx >/dev/null 2>&1 ||
  die "sqlx-cli not found. Install with: cargo install sqlx-cli --no-default-features --features sqlite"

#> Resolve DATABASE_URL: prefer env var, then .env file, then default
if [ -z "${DATABASE_URL:-}" ] && [ -f "${ENV_FILE:-}" ]; then
  DATABASE_URL=$(
    grep -E '^DATABASE_URL=' "${ENV_FILE}" |
      grep -v '^\s*#' | tail -1 | cut -d'=' -f2-
  )
fi
DATABASE_URL="${DATABASE_URL:-sqlite:./${DB_PATH}}"

log "Database URL: ${DATABASE_URL}"

#╔═══════════════════════════════════════════════════════════╗
#║ Database                                                  ║
#╚═══════════════════════════════════════════════════════════╝
#> Initialize the database
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
