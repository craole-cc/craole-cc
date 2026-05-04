{lib}: let
  inherit (lib.attrsets) attrValues optionalAttrs;
  inherit (lib.lists) concatMap flatten foldl';
  inherit (lib.packages) mkBins mkPkgs mkVr3n mkAliases;
  inherit (lib.shells) mkPackage;
  inherit (lib.strings) mkStyledOutput;

  mkTools = {
    pkgs ? mkPkgs {},
    includeEditor ? true,
    includeWeb ? false,
    includeInfo ? true,
  }: let
    inherit (pkgs) writeShellScriptBin;
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

          aliases = with bin; {
            fetch = nitch;
            ls = lsd;
            ll = "${lsd} --long --git --almost-all";
            lt = "${lsd} --tree";
            lr = "${lsd} --long --git --recursive";
            gt = gitui;
            ff = fd;
            rg = ripgrep-all;
            vr3n_bat = mkVr3n bat {};
            vr3n_direnv = mkVr3n direnv {field = 1;};
            vr3n_fd = mkVr3n fd {};
            vr3n_gum = mkVr3n gum {
              head = true;
              field = 3;
            };
            vr3n_gitui = mkVr3n gitui {};
            vr3n_helix = mkVr3n helix {head = true;};
            vr3n_jq = mkVr3n jq {
              custom = "${jq} --version 2>&1 | sed 's/jq-//'";
            };
            vr3n_lsd = mkVr3n lsd {};
            vr3n_mise = mkVr3n mise {
              custom = "${mise} version 2>/dev/null | grep -o '^[0-9]*\\.[0-9]*\\.[0-9]*'";
            };
            vr3n_onefetch = mkVr3n onefetch {};
            vr3n_nitch = mkVr3n nitch {field = 3;};
            vr3n_nixd = mkVr3n nixd {};
            vr3n_rg = mkVr3n ripgrep-all {};
            vr3n_sd = mkVr3n sd {};
            vr3n_trashy = mkVr3n trashy {};
            vr3n_tokei = mkVr3n tokei {};
          };

          scripts = {
            prjfo = writeShellScriptBin "prjfo" ''
              ${bin.tokei}
              ${bin.onefetch}
            '';
            gcp = writeShellScriptBin "gcp" ''
              git add --all
              if [ -n "$(git status --porcelain)" ]; then
                msg="''${*:-$(git log -1 --pretty=%B 2>/dev/null | head -1)}"
                git commit --message "$msg"
                git push
              fi
            '';
            clip = writeShellScriptBin "clip" ''
              if [ -n "$WAYLAND_DISPLAY" ]; then
                exec ${bin.wl-copy} "$@"
              elif [ -n "$DISPLAY" ]; then
                exec ${bin.xclip} -selection clipboard "$@"
              else
                exec pbcopy "$@"
              fi
            '';
            batp = writeShellScriptBin "batp" ''
              exec ${bin.bat} --paging=never --plain "$@"
            '';
            fclip = writeShellScriptBin "fclip" ''
              ${bin.bat} --paging=never --style=header "$@" | clip
            '';
            glog = writeShellScriptBin "glog" ''
              git log -1 --pretty=%B
            '';
            reload = writeShellScriptBin "reload" ''
              gcp "$@"
              ${bin.direnv} reload
            '';
            format = writeShellScriptBin "format" ''
              gcp "$@"
              nix fmt
            '';
            update = mkPackage {
              inherit pkgs;
              name = "update";
              script = "update";
              env = {
                DIRENV = bin.direnv;
                MISE = bin.mise;
              };
            };
          };
        in {inherit aliases bin scripts packages;}
      );

      web = optionalAttrs includeWeb (
        let
          packages = {inherit (pkgs) deno prettierd;};
          bin = mkBins packages;
          aliases = with bin; {
            vr3n_deno = mkVr3n deno {head = true;};
            vr3n_prettier = mkVr3n prettierd {field = 1;};
          };
        in {inherit aliases packages;}
      );

      editor = optionalAttrs includeEditor (
        let
          packages = {inherit (pkgs) helix;};
          bin = {
            helix = "${pkgs.helix}/bin/hx";
          };
          aliases = with bin; {
            vr3n_helix = mkVr3n helix {head = true;};
          };
        in {inherit aliases packages;}
      );
    };
  in {
    inherit print;

    aliases = mkAliases (
      foldl'
      (acc: g: acc // (g.aliases or {}))
      {}
      (attrValues groups)
    );

    packages = flatten (
      concatMap
      (g: attrValues (g.packages or {}) ++ attrValues (g.scripts or {}))
      (attrValues groups)
    );
  };
in {inherit mkTools;}
