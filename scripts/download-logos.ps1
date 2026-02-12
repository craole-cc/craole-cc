# Create logos directory
New-Item -ItemType Directory -Force -Path "public/logos"

# Download function
function Download-Logo {
    param($url, $filename)
    Write-Host "Downloading $filename..."
    try {
        Invoke-WebRequest -Uri $url -OutFile "public/logos/$filename"
        Write-Host "✓ $filename downloaded" -ForegroundColor Green
    } catch {
        Write-Host "✗ Failed to download $filename" -ForegroundColor Red
    }
}

# Languages & Core
Download-Logo "https://cdn.simpleicons.org/rust/000000" "rust.svg"
Download-Logo "https://cdn.simpleicons.org/gnubash/036882" "bash.svg"
Download-Logo "https://cdn.simpleicons.org/python/3776AB" "python.svg"
Download-Logo "https://cdn.simpleicons.org/zig/F7A41D" "zig.svg"

# Web
Download-Logo "https://avatars.githubusercontent.com/u/20248544?s=48&v=4" "tokio.png"
Download-Logo "https://cdn.simpleicons.org/htmx/3366CC" "htmx.svg"
Download-Logo "https://avatars.githubusercontent.com/u/118319153?s=48&v=4" "leptos.png"
Download-Logo "https://tailwindcss.com/_next/static/media/tailwindcss-mark.96ee6a5a.svg" "tailwind.svg"

# Data Engineering
Download-Logo "https://cdn.simpleicons.org/apachespark/E25A1C" "spark.svg"
Download-Logo "https://cdn.simpleicons.org/databricks/FF3621" "databricks.svg"
Download-Logo "https://cdn.simpleicons.org/neo4j/008CC1" "neo4j.svg"
Download-Logo "https://cdn.simpleicons.org/postgresql/316192" "postgresql.svg"

# DevOps & Systems
Download-Logo "https://cdn.simpleicons.org/git/F05032" "git.svg"
Download-Logo "https://cdn.simpleicons.org/github/181717" "github.svg"
Download-Logo "https://nixos.wiki/images/thumb/2/20/Home-nixos-logo.png/207px-Home-nixos-logo.png" "nixos.png"
Download-Logo "https://upload.wikimedia.org/wikipedia/commons/8/87/Windows_logo_-_2021.svg" "windows.svg"

# Editors
Download-Logo "https://helix-editor.com/logo.svg" "helix.svg"
Download-Logo "https://cdn.simpleicons.org/typst/239DAD" "typst.svg"
Download-Logo "https://code.visualstudio.com/assets/branding/code-stable.png" "vscode.png"
Download-Logo "https://zed.dev/logo_icon.webp" "zed.webp"

# Terminal
Download-Logo "https://cdn.simpleicons.org/gnubash/4EAA25" "bash-terminal.svg"
Download-Logo "https://raw.githubusercontent.com/gist/Xainey/d5bde7d01dcbac51ac951810e94313aa/raw/6c858c46726541b48ddaaebab29c41c07a196394/PowerShell.svg" "powershell.svg"
Download-Logo "https://cdn.simpleicons.org/starship/DD0B78" "starship.svg"
Download-Logo "https://ohmyposh.dev/img/logo-dark.svg" "ohmyposh.svg"

Write-Host "`nAll logos downloaded to public/logos/" -ForegroundColor Cyan
