#!/bin/sh
# shellcheck enable=all

usage() {
	printf 'Usage: update [OPTIONS]\n'
	printf '\n'
	printf 'Options:\n'
	printf '  --rust, --cargo  Also run cargo update\n'
	printf '  --mise           Also run mise self-update\n'
	printf '  --no-flake       Skip nix flake update\n'
	printf '  --help           Show this help\n'
}

#? Returns 0 (true) if the argument is 1, yes, true, or on (case-insensitive).
is_true() {
	case "${1}" in
	1 | [Yy]es | [Tt]rue | [Oo]n) return 0 ;;
	*) return 1 ;;
	esac
}

flake=1
cargo=0
mise=0

for arg in "$@"; do
	case "${arg}" in
	--rust | --cargo) cargo=1 ;;
	--mise) mise=1 ;;
	--no-flake) flake=0 ;;
	--help)
		usage
		exit 0
		;;
	*) ;;
	esac
done

if is_true "${flake}"; then
	nix flake update 2>/dev/null
fi

if is_true "${cargo}"; then
	cargo update
	if command -v cargo >/dev/null 2>&1; then
		cargo reload
	fi
fi

if is_true "${mise}"; then
	if command -v mise >/dev/null 2>&1; then
		mise self-update
	fi
fi

git add --all
case "$(git status --porcelain)" in
?*)
	git commit --message "update"
	git push
	;;
*) ;;
esac

if command -v direnv >/dev/null 2>&1; then
	direnv reload
fi
