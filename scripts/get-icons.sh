#!/bin/sh

# Create directories
mkdir -p public/icons/tech
mkdir -p public/icons/social
mkdir -p public/icons/ui

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Download function
download_icon() {
	url="$1"
	output="$2"
	printf "Downloading %s...\n" "$output"
	if curl -fsSL "$url" -o "$output"; then
		printf "${GREEN}✓ %s downloaded${NC}\n" "$output"
	else
		printf "${RED}✗ Failed to download %s${NC}\n" "$output"
	fi
}

printf "${CYAN}=== Downloading Technology Icons ===${NC}\n"

# Languages & Core
download_icon "https://cdn.simpleicons.org/rust/000000" "public/icons/tech/rust.svg"
download_icon "https://cdn.simpleicons.org/gnubash/4EAA25" "public/icons/tech/bash.svg"
download_icon "https://cdn.simpleicons.org/python/3776AB" "public/icons/tech/python.svg"
download_icon "https://cdn.simpleicons.org/zig/F7A41D" "public/icons/tech/zig.svg"
download_icon "https://cdn.simpleicons.org/go/00ADD8" "public/icons/tech/go.svg"
download_icon "https://cdn.simpleicons.org/javascript/F7DF1E" "public/icons/tech/javascript.svg"
download_icon "https://cdn.simpleicons.org/typescript/3178C6" "public/icons/tech/typescript.svg"

# Web Frameworks
download_icon "https://cdn.simpleicons.org/tokio/000000" "public/icons/tech/tokio.svg"
download_icon "https://cdn.simpleicons.org/htmx/3366CC" "public/icons/tech/htmx.svg"
download_icon "https://cdn.simpleicons.org/actix/000000" "public/icons/tech/actix.svg"
download_icon "https://cdn.simpleicons.org/tailwindcss/06B6D4" "public/icons/tech/tailwind.svg"
download_icon "https://cdn.simpleicons.org/react/61DAFB" "public/icons/tech/react.svg"
download_icon "https://cdn.simpleicons.org/svelte/FF3E00" "public/icons/tech/svelte.svg"

# Get Leptos logo from GitHub (converting to SVG if needed)
download_icon "https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/logo.svg" "public/icons/tech/leptos.svg"

# Data Engineering
download_icon "https://cdn.simpleicons.org/apachespark/E25A1C" "public/icons/tech/spark.svg"
download_icon "https://cdn.simpleicons.org/databricks/FF3621" "public/icons/tech/databricks.svg"
download_icon "https://cdn.simpleicons.org/neo4j/008CC1" "public/icons/tech/neo4j.svg"
download_icon "https://cdn.simpleicons.org/postgresql/4169E1" "public/icons/tech/postgresql.svg"
download_icon "https://cdn.simpleicons.org/mysql/4479A1" "public/icons/tech/mysql.svg"
download_icon "https://cdn.simpleicons.org/mongodb/47A248" "public/icons/tech/mongodb.svg"
download_icon "https://cdn.simpleicons.org/redis/DC382D" "public/icons/tech/redis.svg"
download_icon "https://cdn.simpleicons.org/sqlite/003B57" "public/icons/tech/sqlite.svg"

# DevOps & Systems
download_icon "https://cdn.simpleicons.org/git/F05032" "public/icons/tech/git.svg"
download_icon "https://cdn.simpleicons.org/github/181717" "public/icons/tech/github.svg"
download_icon "https://cdn.simpleicons.org/gitlab/FC6D26" "public/icons/tech/gitlab.svg"
download_icon "https://cdn.simpleicons.org/nixos/5277C3" "public/icons/tech/nixos.svg"
download_icon "https://cdn.simpleicons.org/linux/FCC624" "public/icons/tech/linux.svg"
download_icon "https://cdn.simpleicons.org/windows/0078D4" "public/icons/tech/windows.svg"
download_icon "https://cdn.simpleicons.org/apple/000000" "public/icons/tech/apple.svg"
download_icon "https://cdn.simpleicons.org/docker/2496ED" "public/icons/tech/docker.svg"
download_icon "https://cdn.simpleicons.org/kubernetes/326CE5" "public/icons/tech/kubernetes.svg"

# Editors
download_icon "https://cdn.simpleicons.org/helix/281733" "public/icons/tech/helix.svg"
download_icon "https://cdn.simpleicons.org/neovim/57A143" "public/icons/tech/neovim.svg"
download_icon "https://cdn.simpleicons.org/visualstudiocode/007ACC" "public/icons/tech/vscode.svg"
download_icon "https://cdn.simpleicons.org/vscodium/2F80ED" "public/icons/tech/vscodium.svg"
download_icon "https://cdn.simpleicons.org/zedindustries/084CCF" "public/icons/tech/zed.svg"
download_icon "https://cdn.simpleicons.org/typst/239DAD" "public/icons/tech/typst.svg"

# Terminal
download_icon "https://cdn.simpleicons.org/powershell/5391FE" "public/icons/tech/powershell.svg"
download_icon "https://cdn.simpleicons.org/starship/DD0B78" "public/icons/tech/starship.svg"
download_icon "https://cdn.simpleicons.org/alacritty/F46D01" "public/icons/tech/alacritty.svg"
download_icon "https://cdn.simpleicons.org/wezterm/4E49EE" "public/icons/tech/wezterm.svg"

