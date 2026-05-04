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
  inherit (builtins) getEnv;

  mkTools = {
    pkgs ? mkPkgs {},
    includeEditor ? true,
    includeWeb ? false,
    includeInfo ? true,
  }: let
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
                fd #? Fast find alternative
                gnused #? GNU stream editor
                jq #? JSON query processor
                lsd #? LSDeluxe file lister
                nitch # ? System fetch written in nim
                nixd # ? Nix language daemon
                ripgrep-all #? Fast grep alternative
                sd #? Intuitive find & replace CLI (sed alternative)
                onefetch #? Git repository summary on your terminal
                undollar #? Remove leading dollar signs
                gitui
                tokei
                direnv
                gum
                mise
                trashy
                ;
            }
            // optionalAttrs isLinux {
              inherit
                (pkgs)
                xclip
                wl-clipboard
                xsel
                ;
            };
          bin =
            (mkBins packages)
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

            #~@ Files
            clip = with bin; "$([ -n \"$WAYLAND_DISPLAY\" ] && echo \"${wl-copy}\" || echo \"${xclip} -selection clipboard\")";
            copy =
              if isLinux
              then
                if getEnv "WAYLAND_DISPLAY" != ""
                then "${bin.wl-copy}"
                else "${bin.xclip} -selection clipboard"
              else "pbcopy";
            fcat = "${bin.bat} --paging=never \"\$@\"";
            fclip = "${bin.bat} --paging=never \"\$@\" | ${cmd.clip}";
            fcopy = "${bin.bat} --paging=never \"\$@\" | ${cmd.copy}";

            #~@ Search
            ff = bin.fd;
            rg = bin.ripgrep-all;

            #~@ Shell
            reload = "${bin.direnv} reload";

            #~@ Nix
            update-flake = ''
              ${print.yellow} "Updating flake inputs..."
              nix flake update
            '';
          };
          aliases = concatStringsSep "\n" (
            mapAttrsToList
            (name: value: "alias ${name}=${escapeShellArg value}")
            cmd
          );
          ver = {
            vr3n_gitui = mkVr3n bin.gitui {};
            vr3n_onefetch = mkVr3n bin.onefetch {};
            vr3n_tokei = mkVr3n bin.tokei {};
            vr3n_direnv = mkVr3n bin.direnv {field = 1;};
            vr3n_gum = mkVr3n bin.gum {head = true;};
            vr3n_trashy = mkVr3n bin.trashy {};
            vr3n_mise = mkVr3n bin.mise {
              custom = "${bin.mise} version 2>/dev/null | grep -o '^[0-9]*\\.[0-9]*\\.[0-9]*'";
            };
            vr3n_bat = mkVr3n bin.bat {};
            vr3n_fd = mkVr3n bin.fd {};
            vr3n_jq = mkVr3n bin.jq {};
            vr3n_lsd = mkVr3n bin.lsd {};
            vr3n_rg = mkVr3n bin.ripgrep-all {};
            vr3n_sd = mkVr3n bin.sd {};
            vr3n_nitch = mkVr3n bin.nitch {};
            vr3n_nixd = mkVr3n bin.nixd {};
            vr3n_helix = mkVr3n bin.helix {head = true;};
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
