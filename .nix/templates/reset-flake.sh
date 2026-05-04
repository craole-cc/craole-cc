#!/bin/sh
# shellcheck enable=all

set -u

usage() {
	cat <<'EOF'
Usage: reset-flake [OPTIONS]

Removes deployed template files and transient build directories from the
current project root.

Options:
  --help, -h  Show this help text
EOF
}

parse_args() {
	case "${1:-}" in
	'')
		return 0
		;;
	-h | --help)
		usage
		return 1
		;;
	*)
		printf 'Unknown option: %s\n' "${1}" >&2
		usage >&2
		return 2
		;;
	esac
}

resolve_root() {
	ROOT=${PRJ_ROOT:-${PWD}}

	[ -n "${ROOT}" ] || {
		printf 'Error: ROOT is empty.\n' >&2
		exit 1
	}

	[ -f "${ROOT}/flake.nix" ] || {
		printf 'Error: %s is not a flake root; missing flake.nix.\n' "${ROOT}" >&2
		exit 1
	}

	[ -f "${ROOT}/.nix-template-state" ] || {
		printf 'Error: refusing to reset %s.\n' "${ROOT}" >&2
		printf 'Missing .nix-template-state marker. This does not look like a deployed template target.\n' >&2
		exit 1
	}
}

remove_path() {
	path=$(printf '%s/%s' "${ROOT}" "${1}")

	if [ -e "${path}" ]; then
		rm -rf "${path}"
		printf 'removed %s\n' "${path}"
	fi
}

dir_is_empty() {
	path=${1}
	first_entry=$(find "${path}" -mindepth 1 -maxdepth 1 -print -quit 2>/dev/null || true)

	[ -z "${first_entry}" ]
}

remove_empty_dir() {
	path=$(printf '%s/%s' "${ROOT}" "${1}")

	if [ -d "${path}" ] && dir_is_empty "${path}"; then
		rmdir "${path}" 2>/dev/null || true
	fi
}

execute() {
	remove_path ".cargo/config.toml"
	remove_empty_dir ".cargo"

	remove_path ".envrc"
	remove_path ".gitignore"

	remove_path ".markdownlint-cli2.yaml"
	remove_path "markdownlint-cli2.yaml"

	remove_path ".mise.toml"
	remove_path "mise.toml"

	remove_path ".shellcheckrc"
	remove_path "shellcheckrc"

	remove_path ".treefmt.toml"
	remove_path "treefmt.toml"

	remove_path "rust-analyzer.toml"
	remove_path "rust-toolchain.toml"
	remove_path "rustfmt.toml"

	remove_path ".direnv"
	remove_path "target"

	remove_path ".nix-template-state"
}

main() {
	parse_args "${@}"
	status=${?}

	case "${status}" in
	0)
		resolve_root
		execute
		;;
	1) exit 0 ;;
	*) exit "${status}" ;;
	esac
}

main "${@}"
