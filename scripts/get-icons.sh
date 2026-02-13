#!/bin/sh
# shellcheck enable=all
# force=1
#╔═══════════════════════════════════════════════════════════╗
#║ Color Output                                              ║
#╚═══════════════════════════════════════════════════════════╝
if [ -t 1 ]; then
  GREEN=$(tput setaf 2)
  RED=$(tput setaf 1)
  CYAN=$(tput setaf 6)
  YELLOW=$(tput setaf 3)
  BOLD=$(tput bold)
  NC=$(tput sgr0)
else
  GREEN=""
  RED=""
  CYAN=""
  YELLOW=""
  BOLD=""
  NC=""
fi

#╔═══════════════════════════════════════════════════════════╗
#║ Directories                                               ║
#╚═══════════════════════════════════════════════════════════╝
icons="${PWD}/public/icons"
tech="${icons}/tech"
social="${icons}/social"
ui="${icons}/ui"
mkdir -p "${tech}" "${social}" "${ui}"

#╔═══════════════════════════════════════════════════════════╗
#║ Function                                                  ║
#╚═══════════════════════════════════════════════════════════╝
download_icon() {
  url="$1"
  output="$2"
  filename=$(basename "${output}")

  printf 'Fetching %-30s ... ' "${filename}"

  #? Some "Real" sources like Wikimedia/GitHub block empty User-Agents
  if [ -f "${output}" ] && [ -z "${force:-}" ]; then
    printf '%sSKIPPED%s\n' "${YELLOW}" "${NC}"
    return
  fi
  if curl -fsSL -A "Mozilla/5.0" "${url}" -o "${output}" 2> /dev/null; then
    printf '%sOK%s\n' "${GREEN}" "${NC}"
  else
    printf '%sFAILED%s\n' "${RED}" "${NC}"
    rm -f "${output}" 2> /dev/null
  fi
}

#╔═══════════════════════════════════════════════════════════╗
#║ Icons: Technology                                         ║
#╚═══════════════════════════════════════════════════════════╝
printf '\n%s%s=== Downloading Technology Icons ===%s\n' \
  "${BOLD}" "${CYAN}" "${NC}"

# -- Languages --
download_icon \
  "https://cdn.worldvectorlogo.com/logos/rust.svg" \
  "${tech}/rust.svg"
download_icon \
  "https://cdn.worldvectorlogo.com/logos/python-5.svg" \
  "${tech}/python.svg"
download_icon \
  "https://raw.githubusercontent.com/ziglang/logo/4f97e7a9ebce12fa48511c0b6502b6190005bc0e/zig-mark.svg" \
  "${tech}/zig.svg"
download_icon \
  "https://cdn.worldvectorlogo.com/logos/go-8.svg" \
  "${tech}/go.svg"
download_icon \
  "https://cdn.worldvectorlogo.com/logos/bash-2.svg" \
  "${tech}/bash.svg"
download_icon \
  "https://cdn.worldvectorlogo.com/logos/powershell.svg" \
  "${tech}/powershell.svg"

# -- Rust Ecosystem --
download_icon \
  "https://raw.githubusercontent.com/tokio-rs/website/master/public/img/icons/tokio.svg" \
  "${tech}/tokio.svg"
download_icon \
  "https://raw.githubusercontent.com/leptos-rs/leptos/6e83f712d2d64014e000302c9cd265d4a9a61311/logos/Simple_Icon.svg" \
  "${tech}/leptos.png"
download_icon \
  "https://cdn.worldvectorlogo.com/logos/tauri-1.svg" \
  "${tech}/tauri.svg"
download_icon \
  "https://usw2-zeet-misc.s3.us-west-2.amazonaws.com/images/SurrealDB.png" \
  "${tech}/surrealdb.png"
download_icon \
  "https://raw.githubusercontent.com/surrealdb/surrealdb/main/img/logo.svg" \
  "${tech}/surrealdb.svg"

