{lib}: let
  inherit
    (lib.attrsets)
    attrValues
    mapAttrs
    mapAttrs'
    nameValuePair
    optionalAttrs
    ;
  inherit (lib.lists) flatten foldl';
  inherit (lib.packages) mkBins mkPkgs mkVr3n;
  inherit (lib.strings) mkStyledOutput replaceStrings;
  # inherit (lib.trivial) isNotEmpty;

  mkKeys = f: attrs: mapAttrs' (k: v: nameValuePair (f k) v) attrs;

  mkTools = {
    pkgs ? mkPkgs {},
    minimal ? false,
    includeEditor ? true,
    includeWeb ? false,
    includeInfo ? true,
  }: let
    print = mkStyledOutput {inherit pkgs;};

    groups = {
      info = optionalAttrs (includeInfo && !minimal) (
        let
          packages = {inherit (pkgs) gitui onefetch tokei direnv gum mise trashy;};
          bin = mkBins packages;
          cmd = {
            info = with bin; "${tokei}; ${onefetch}";
            gt = bin.gitui;
            reload = "${bin.direnv} reload";
            update-flake = ''
              ${print.yellow} "Updating flake inputs..."
              nix flake update
            '';
          };
          ver = {
            gitui = mkVr3n bin.gitui {};
            onefetch = mkVr3n bin.onefetch {};
            tokei = mkVr3n bin.tokei {};
            direnv = mkVr3n bin.direnv {field = 1;};
            gum = mkVr3n bin.gum {head = true;};
            trashy = mkVr3n bin.trashy {};
            mise = mkVr3n bin.mise {
              custom = ''${bin.mise} version 2>/dev/null | grep -o '^[0-9]*\.[0-9]*\.[0-9]*' '';
            };
          };
        in {
          inherit bin cmd ver;
          pkgs = packages;
        }
      );

      web = optionalAttrs (includeWeb && !minimal) (
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

      editor = optionalAttrs (includeEditor && !minimal) (
        let
          packages = {inherit (pkgs) helix;};
          bin = mkBins packages;
          cmd = {};
          ver = {helix = mkVr3n bin.helix {head = true;};};
        in {
          inherit bin cmd ver;
          pkgs = packages;
        }
      );
    };

    mergeAttr = set:
      foldl' (acc: grp: acc // (grp.${set} or {})) {} (attrValues groups);

    bin = mergeAttr "bin";
    cmd = mergeAttr "cmd";
    ver = mergeAttr "ver";
    vr3n = mkKeys (pkg: "vr3n_${replaceStrings ["-"] ["_"] pkg}") ver;
    packages = flatten (map attrValues (attrValues (mapAttrs (_: g: g.pkgs or {}) groups)));
  in {inherit bin cmd packages print ver vr3n;};
in {inherit mkTools;}
