#!/usr/bin/env bash
# shellcheck disable=SC1090
[[ -n "${TOOLS_ALIASES}" && -f "${TOOLS_ALIASES}" ]] &&
	source "${TOOLS_ALIASES}"