# -- Data --
download_icon \
  "https://upload.wikimedia.org/wikipedia/commons/f/f3/Apache_Spark_logo.svg" \
  "${tech}/apache-spark.svg"
download_icon \
  "https://cdn.worldvectorlogo.com/logos/scala-4.svg" \
  "${tech}/scala.svg"
download_icon \
  "https://cdn.prod.website-files.com/68c803b3497f18f5503b830d/68da505ee9382ac2316b3e67_66192bf45f99cf9cd103c8b3_delta.svg" \
  "${tech}/deltalake.svg"
download_icon \
  "https://cdn.worldvectorlogo.com/logos/kafka.svg" \
  "${tech}/kafka.svg"
download_icon \
  "https://upload.wikimedia.org/wikipedia/commons/2/29/Postgresql_elephant.svg" \
  "${tech}/postgresql.svg"
download_icon \
  "https://cdn.worldvectorlogo.com/logos/mysql-logo-pure.svg" \
  "${tech}/mysql.svg"
download_icon \
  "https://cdn.worldvectorlogo.com/logos/duckdb-logo.svg" \
  "${tech}/duckdb.svg"

# -- Web --
download_icon \
  "https://cdn.worldvectorlogo.com/logos/typescript.svg" \
  "${tech}/typescript.svg"
download_icon \
  "https://cdn.worldvectorlogo.com/logos/javascript-1.svg" \
  "${tech}/javascript.svg"
download_icon \
  "https://cdn.worldvectorlogo.com/logos/html-1.svg" \
  "${tech}/html.svg"
download_icon \
  "https://cdn.worldvectorlogo.com/logos/css-3.svg" \
  "${tech}/css.svg"
download_icon \
  "https://cdn.worldvectorlogo.com/logos/sass-1.svg" \
  "${tech}/sass.svg"
download_icon \
  "https://cdn.worldvectorlogo.com/logos/tailwind-css-2.svg" \
  "${tech}/tailwind.svg"
download_icon \
  "https://cdn.worldvectorlogo.com/logos/svelte-1.svg" \
  "${tech}/svelte.svg"
download_icon \
  "https://raw.githubusercontent.com/vitejs/vite/main/docs/public/logo.svg" \
  "${tech}/vite.svg"
download_icon \
  "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcTuW9lcdXGNSXkg7EsdpXy0wNhPz8YcGXFwRA&s" \
  "${tech}/htmx.png"
download_icon \
  "https://logo.svgcdn.com/logos/htmx-icon.png" \
  "${tech}/htmx.png"

# -- Operating System --
download_icon \
  "https://cdn.worldvectorlogo.com/logos/windows-3.svg" \
  "${tech}/windows.svg"
download_icon \
  "https://cdn.worldvectorlogo.com/logos/raspberry-pi.svg" \
  "${tech}/raspberry-pi.svg"
download_icon \
  "https://raw.githubusercontent.com/NixOS/nixos-artwork/master/logo/nixos-white.svg" \
  "${tech}/nixos.svg"
download_icon \
  "https://cdn.worldvectorlogo.com/logos/linux-tux.svg" \
  "${tech}/linux-tux.svg"
download_icon \
  "https://upload.wikimedia.org/wikipedia/commons/1/13/Arch_Linux_%22Crystal%22_icon.svg" \
  "${tech}/arch.svg"
download_icon \
  "https://cdn.worldvectorlogo.com/logos/arch-linux-logo.svg" \
  "${tech}/archlinux.svg"

# -- DevOps --
download_icon \
  "https://cdn.worldvectorlogo.com/logos/visual-studio-code-1.svg" \
  "${tech}/vscode.svg"
download_icon \
  "https://cdn.worldvectorlogo.com/logos/docker.svg" \
  "${tech}/docker-full.svg"
download_icon \
  "https://raw.githubusercontent.com/kubernetes/kubernetes/master/logo/logo.svg" \
  "${tech}/kubernetes.svg"
