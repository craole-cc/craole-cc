{lib}: let
  inherit
    (lib.attrsets)
    attrValues
    mapAttrs
    optionalAttrs
    mapAttrsToList
    ;
  inherit (lib.lists) flatten foldl';
  inherit (lib.packages) mkBins mkPkgs mkVr3n;
  inherit (lib.strings) concatStringsSep escapeShellArg mkStyledOutput;

  mkTools = {
    pkgs ? mkPkgs {},
    includeEditor ? true,
    includeWeb ? false,
    includeInfo ? true,
  }: let
    inherit (pkgs) writeShellScript;
    inherit (pkgs.stdenv) isLinux;

    print = mkStyledOutput {inherit pkgs;};
    groups = {
      info = optionalAttrs includeInfo (
        let
          packages =
            {
              inherit
                (pkgs)
                bat #? Cat clone with syntax highlighting
                direnv
                fd #? Fast find alternative
                gitui
                gnused #? GNU stream editor
                gum
                jq #? JSON query processor
                lsd #? LSDeluxe file lister
                mise
                nitch # ? System fetch written in nim
                nixd # ? Nix language daemon
                onefetch #? Git repository summary on your terminal
                ripgrep-all #? Fast grep alternative
                sd #? Intuitive find & replace CLI (sed alternative)
                tokei
                trashy
                undollar #? Remove leading dollar signs
                ;
            }
            // optionalAttrs isLinux {
              inherit (pkgs) xclip wl-clipboard xsel;
            };

          bin =
            mkBins packages
            // {
              helix = "${pkgs.helix}/bin/hx";
              wl-copy = "${pkgs.wl-clipboard}/bin/wl-copy";
            };

          cmd = {
            #~@ Info
            prjfo = with bin; "${tokei}; ${onefetch}";
            fetch = bin.nitch;

            #~@ Navigation
            ls = "${bin.lsd}";
            ll = "${bin.lsd} --long --git --almost-all";
            lt = "${bin.lsd} --tree";
            lr = "${bin.lsd} --long --git --recursive";

            #~@ Git
            gt = bin.gitui;
            glog = "$(git log -1 --pretty=%B)";
            gcp = writeShellScript "git-add-commit-push" ''
              git add --all
              if [ -n "$(git status --porcelain)" ]; then
                msg="''${*:-$(git log -1 --pretty=%B 2>/dev/null | head -1)}"
                git commit --message "$msg"
                git push
              fi
            '';
            #~@ Files
            clip = with bin; "$(if [ -n \"$WAYLAND_DISPLAY\" ]; then echo \"${wl-copy}\"; elif [ -n \"$DISPLAY\" ]; then echo \"${xclip} -selection clipboard\"; else echo \"pbcopy\"; fi)";
            bat-plain = "${bin.bat} --paging=never --plain \"\$@\"";
            fclip = "${bin.bat} --paging=never --style=header \"\$@\" | ${cmd.clip}";

            #~@ Search
            ff = bin.fd;
            rg = bin.ripgrep-all;

            #~@ Nix
            reload = "${cmd.gcp} \"\$@\"; ${bin.direnv} reload";
            format = "${cmd.gcp} \"\$@\"; nix fmt";
            # update = writeShellScript "update-flake" ''
            #   ${print.yellow} "Updating flake inputs..."
            #   nix flake update
            #   ${cmd.gcp} "flake update"
            # '';
            update = "${writeShellScript "update-flake" ''
              ${print.yellow} "Updating flake inputs..."
              nix flake update
              ${cmd.gcp} "flake update"
            ''}";
          };

          aliases = concatStringsSep "\n" (
            mapAttrsToList
            (name: value: "alias ${name}=${escapeShellArg value}")
            cmd
          );
          ver = with bin; {
            vr3n_gitui = mkVr3n gitui {};
            vr3n_onefetch = mkVr3n onefetch {};
            vr3n_tokei = mkVr3n tokei {};
            vr3n_direnv = mkVr3n direnv {field = 1;};
            vr3n_gum = mkVr3n gum {
              head = true;
              field = 3;
            };
            vr3n_helix = mkVr3n helix {
              head = true;
              # field = 2;
            };
            vr3n_trashy = mkVr3n trashy {};
            vr3n_mise = mkVr3n mise {
              custom = "${mise} version 2>/dev/null | grep -o '^[0-9]*\\.[0-9]*\\.[0-9]*'";
            };
            vr3n_bat = mkVr3n bat {};
            vr3n_fd = mkVr3n fd {};
            vr3n_nitch = mkVr3n nitch {field = 3;};
            vr3n_jq = mkVr3n jq {
              custom = "${jq} --version 2>&1 | sed 's/jq-//'";
            };
            vr3n_lsd = mkVr3n lsd {};
            vr3n_rg = mkVr3n ripgrep-all {};
            vr3n_sd = mkVr3n sd {};
            vr3n_nixd = mkVr3n nixd {};
          };
        in {
          inherit aliases bin cmd ver;
          pkgs = packages;
        }
      );

      web = optionalAttrs includeWeb (
        let
          packages = {inherit (pkgs) deno prettierd;};
          bin = mkBins packages;
          cmd = {};
          ver = {
            deno = mkVr3n bin.deno {head = true;};
            prettierd = mkVr3n bin.prettierd {field = 1;};
          };
        in {
          inherit bin cmd ver;
          pkgs = packages;
        }
      );

      editor = optionalAttrs includeEditor (
        let
          packages = {inherit (pkgs) helix;};
          bin = mkBins packages;
          cmd = {};
          ver = {
            helix = mkVr3n bin.helix {head = true;};
          };
        in {
          inherit bin cmd ver;
          pkgs = packages;
        }
      );
    };

    mergeAttr = set:
      foldl' (acc: grp: acc // (grp.${set} or {})) {} (attrValues groups);
    aliases = concatStringsSep "\n" (
      mapAttrsToList (name: value: "alias ${name}=${escapeShellArg value}") cmd
      ++ mapAttrsToList (name: value: "alias ${name}=${escapeShellArg value}") ver
    );
    bin = mergeAttr "bin";
    cmd = mergeAttr "cmd";
    ver = mergeAttr "ver";
    vr3n = ver;
    packages = flatten (map attrValues (attrValues (mapAttrs (_: g: g.pkgs or {}) groups)));
  in {inherit aliases bin cmd packages print ver vr3n;};
in {inherit mkTools;}
