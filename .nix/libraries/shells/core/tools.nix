{lib}: let
  inherit
    (lib.attrsets)
    attrValues
    mapAttrs
    mapAttrs'
    nameValuePair
    optionalAttrs
    mapAttrsToList
    ;
  inherit (lib.lists) flatten filter foldl';
  inherit (lib.packages) mkBins mkPkgs mkVr3n;
  inherit (lib.strings) concatStringsSep escapeShellArg mkStyledOutput replaceStrings;
  inherit (lib.trivial) isNotEmpty;

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
          aliases = concatStringsSep "\n" (
            mapAttrsToList (name: value: "alias ${name}=${escapeShellArg value}") cmd
          );
          ver = {
            vr3n_gitui = mkVr3n bin.gitui {};
            vr3n_onefetch = mkVr3n bin.onefetch {};
            vr3n_tokei = mkVr3n bin.tokei {};
            vr3n_direnv = mkVr3n bin.direnv {field = 1;};
            vr3n_gum = mkVr3n bin.gum {head = true;};
            vr3n_trashy = mkVr3n bin.trashy {};
            vr3n_mise = mkVr3n bin.mise {
              custom = ''${bin.mise} version 2>/dev/null | grep -o '^[0-9]*\.[0-9]*\.[0-9]*' '';
            };
          };
        in {
          inherit aliases bin cmd ver;
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
    # aliases = concatStringsSep "\n" (
    #   filter isNotEmpty (map (g: g.aliases or "") (attrValues groups))
    # );
    # aliases = concatStringsSep "\n" (
    #   mapAttrsToList (name: value: "alias ${name}=${escapeShellArg value}") cmd
    #   ++ mapAttrsToList (name: value: "alias ${name}=${escapeShellArg value}") ver
    # );
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