download_icon \
  "https://cdn.worldvectorlogo.com/logos/git-icon.svg" \
  "${tech}/git.svg"
download_icon \
  "https://cdn.worldvectorlogo.com/logos/git-bash.svg" \
  "${tech}/gitbash.svg"
download_icon \
  "https://raw.githubusercontent.com/helix-editor/helix/master/logo_dark.svg" \
  "${tech}/helix-editor.svg"
download_icon \
  "https://upload.wikimedia.org/wikipedia/commons/3/3a/Neovim-mark.svg" \
  "${tech}/neovim.svg"
download_icon \
  "https://cdn.worldvectorlogo.com/logos/vim.svg" \
  "${tech}/vim.svg"

download_icon \
  "https://cdn.worldvectorlogo.com/logos/sony-logo-1.svg" \
  "${tech}/sony.svg"
download_icon \
  "https://cdn.worldvectorlogo.com/logos/sony-alpha-logo.svg" \
  "${tech}/sony-alpha.svg"

#╔═══════════════════════════════════════════════════════════╗
#║ Icons: Social                                             ║
#╚═══════════════════════════════════════════════════════════╝
printf '\n%s%s=== Downloading Social Icons ===%s\n' "${BOLD}" "${CYAN}" "${NC}"
download_icon \
  "https://cdn.worldvectorlogo.com/logos/slack-new-logo.svg" \
  "${social}/slack.svg"
download_icon \
  "https://cdn.worldvectorlogo.com/logos/github-icon.svg" \
  "${social}/github.svg"
download_icon \
  "https://cdn.worldvectorlogo.com/logos/gitlab-3.svg" \
  "${tech}/gitlab.svg"
download_icon \
  "https://cdn.worldvectorlogo.com/logos/linkedin-icon-2.svg" \
  "${social}/linkedin.svg"
download_icon \
  "https://cdn.worldvectorlogo.com/logos/x-twitter.svg" \
  "${social}/x.svg"
download_icon \
  "https://cdn.worldvectorlogo.com/logos/facebook-modern-design-.svg" \
  "${social}/fb.svg"
download_icon \
  "https://cdn.worldvectorlogo.com/logos/meta-3.svg" \
  "${social}/meta.svg"
download_icon \
  "https://cdn.worldvectorlogo.com/logos/instagram-2016-5.svg" \
  "${social}/instagram.svg"
download_icon \
  "https://cdn.worldvectorlogo.com/logos/whatsapp-8.svg" \
  "${social}/whatsapp.svg"
download_icon \
  "https://cdn.worldvectorlogo.com/logos/bluesky-1.svg" \
  "${social}/bluesky.svg"

#╔═══════════════════════════════════════════════════════════╗
#║ Icons: UI                                                 ║
#╚═══════════════════════════════════════════════════════════╝
printf '\n%s%s=== Downloading UI Icons ===%s\n' "${BOLD}" "${CYAN}" "${NC}"
HERO_SRC="https://raw.githubusercontent.com/tailwindlabs/heroicons/master/src/24/outline"
download_icon "${HERO_SRC}/home.svg" "${ui}/home.svg"
download_icon "${HERO_SRC}/bars-3.svg" "${ui}/menu.svg"
download_icon "${HERO_SRC}/x-mark.svg" "${ui}/close.svg"
download_icon "${HERO_SRC}/magnifying-glass.svg" "${ui}/search.svg"
download_icon "${HERO_SRC}/circle-stack.svg" "${ui}/database.svg"
download_icon "${HERO_SRC}/cpu-chip.svg" "${ui}/cpu.svg"
download_icon "${HERO_SRC}/cloud.svg" "${ui}/cloud.svg"
download_icon "${HERO_SRC}/bolt.svg" "${ui}/bolt.svg"

printf '\n%s%s=== Complete ===%s\n' "${BOLD}" "${GREEN}" "${NC}"
