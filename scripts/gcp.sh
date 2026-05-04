#!/bin/sh
# shellcheck enable=all

usage() {
	printf 'Usage: gcp [OPTIONS] [MESSAGE]\n'
	printf '\n'
	printf 'Stage all changes, commit, and push.\n'
	printf 'If no message is given, the last commit subject is reused.\n'
	printf '\n'
	printf 'Arguments:\n'
	printf '  MESSAGE          Commit message (optional)\n'
	printf '\n'
	printf 'Options:\n'
	printf '  --no-push        Commit without pushing\n'
	printf '  --message MSG    Explicit commit message\n'
	printf '  --help           Show this help\n'
}

parse_args() {
	push=1
	msg=''

	while [ "$#" -gt 0 ]; do
		case "${1}" in
		--help)
			usage
			exit 0
			;;
		--no-push)
			push=0
			;;
		--message)
			msg="${2}"
			shift
			;;
		--)
			shift
			break
			;;
		--*)
			printf 'Unknown option: %s\n' "${1}" >&2
			usage
			exit 1
			;;
		*)
			break
			;;
		esac
		shift
	done

	# Remaining args after flags become the message
	case "${*}" in
	?*) msg="${*}" ;;
	*) ;;
	esac
}

execute() {
	git add --all

	case "$(git status --porcelain)" in
	?*)
		case "${msg}" in
		'') msg="$(git log -1 --pretty=%B 2>/dev/null | head -1)" ;;
		*) ;;
		esac

		git commit --message "${msg}"

		case "${push}" in
		1) git push ;;
		*) ;;
		esac
		;;
	*) ;;
	esac
}

main() {
	parse_args "$@"
	execute
}

main "$@"
