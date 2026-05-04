{
  lib,
  paths,
  ...
}: let
  inherit (lib.attrsets) attrValues mapAttrs' nameValuePair optionalAttrs;
  inherit (lib.lists) concatMap flatten;
  inherit (lib.packages) mkBins mkPkgs mkVr3n;
  inherit (lib.shells) mkPackage mkPackagesFrom;
  inherit (lib.strings) mkStyledOutput;

  /**
    Build a unified tool environment for dev shells.

    Produces `packages` (a list of derivations placed on PATH) from three
    optional groups — info, web, and editor — each controlled by an
    `include*` flag. All commands are proper scripts, no aliases needed.

    # Type
  ```nix
    mkTools :: {
      pkgs          :: AttrSet;
      includeEditor :: bool;
      includeWeb    :: bool;
      includeInfo   :: bool;
    } -> {
      packages :: [derivation];
      print    :: AttrSet;
    }
  ```

    # Examples
  ```nix
    tools = mkTools { inherit pkgs; includeWeb = true; };
    # tools.packages — add to shell packages list
    # tools.print    — gum-based styled output helpers
  ```
  */
  mkTools = {
    pkgs ? mkPkgs {},
    includeEditor ? true,
    includeWeb ? false,
    includeInfo ? true,
  }: let
    mkBin = pkgs.writeShellScriptBin;
    inherit (pkgs.stdenv) isLinux;

    print = mkStyledOutput {inherit pkgs;};
    groups = {
      /**
      Core info/navigation/git/file tooling.

      Scripts are built in two passes: `bin` is first resolved from
      packages, then augmented with sibling script paths via `mkBins
      scripts`, allowing scripts to reference each other by name.
      */
      info = optionalAttrs includeInfo (
        let
          packages =
            {
              inherit
                (pkgs)
                bat
                direnv
                fastfetch
                fd
                git
                gitui
                gnused
                gum
                jq
                lsd
                mise
                nitch
                nixd
                onefetch
                ripgrep-all
                sd
                tokei
                trashy
                undollar
                ;
            }
            // optionalAttrs isLinux {
              inherit (pkgs) xclip wl-clipboard xsel;
            };

          bin = {
            packages =
              mkBins packages
              // optionalAttrs isLinux {
                wl-copy = "${pkgs.wl-clipboard}/bin/wl-copy";
              };
            scripts = mkBins scripts;
            all = bin.packages // bin.scripts;
          };

          scripts = let
            auto =
              if paths ? scripts && paths.scripts ? src
              then
                mkPackagesFrom {
                  inherit pkgs;
                  dir = paths.scripts.src;
                }
              else {};
            manual = with bin.packages; {
              #~@ Navigation
              fetch = mkBin "fetch" ''${fastfetch} "$@"'';
              ls = mkBin "ls" ''${lsd} "$@"'';
              ll = mkBin "ll" ''${lsd} --long --git --almost-all "$@"'';
              lt = mkBin "lt" ''${lsd} --tree "$@"'';
              lr = mkBin "lr" ''${lsd} --long --git --recursive "$@"'';
              ff = mkBin "ff" ''${fd} "$@"'';
              rg = mkBin "rg" ''${ripgrep-all} "$@"'';

              #~@ Project Info
              prjfo = mkBin "prjfo" ''
                ${tokei}
                ${onefetch} \
                  --no-art \
                  --no-title \
                  --no-color-palette \
                  --nerd-fonts \
                  --hide-token \
                  --number-separator comma
              '';

              #~@ Git
              gt = mkBin "gt" ''${gitui} "$@"'';
              glog = mkBin "glog" ''git log -1 --pretty=%B'';

              gcp = mkPackage {
                inherit pkgs;
                name = "gcp";
                env = {CMD_GIT = git;};
              };

              #~@ Clipboard
              clip = mkBin "clip" ''
                if [ -n "$WAYLAND_DISPLAY" ]; then
                  exec ${wl-copy} "$@"
                elif [ -n "$DISPLAY" ]; then
                  exec ${xclip} -selection clipboard "$@"
                else
                  exec pbcopy "$@"
                fi
              '';

              #~@ Files
              bat-plain = mkBin "batp" ''exec ${bat} --paging=never --plain "$@"'';
              fclip = mkBin "fclip" ''
                for f in "$@"; do
                  printf 'File: %s\n' "$f"
                  bat-plain "$f"
                done | clip
              '';

              #~@ Command Inspection
              cmd-loc = mkBin "cmd-loc" ''
                if [ "$#" -eq 0 ]; then
                  printf 'Usage: cmd-loc COMMAND...\n' >&2
                  exit 2
                fi

                status=0

                for cmd in "$@"; do
                  path="$(command -v "$cmd" 2>/dev/null || true)"

                  if [ -z "$path" ]; then
                    printf 'Command not found: %s\n' "$cmd" >&2
                    status=1
                    continue
                  fi

                  printf '%s\n' "$path"
                done

                exit "$status"
              '';
              cmd-src = mkBin "cmd-src" ''
                if [ "$#" -eq 0 ]; then
                  printf 'Usage: cmd-src COMMAND...\n' >&2
                  exit 2
                fi

                status=0

                for cmd in "$@"; do
                  path="$(command -v "$cmd" 2>/dev/null || true)"

                  if [ -z "$path" ]; then
                    printf 'Command not found: %s\n' "$cmd" >&2
                    status=1
                    continue
                  fi

                  ${bat} "$path" || status=$?
                done

                exit "$status"
              '';
              cmd-cp = mkBin "cmd-cp" ''
                if [ "$#" -eq 0 ]; then
                  printf 'Usage: cmd-cp COMMAND...\n' >&2
                  exit 2
                fi

                status=0
                first=1

                {
                  for cmd in "$@"; do
                    path="$(command -v "$cmd" 2>/dev/null || true)"

                    if [ -z "$path" ]; then
                      printf 'Command not found: %s\n' "$cmd" >&2
                      status=1
                      continue
                    fi

                    # spacing between entries
                    if [ "$first" -eq 0 ]; then
                      printf '\n\n'
                    fi
                    first=0

                    printf '# cmd: %s (%s)\n\n' "$cmd" "$path"
                    cat "$path"
                  done
                } | clip

                exit "$status"
              '';
              # cmd-cp = mkBin "cmd-cp" ''
              #   if [ "$#" -eq 0 ]; then
              #     printf 'Usage: cmd-cp COMMAND...\n' >&2
              #     exit 2
              #   fi

              #   status=0

              #   {
              #     for cmd in "$@"; do
              #       path="$(command -v "$cmd" 2>/dev/null || true)"

              #       if [ -z "$path" ]; then
              #         printf 'Command not found: %s\n' "$cmd" >&2
              #         status=1
              #         continue
              #       fi

              #       printf '# Command: %s\n' "$cmd"
              #       printf '# Path: %s\n' "$path"
              #       cat "$path"
              #     done
              #   } | clip

              #   exit "$status"
              # '';

              #~@ Nix
              reload = mkBin "reload" ''
                gcp "$@"
                ${direnv} reload
              '';

              format = mkBin "format" ''
                gcp "$@"
                nix fmt
              '';

              update = mkPackage {
                inherit pkgs;
                name = "update";
                env = {
                  CMD_DIRENV = direnv;
                  CMD_MISE = mise;
                };
              };

              #~@ Script Helpers
              find_cmd = mkBin "find_cmd" ''
                command -v "$1" 2>/dev/null || true
              '';

              require_cmd = mkBin "require_cmd" ''
                cmd="$(command -v "$1" 2>/dev/null || true)"
                [ -n "''${cmd}" ] || {
                  printf 'Error: required command not found: %s\n' "$1" >&2
                  exit 1
                }
                printf '%s' "''${cmd}"
              '';

              is_true = mkBin "is_true" ''
                case "$(printf '%s' "''${1:-}" | tr '[:upper:]' '[:lower:]')" in
                1 | yes | true | on | enable*) exit 0 ;;
                *) exit 1 ;;
                esac
              '';

              #~@ Versions
              vr3n_bat = mkBin "vr3n_bat" ''${mkVr3n bat {}}'';
              vr3n_direnv = mkBin "vr3n_direnv" ''${mkVr3n direnv {field = 1;}}'';
              vr3n_fd = mkBin "vr3n_fd" ''${mkVr3n fd {}}'';
              vr3n_gum = mkBin "vr3n_gum" ''${mkVr3n gum {
                  head = true;
                  field = 3;
                }}'';
              vr3n_gitui = mkBin "vr3n_gitui" ''${mkVr3n gitui {}}'';
              vr3n_jq = mkBin "vr3n_jq" ''${jq} --version 2>&1 | sed 's/jq-//' '';
              vr3n_lsd = mkBin "vr3n_lsd" ''${mkVr3n lsd {}}'';
              vr3n_mise = mkBin "vr3n_mise" ''
                ${mise} version 2>/dev/null | grep -o '^[0-9]*\.[0-9]*\.[0-9]*'
              '';
              vr3n_onefetch = mkBin "vr3n_onefetch" ''${mkVr3n onefetch {}}'';
              vr3n_nitch = mkBin "vr3n_nitch" ''${mkVr3n nitch {field = 3;}}'';
              vr3n_nixd = mkBin "vr3n_nixd" ''${mkVr3n nixd {}}'';
              vr3n_rg = mkBin "vr3n_rg" ''${mkVr3n ripgrep-all {}}'';
              vr3n_sd = mkBin "vr3n_sd" ''${mkVr3n sd {}}'';
              vr3n_trashy = mkBin "vr3n_trashy" ''${mkVr3n trashy {}}'';
              vr3n_tokei = mkBin "vr3n_tokei" ''${mkVr3n tokei {}}'';
            };
            printers =
              mapAttrs'
              (name: command:
                nameValuePair
                "print_${name}"
                (mkBin "print_${name}" ''
                  exec ${command} "$@"
                ''))
              (removeAttrs print ["gum"]);
          in
            auto // manual // printers;
        in {inherit scripts packages;}
      );

      /**
      Web development tooling: Deno and Prettier.
      Enabled when `includeWeb = true`.
      */
      web = optionalAttrs includeWeb (
        let
          packages = {inherit (pkgs) deno prettierd;};
          bin = mkBins packages;
          scripts = with bin; {
            vr3n_deno = mkBin "vr3n_deno" ''${mkVr3n deno {head = true;}}'';
            vr3n_prettierd = mkBin "vr3n_prettierd" ''${mkVr3n prettierd {field = 1;}}'';
          };
        in {inherit scripts packages;}
      );

      /**
      Editor tooling: Helix.
      Enabled when `includeEditor = true`.
      */
      editor = optionalAttrs includeEditor (
        let
          packages = {inherit (pkgs) helix;};
          bin = {helix = "${pkgs.helix}/bin/hx";};
          scripts = with bin; {
            vr3n_helix = mkBin "vr3n_helix" ''${mkVr3n helix {head = true;}}'';
          };
        in {inherit scripts packages;}
      );
    };
  in {
    inherit print;
    packages = flatten (
      concatMap
      (g: attrValues (g.packages or {}) ++ attrValues (g.scripts or {}))
      (attrValues groups)
    );
  };
in {inherit mkTools;}
