#!/usr/bin/env bash
set -euo pipefail

# Production deployment helper. It is intentionally root-owned and invoked by
# the dedicated GitHub Actions runner through a narrow sudoers rule.

APP_ROOT=/opt/craole-cc
RELEASES="$APP_ROOT/releases"
CURRENT="$APP_ROOT/current"
WORKSPACE=/opt/actions-runner/_work/craole-cc/craole-cc
SERVICE=craole-cc.service

fail() {
  printf 'deployment error: %s\n' "$*" >&2
  exit 1
}

[ "$(id -u)" -eq 0 ] || fail 'must run as root'
[ -d "$WORKSPACE" ] || fail "workspace missing: $WORKSPACE"
[ -x "$WORKSPACE/target/release/backend" ] || fail 'release backend missing'
[ -f "$WORKSPACE/target/site/pkg/craole-cc.js" ] || fail 'release JavaScript missing'
[ -f "$WORKSPACE/target/site/pkg/craole-cc.wasm" ] || fail 'release WASM missing'
[ -f "$WORKSPACE/.deploy-static/index.html" ] || fail 'static export missing'

sha="$(git -C "$WORKSPACE" rev-parse --short=12 HEAD)"
timestamp="$(date -u +%Y%m%dT%H%M%SZ)"
release="$RELEASES/$timestamp-$sha"
old_release="$(readlink -f "$CURRENT" 2>/dev/null || true)"

mkdir -p "$release/site" "$release/static"
cp -a "$WORKSPACE/target/site/." "$release/site/"
cp -a "$WORKSPACE/.deploy-static/." "$release/static/"
install -m 0755 "$WORKSPACE/target/release/backend" "$release/backend"
chown -R caddy:caddy "$release"

ln -sfn "$release" "$APP_ROOT/current.next"
mv -Tf "$APP_ROOT/current.next" "$CURRENT"
systemctl restart "$SERVICE"

healthy=0
for _ in {1..15}; do
  if systemctl is-active --quiet "$SERVICE" \
    && curl --fail --silent --show-error http://127.0.0.1:3000/ >/dev/null \
    && curl --fail --silent --show-error http://127.0.0.1:3000/dev >/dev/null; then
    healthy=1
    break
  fi
  sleep 2
done

if [ "$healthy" -ne 1 ]; then
  printf 'new release failed health check; rolling back\n' >&2
  if [ -n "$old_release" ] && [ -d "$old_release" ]; then
    ln -sfn "$old_release" "$APP_ROOT/current.next"
    mv -Tf "$APP_ROOT/current.next" "$CURRENT"
    systemctl restart "$SERVICE" || true
  fi
  exit 1
fi

# Keep the active release and the previous rollback target. Pruning is best
# effort and never makes a healthy deployment fail.
find "$RELEASES" -mindepth 1 -maxdepth 1 -type d -printf '%T@ %p\n' \
  | sort -nr \
  | awk 'NR > 5 {sub(/^[^ ]+ /, ""); print}' \
  | while read -r candidate; do
      [ "$candidate" = "$release" ] && continue
      [ "$candidate" = "$old_release" ] && continue
      rm -rf -- "$candidate"
    done

printf 'deployed %s\n' "$release"
