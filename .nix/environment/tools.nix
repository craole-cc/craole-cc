# nix/tools.nix
# All Rust dev tooling: packages, bin paths, and shell commands.
# Returns: { packages, bin, cmd, ... } (all cmd entries also exposed at top level)
{
  pkgs,
  rust,
  templates,
}: let
  inherit (pkgs.lib.attrsets) attrValues mapAttrs mapAttrsToList;
  inherit (pkgs.lib.lists) filter;
  inherit (pkgs.lib.strings) concatStringsSep hasPrefix;

  files = rec {
    list = [
      ".cargo/config.toml"
      ".envrc"
      ".gitignore"
      ".markdownlint-cli2.yaml"
      ".mise.toml"
      "mise.toml"
      ".treefmt.toml"
      "treefmt.toml"
    ];
    drop = concatStringsSep " " list;
    keep = concatStringsSep " " (filter (hasPrefix ".") list);
  };

  tools = with pkgs; {
    inherit
      #~@ Build Essentials
      gcc
      #~@ Development
      cargo-leptos
      trunk
      binaryen
      wasm-bindgen-cli_0_2_126
      dart-sass
      tailwindcss
      sqlx-cli
      sqlite
      gh
      lsof
      psmisc
      #~@ Build & Watch
      cargo-watch
      cargo-make
      bacon
      #~@ Dependencies & Security
      cargo-edit
      cargo-outdated
      cargo-audit
      cargo-deny
      #~@ Performance & Analysis
      cargo-flamegraph
      cargo-bloat
      cargo-expand
      #~@ Testing & Quality
      cargo-nextest
      cargo-tarpaulin
      #~@ Formatting
      leptosfmt
      markdownlint-cli2
      prettierd
      rustfmt
      taplo
      treefmt
      yamlfmt
      #~@ Git & Project Info
      gitui
      onefetch
      tokei
      #~@ Utilities
      gawk
      direnv
      gum
      mise
      trashy
      ;
    #~@ Editor
    inherit helix;
    inherit (jetbrains) rust-rover;
  };

  bin =
    mapAttrs
    (name: drv: "${drv}/bin/${drv.meta.mainProgram or name}")
    tools
    // {
      cargo = "${rust}/bin/cargo";
      rustc = "${rust}/bin/rustc";
      sqlx = "${tools.sqlx-cli}/bin/cargo-sqlx";
      sqlite3 = "${tools.sqlite}/bin/sqlite3";
    };

  cmd = {
    inherit rust;
    awk = bin.gawk;
    audit = "${bin.cargo} audit";
    baconv = ''
      ${bin.bacon} --version 2>&1 |
        ${cmd.awk} '{print substr($2, 1)}'
    '';
    bench = "${bin.cargo} bench";
    clippy = "${bin.cargo} clippy -- -D warnings";
    coverage = "${bin.cargo} tarpaulin --out Html --output-dir coverage";
    edit = "\"$VISUAL\" \"$PWD\" > /dev/null 2>&1 & disown";
    green = ''${bin.gum} style --foreground=82'';
    grey = ''${bin.gum} style --foreground=250'';
    gt = "${bin.gitui}";
    info = "${bin.tokei}; ${bin.onefetch}";
    init = ''
      #> Deploy templates using the deploy-templates script
      deploy-templates

      #> Make files writable
      chmod +w ${files.keep} 2>/dev/null || true

      #> Remove cached files from git
      git rm -r --cached .direnv target 2>/dev/null || true
      git rm --cached ${files.drop} 2>/dev/null || true

      #> Allow direnv if needed
      if ! direnv status 2>/dev/null | rg -q "Found RC allowed 2"; then
        ${cmd.yn} "Allow direnv?" && direnv allow .envrc 2>/dev/null || true
      fi
    '';
    leptosfmtv = ''${bin.leptosfmt} --version 2>&1 | cut -d ' ' -f2'';
    lint = ''
      ${cmd.init}
      ${cmd.yn} "Proceed with linting?" ||
        { ${cmd.yellow} "Linting cancelled."; exit 0; }
      failed=0
      ${bin.treefmt}              || failed=1
      ${bin.leptosfmt} **/*.rs    || failed=1
      ${bin.cargo} clippy         || failed=1
      exit $failed
    '';
    magenta = ''${bin.gum} style --foreground=212'';
    markdownlint = bin.markdownlint-cli2;
    markdownlintv = ''
      ${bin.markdownlint-cli2} --version 2>&1 |
        head -1 | ${cmd.awk} '{print substr($2, 2)}'
    '';
    misev = ''${bin.mise} version 2>/dev/null | rg -o '^[0-9]+\.[0-9]+\.[0-9]+'  '';
    prettiest = bin.prettierd;
    prettiestv = ''${cmd.prettiest} --version 2>&1 | cut -d ' ' -f2'';
    red = ''${bin.gum} style --foreground=196'';
    reload = ''${bin.direnv} reload'';
    reset = ''
      ${cmd.yn} "Clean cargo build cache?" && ${bin.cargo} clean
      ${cmd.yn} "Remove lock files? (flake.lock + Cargo.lock)" && {
        ${cmd.trash} flake.lock Cargo.lock 2>/dev/null || true
      }
      ${cmd.yn} "Config files will be re-generated. Continue?" && {
        for f in .direnv ${files.drop}; do
          ${cmd.trash} "$f" 2>/dev/null || true
        done
        [ -d .cargo ] && [ -z "$(ls -A .cargo)" ] && \
          ${cmd.trash} .cargo 2>/dev/null || true
        ${cmd.init}
      }
    '';
    rustv = "${bin.rustc} --version | cut -d ' ' -f2";
    rustvv = "${bin.rustc} --version | cut -d ' ' -f2-";
    rr = bin.rust-rover;
    rrv = ''
      ${cmd.rr} --version 2>&1 | head -n1 |
        ${cmd.awk} '{print substr($2, 1)}'
    '';
    test = "${bin.cargo} nextest run";
    sqlx = "${bin.sqlx}";
    sqlite3 = "${bin.sqlite3}";
    port = ''
    port="''${1:-3000}"
    ${bin.lsof} -nP -iTCP:"$port" -sTCP:LISTEN || {
      ${cmd.yellow} "No process is listening on TCP port $port."
    }
    '';
    kill-port = ''
    port="''${1:-3000}"
    pids="$(${bin.lsof} -tiTCP:"$port" -sTCP:LISTEN 2>/dev/null || true)"
    if [ -z "$pids" ]; then
      ${cmd.yellow} "No process is listening on TCP port $port."
      exit 0
    fi
    ${cmd.yellow} "Killing TCP port $port listener(s): $pids"
    kill $pids
    '';
    kill-3000 = ''
      exec kill-port 3000 "$@"
    '';
    leptoswatch = ''
    port="''${1:-3000}"
    if [ -z "$port" ]; then
      port=3000
    else
      case "$port" in
        *[!0-9]*)
          port=3000
          ;;
        *)
          shift
          ;;
      esac
    fi

    reload_port=$((port + 1))
    occupied_ports=""
    occupied_pids=""

    for check_port in "$port" "$reload_port"; do
      pids="$(${bin.lsof} -tiTCP:"$check_port" -sTCP:LISTEN 2>/dev/null || true)"
      if [ -n "$pids" ]; then
        occupied_ports="''${occupied_ports}''${occupied_ports:+ }$check_port"
        occupied_pids="''${occupied_pids}''${occupied_pids:+ }$pids"
        ${bin.lsof} -nP -iTCP:"$check_port" -sTCP:LISTEN || true
      fi
    done

    if [ -n "$occupied_pids" ]; then
      printf '\nPort(s) in use: %s\n' "$occupied_ports"
      printf 'Kill listener(s) and continue with cargo leptos watch? [y/N] '
      read -r answer
      case "$answer" in
        y|Y|yes|YES)
          printf 'Killing listener PID(s): %s\n' "$occupied_pids"
          kill $occupied_pids
          for _ in 1 2 3 4 5 6 7 8 9 10; do
            remaining=""
            for check_port in "$port" "$reload_port"; do
              remaining="''${remaining} $(${bin.lsof} -tiTCP:"$check_port" -sTCP:LISTEN 2>/dev/null || true)"
            done
            if [ -z "$(printf '%s' $remaining)" ]; then
              break
            fi
            sleep 1
          done
          ;;
        *)
          printf 'Aborted. Run `port %s` or `kill-port %s` to inspect/clear manually.\n' "$port" "$port"
          exit 1
          ;;
      esac
    fi

    printf 'Starting cargo leptos watch on http://127.0.0.1:%s (reload port %s)\n' "$port" "$reload_port"
    LEPTOS_SITE_ADDR="127.0.0.1:$port" \
      LEPTOS_RELOAD_PORT="$reload_port" \
      exec ${bin.cargo} leptos watch "$@"
    '';
    trash = bin.trashy;
    treefmtv = ''
      ${bin.treefmt} --version 2>&1 |
        ${cmd.awk} '{print substr($2, 2)}'
    '';
    update = ''
      ${cmd.yellow} "Updating flake inputs..."
      nix flake update
      if [ -f Cargo.toml ]; then
        ${cmd.green} "Updating cargo dependencies..."
        ${bin.cargo} update
      else
        ${cmd.grey} "No Cargo.toml found, skipping cargo update."
      fi
      ${cmd.magenta} "Done! Reloading shell..."
      direnv reload
    '';
    version = ''
      bacon=$(${cmd.baconv})
      leptosfmt=$(${cmd.leptosfmtv})
      markdownlint=$(${cmd.markdownlintv})
      mise=$(${cmd.misev})
      prettier=$(${cmd.prettiestv})
      rust=$(${cmd.rustvv})
      rover=$(${cmd.rrv})
      treefmt=$(${cmd.treefmtv})
      ${cmd.green} "         Bacon |> $bacon"
      ${cmd.green} "     Leptosfmt |> $leptosfmt"
      ${cmd.green} "  Markdownlint |> $markdownlint"
      ${cmd.green} "          Mise |> $mise"
      ${cmd.green} "      Prettier |> $prettier"
      ${cmd.green} "          Rust |> $rust"
      ${cmd.green} "    Rust-Rover |> $rover"
      ${cmd.green} "       Treefmt |> $treefmt"
    '';
    watch = "${bin.cargo} watch --quiet --clear --exec";
    watch-lint = "${cmd.watch} 'clippy -- -D warnings'";
    watch-run = "${cmd.watch} 'run'";
    watch-test = "${cmd.watch} 'nextest run'";
    yellow = ''${bin.gum} style --foreground=226'';
    yn = ''${bin.gum} confirm'';
  };

  packages =
    [rust]
    ++ (attrValues tools)
    ++ (
      mapAttrsToList
      (name: val: pkgs.writeShellScriptBin name ''${val} "$@"'')
      (removeAttrs cmd ["rust"])
    );
in
  {inherit packages;} // cmd