# Cloud & Services
download_icon "https://cdn.simpleicons.org/amazonaws/FF9900" "public/icons/tech/aws.svg"
download_icon "https://cdn.simpleicons.org/googlecloud/4285F4" "public/icons/tech/gcp.svg"
download_icon "https://cdn.simpleicons.org/microsoftazure/0078D4" "public/icons/tech/azure.svg"
download_icon "https://cdn.simpleicons.org/vercel/000000" "public/icons/tech/vercel.svg"
download_icon "https://cdn.simpleicons.org/netlify/00C7B7" "public/icons/tech/netlify.svg"

printf "\n${CYAN}=== Downloading Social Icons ===${NC}\n"

# Social Media
download_icon "https://cdn.simpleicons.org/github/181717" "public/icons/social/github.svg"
download_icon "https://cdn.simpleicons.org/x/000000" "public/icons/social/twitter.svg"
download_icon "https://cdn.simpleicons.org/linkedin/0A66C2" "public/icons/social/linkedin.svg"
download_icon "https://cdn.simpleicons.org/gmail/EA4335" "public/icons/social/email.svg"
download_icon "https://cdn.simpleicons.org/stackoverflow/F58025" "public/icons/social/stackoverflow.svg"
download_icon "https://cdn.simpleicons.org/discord/5865F2" "public/icons/social/discord.svg"
download_icon "https://cdn.simpleicons.org/mastodon/6364FF" "public/icons/social/mastodon.svg"
download_icon "https://cdn.simpleicons.org/dev.to/0A0A0A" "public/icons/social/devto.svg"
download_icon "https://cdn.simpleicons.org/medium/000000" "public/icons/social/medium.svg"
download_icon "https://cdn.simpleicons.org/hashnode/2962FF" "public/icons/social/hashnode.svg"

printf "\n${CYAN}=== Downloading UI Icons ===${NC}\n"

# UI Icons from Heroicons (via CDN)
download_icon "https://cdn.jsdelivr.net/npm/heroicons@2.0.18/24/outline/arrow-right.svg" "public/icons/ui/arrow-right.svg"
download_icon "https://cdn.jsdelivr.net/npm/heroicons@2.0.18/24/outline/arrow-up-right.svg" "public/icons/ui/external-link.svg"
download_icon "https://cdn.jsdelivr.net/npm/heroicons@2.0.18/24/outline/bars-3.svg" "public/icons/ui/menu.svg"
download_icon "https://cdn.jsdelivr.net/npm/heroicons@2.0.18/24/outline/x-mark.svg" "public/icons/ui/close.svg"
download_icon "https://cdn.jsdelivr.net/npm/heroicons@2.0.18/24/outline/sun.svg" "public/icons/ui/sun.svg"
download_icon "https://cdn.jsdelivr.net/npm/heroicons@2.0.18/24/outline/moon.svg" "public/icons/ui/moon.svg"
download_icon "https://cdn.jsdelivr.net/npm/heroicons@2.0.18/24/outline/check.svg" "public/icons/ui/check.svg"
download_icon "https://cdn.jsdelivr.net/npm/heroicons@2.0.18/24/outline/chevron-down.svg" "public/icons/ui/chevron-down.svg"
download_icon "https://cdn.jsdelivr.net/npm/heroicons@2.0.18/24/outline/chevron-up.svg" "public/icons/ui/chevron-up.svg"
download_icon "https://cdn.jsdelivr.net/npm/heroicons@2.0.18/24/outline/envelope.svg" "public/icons/ui/envelope.svg"
download_icon "https://cdn.jsdelivr.net/npm/heroicons@2.0.18/24/outline/link.svg" "public/icons/ui/link.svg"
download_icon "https://cdn.jsdelivr.net/npm/heroicons@2.0.18/24/outline/arrow-down-tray.svg" "public/icons/ui/download.svg"
download_icon "https://cdn.jsdelivr.net/npm/heroicons@2.0.18/24/outline/document-text.svg" "public/icons/ui/document.svg"
download_icon "https://cdn.jsdelivr.net/npm/heroicons@2.0.18/24/outline/code-bracket.svg" "public/icons/ui/code.svg"
download_icon "https://cdn.jsdelivr.net/npm/heroicons@2.0.18/24/outline/command-line.svg" "public/icons/ui/terminal.svg"
download_icon "https://cdn.jsdelivr.net/npm/heroicons@2.0.18/24/outline/rocket-launch.svg" "public/icons/ui/rocket.svg"
download_icon "https://cdn.jsdelivr.net/npm/heroicons@2.0.18/24/outline/sparkles.svg" "public/icons/ui/sparkles.svg"
download_icon "https://cdn.jsdelivr.net/npm/heroicons@2.0.18/24/outline/home.svg" "public/icons/ui/home.svg"
download_icon "https://cdn.jsdelivr.net/npm/heroicons@2.0.18/24/outline/user.svg" "public/icons/ui/user.svg"
download_icon "https://cdn.jsdelivr.net/npm/heroicons@2.0.18/24/outline/briefcase.svg" "public/icons/ui/briefcase.svg"
download_icon "https://cdn.jsdelivr.net/npm/heroicons@2.0.18/24/outline/academic-cap.svg" "public/icons/ui/academic.svg"
download_icon "https://cdn.jsdelivr.net/npm/heroicons@2.0.18/24/outline/chat-bubble-left-right.svg" "public/icons/ui/chat.svg"

printf "\n${GREEN}=== All icons downloaded! ===${NC}\n"
printf "Total: %s\n" "$(find public/icons -type f | wc -l)"
