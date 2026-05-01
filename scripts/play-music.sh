#!/bin/sh
# shellcheck enable=all

# play-music - Play music files from a library using fd + mpv
#
# USAGE:
#   play-music [OPTIONS] [SEARCH TERMS...]
#
# OPTIONS:
#   -x, --shuffle        Shuffle playback order (default: sequential)
#   -s, --search TERM    Explicit search term; may be repeated for
#                        multiple searches (same effect as positional args
#                        but takes priority in playlist order)
#
# ARGUMENTS:
#   SEARCH TERMS         One or more terms to match against full file paths.
#                        Unmatched terms are silently ignored.
#                        Omit entirely to play the whole library.
#
# EXAMPLES:
#   play-music                         # Entire library, sequential
#   play-music -x                      # Entire library, shuffled
#   play-music Sade                    # All Sade matches, sequential
#   play-music -x Sade                 # All Sade matches, shuffled
#   play-music -s Beyonce -x Sade      # Beyonce first, then Sade, shuffled
#   play-music Sade Eminem "Jay-Z"     # Multiple artists, sequential
#
# ENVIRONMENT:
#   MUSIC       Music library root directory
#               (default: /mnt/Storage/Music)
#   EXTENSIONS  Space-separated list of audio file extensions to include
#               (default: mp3 flac m4a aac ogg opus wav alac wma ape mka)
#
# DEPENDENCIES:
#   fd    - Fast file finder (github.com/sharkdp/fd)
#   mpv   - Media player (mpv.io)
#
# NOTES:
#   - Explicit --search terms are added to playlist before positional terms
#   - SHUFFLE is a boolean; ${SHUFFLE:+--shuffle} avoids passing empty args
#   - build_playlist streams paths via stdout into mpv --playlist=-
#   - fd non-matches return exit 1, suppressed with || true
#   - Multi-word searches with spaces work via quoted positional args
#     but are split on spaces internally; avoid spaces in search terms

MUSIC="${MUSIC:-/mnt/Storage/Music}"
# EXTENSIONS="${EXTENSIONS:-mp3 flac m4a aac ogg opus wav alac wma ape mka}"
# SHUFFLE="${SHUFFLE:-}"
# DRY_RUN="${DRY_RUN:-}"
# SEARCHES="${SEARCHES:-}"
# DELIMITER=printf '\037'

main() {
	: "${VERBOSITY:="info"}"
	: "${DELIMITER:="$(printf "|")"}"
	# : "${DELIMITER:="$(printf "\037")"}"
	: "${MUSIC:="/mnt/Storage/Music"}"
	: "${EXTENSIONS:=mp3 flac m4a aac ogg opus wav alac wma ape mka}"
	: "${SHUFFLE:=}"
	: "${DRY_RUN:=1}"
	SEARCH_COUNT=0

	parse_arguments "$@"
	# execute
}

cleanup() {
	unset args SEARCH_COUNT
}

parse_arguments() {
	while [ "$#" -gt 0 ]; do
		case "$1" in
		-d | --dry-run) DRY_RUN=true ;;
		-x | --shuffle) SHUFFLE=true ;;
		-s | --search)
			shift
			if [ "$#" -gt 0 ]; then
				build_search "$1"
			else
				printf 'Error: --search requires a value\n' >&2
				exit 1
			fi
			;;
		-*)
			printf 'Error: Unknown option %s\n' "$1" >&2
			exit 1
			;;
		*) args="${args:+${args}${DELIMITER}}$1" ;;
		esac
		shift
	done

	#> Parse positional arguments, if necessary
	ifs="${IFS}"
	IFS="${DELIMITER}"
	# shellcheck disable=SC2086
	set -- ${args}
	IFS="${ifs}"

	while [ $# -ge 1 ]; do
		find_music "$1"
		echo $1
		shift
	done

	#> Parse positional arguments, if necessary
	ifs="${IFS}"
	IFS="${DELIMITER}"
	# shellcheck disable=SC2086
	set -- ${PLAYLIST}
	IFS="${ifs}"
	n=0
	while [ $# -ge 1 ]; do
		echo $1
		# n=$((n + 1))
		# printf "%s. %s\n" "${n}" "${1}"
		shift
	done
	# echo "${PLAYLIST}"
}

find_music() {
	if is_url "$1"; then
		PLAYLIST="${PLAYLIST:+${PLAYLIST}${DELIMITER}}$1"
		# if [ -n "${PLAYLIST}" ]; then
		# 	PLAYLIST="${PLAYLIST}$(printf '\n%s' "$1")"
		# else
		# 	PLAYLIST="$1"
		# fi
	else
		matches="$(fd --fixed-strings --full-path "$1" "${MUSIC}" \
			--extension mp3 \
			--extension flac \
			--extension m4a \
			--extension aac \
			--extension ogg \
			--extension opus \
			--extension wav \
			--extension alac \
			--extension wma \
			--extension ape \
			--extension mka || true)"

		if [ -n "${matches}" ]; then
		echo "pop --- $matches"
			PLAYLIST="${PLAYLIST:+${PLAYLIST}${DELIMITER}}${matches}"
			# if [ -n "${PLAYLIST}" ]; then
			# 	PLAYLIST="${PLAYLIST}$(printf '\n%s' "${matches}")"
			# else
			# 	PLAYLIST="${matches}"
			# fi
		fi
	fi
}

generate_playlist() {
	printf '%s\n' "${PLAYLIST}" | sort
}

validate() {
	if [ -d "${MUSIC}" ]; then :; else
		printf 'Error: Music directory %s does not exist\n' "${MUSIC}" >&2
		exit 1
	fi
}

is_url() {
	case "$1" in
	http://* | https://*) return 0 ;;
	*) return 1 ;;
	esac
}

build_extensions() {
	validate
	ifs=${IFS}
	IFS="${DELIMITER}"

	for ext in ${EXTENSIONS}; do
		printf ' --extension %s' "${ext}"
	done

	IFS=${ifs}
}

build_search() {
	SEARCH_COUNT=$((SEARCH_COUNT + 1))
	eval "SEARCH_${SEARCH_COUNT}=\$1"
}

build_playlist() {
	if [ "${SEARCH_COUNT}" -gt 0 ]; then
		i=1
		while [ "${i}" -le "${SEARCH_COUNT}" ]; do
			eval "item=\${SEARCH_${i}}"
			# shellcheck disable=SC2154
			if is_url "${item}"; then
				printf '%s\n' "${item}"
			else
				build_extensions
				fd --fixed-strings --full-path "${item}" "${MUSIC}" "$@" || true
			fi
			i=$((i + 1))
		done
	else
		build_extensions
		fd . "${MUSIC}" "$@" || true
	fi
}

show_playlist() {

	printf '\n📋 Playlist:\n'
	generate_playlist | nl -w2 -s' '
	# # printf '\n📋 Playlist (%d items):\n' "$(build_playlist | wc -l)"
	# build_playlist | nl -w2 -s' ' | sed 's|^|  |'
	# printf '\n'
}

play() {
	# build_playlist |
	# 	mpv --script-opts=ytdl_hook-program=yt-dlp ${SHUFFLE:+--shuffle} --playlist=-
	generate_playlist | mpv ${SHUFFLE:+--shuffle} --playlist=-
}

execute() {
	case "${DRY_RUN}" in
	1 | true | [yY]* | on) show_playlist ;;
	0 | false | [nN]* | off) play ;;
	*) play ;;
	esac
}

main "$@"
