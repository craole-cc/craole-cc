#!/bin/sh
#DOC Format .rs files passed as arguments
#DOC Formatters: leptosfmt → rustfmt (if leptosfmt available), rustfmt alone otherwise
#DOC Intended for use as a treefmt formatter; treefmt handles file discovery.
# shellcheck enable=all

set -eu

#╔═══════════════════════════════════════════════════════════╗
#║ State                                                     ║
#╚═══════════════════════════════════════════════════════════╝

CHECK=""
FILES=""
FAILED=0
ERRTMP=""
FILETMP=""

#╔═══════════════════════════════════════════════════════════╗
#║ Utilities                                                 ║
#╚═══════════════════════════════════════════════════════════╝

die() {
	printf 'fmt-rust: %s\n' "$*" >&2
	exit 1
}

fail() {
	printf 'fmt-rust: ✗ %s\n' "$*" >&2
	FAILED=1
}

pass() { printf 'fmt-rust: ✔ %s\n' "$*"; }

#╔═══════════════════════════════════════════════════════════╗
#║ Argument Parsing                                          ║
#╚═══════════════════════════════════════════════════════════╝

parse_arguments() {
	for arg in "$@"; do
		case "${arg}" in
		--check) CHECK="1" ;;
		-*) die "unknown flag: ${arg}" ;;
		*) FILES="${FILES} ${arg}" ;;
		esac
	done

	[ -n "${FILES}" ] || die "no files given"
}

#╔═══════════════════════════════════════════════════════════╗
#║ Formatters                                                ║
#╚═══════════════════════════════════════════════════════════╝

fmt_with_leptosfmt() {
	_file="$1"

	if [ -n "${CHECK}" ]; then
		if leptosfmt --stdin --rustfmt --experimental-tailwind --quiet --check \
			<"${_file}" >"${ERRTMP}" 2>&1; then
			pass "${_file}"
		else
			cat "${ERRTMP}" >&2
			fail "${_file}"
		fi
	else
		if leptosfmt --stdin --rustfmt --experimental-tailwind --quiet \
			<"${_file}" >"${FILETMP}" 2>"${ERRTMP}"; then
			mv "${FILETMP}" "${_file}"
			pass "${_file}"
		else
			cat "${ERRTMP}" >&2
			fail "${_file}"
		fi
	fi
}

fmt_with_rustfmt() {
	_file="$1"

	if [ -n "${CHECK}" ]; then
		if rustfmt --check "${_file}" 2>"${ERRTMP}"; then
			pass "${_file}"
		else
			cat "${ERRTMP}" >&2
			fail "${_file}"
		fi
	else
		if rustfmt "${_file}" 2>"${ERRTMP}"; then
			pass "${_file}"
		else
			cat "${ERRTMP}" >&2
			fail "${_file}"
		fi
	fi
}

#╔═══════════════════════════════════════════════════════════╗
#║ Main                                                      ║
#╚═══════════════════════════════════════════════════════════╝

main() {
	parse_arguments "$@"

	ERRTMP=$(mktemp)
	FILETMP=$(mktemp)
	trap 'rm -f "${ERRTMP}" "${FILETMP}"' EXIT INT TERM

	if command -v leptosfmt >/dev/null 2>&1; then
		for file in ${FILES}; do
			fmt_with_leptosfmt "${file}"
		done
	elif command -v rustfmt >/dev/null 2>&1; then
		for file in ${FILES}; do
			fmt_with_rustfmt "${file}"
		done
	else
		die "neither leptosfmt nor rustfmt found"
	fi

	exit "${FAILED}"
}

main "$@"
