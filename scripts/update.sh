#!/bin/sh
# shellcheck enable=all

flake=1
cargo=0
mise=0

for arg in "$@"; do
	case "$arg" in
	--rust | --cargo) cargo=1 ;;
	--mise) mise=1 ;;
	--no-flake) flake=0 ;;
	esac
done

if [ "$flake" = 1 ]; then
	nix flake update
fi

if [ "$cargo" = 1 ]; then
	cargo update
fi

if [ "$mise" = 1 ]; then
	mise self-update
fi

git add --all
if [ -n "$(git status --porcelain)" ]; then
	git commit --message "update"
	git push
fi
