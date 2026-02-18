#!/bin/sh
# scripts/fmt-rust.sh
# Format all .rs files (excluding .git/, target/, archives/, hidden)
# Discovery: fd → git ls-files → find
# Formatters: leptosfmt (if available) → rustfmt
# shellcheck enable=all

set -eu

#> Collect discovered files into a temp file, cleaned up on exit
TMP=$(mktemp)
trap 'rm -f "$TMP"' EXIT INT TERM

if false; then
	:
elif command -v git >/dev/null 2>&1 && [ -d .git ]; then
	#> Use git ls-files to discover target files
	#? git ls-files gives exact .gitignore semantics via pathspec exclusions
	git ls-files --cached --others --exclude-standard \
		-- ':!*/.*' ':!archives/*' '*.rs' >"${TMP}"
elif command -v fd >/dev/null 2>&1; then
	#> Use fd to discover target files
	#? fd respects .gitignore and excludes hidden dirs by default
	fd --type file --extension rs --exclude archives >"${TMP}"
else
	#> Fallback to find, pruning .git/, target/, archives/, and hidden dirs
	#? Manual exclusions mirror .gitignore behaviour as closely as possible
	\find . \
		-type d \( -path '*/.git*' -o -path '*/target*' -o -path '*/archives*' \) -prune -o \
		-type f -name '*.rs' ! -path '*/.*/*' \
		-print >"${TMP}"
fi

#> Abort early if no files were found
[ -s "${TMP}" ] || {
	printf 'fmt-rust: no .rs files found\n' >&2
	exit 0
}

#> Format with leptosfmt first
#? leptosfmt handles Leptos view! macros that rustfmt would mangle
if command -v leptosfmt >/dev/null 2>&1; then
	xargs -n 1 leptosfmt --quiet --experimental-tailwind <"${TMP}"
fi

#> Format with rustfmt (runs after leptosfmt, or alone if leptosfmt is absent)
#? rustfmt handles Leptos view! macros that rustfmt would mangle
if command -v rustfmt >/dev/null 2>&1; then
	xargs -n 1 rustfmt <"${TMP}"
else
	printf 'fmt-rust: rustfmt not found, skipping formatting\n' >&2
fi
