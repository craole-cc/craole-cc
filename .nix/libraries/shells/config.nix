{lib}: let
  inherit (lib.attrsets) attrValues filterAttrs mapAttrsToList;
  inherit (lib.packages) getSystem;
  inherit (lib.shells) mergeNamespaces mkShells mkTools rust ai;
  inherit (lib.strings) concatStringsSep escapeShellArg hasInfix;

  combined = mergeNamespaces {inherit rust ai;};
  inherit (combined) mkSpec;

  mkSuite = {
    inputs,
    pkgs,
    fmt,
  }: let
    mk = args: let
      inherit (pkgs) runCommand writeText;

      spec = mkSpec ({inherit pkgs;} // args);

      tools = mkTools {
        inherit pkgs;
        includeEditor = args.includeEditor or true;
        includeWeb = args.includeWeb or false;
        includeInfo = args.includeInfo or true;
      };

      singleLine = filterAttrs (_: v: !(hasInfix "\n" v));

      scripts =
        pkgs.runCommand "tool-scripts" {
          passAsFile = ["cmds"];
          cmds = concatStringsSep "\n" (
            mapAttrsToList (name: cmd: "${name}\t${cmd}") tools.ver
            ++ mapAttrsToList (name: cmd: "${name}\t${cmd}") (singleLine tools.cmd)
          );
        } ''
          mkdir -p $out/bin
          while IFS=$'\t' read -r name cmd; do
            {
              echo '#!/bin/sh'
              echo "$cmd"
            } > "$out/bin/$name"
            chmod +x "$out/bin/$name"
          done < "$cmdsPath"
        '';

      packages =
        spec.shell.packages
        ++ (attrValues fmt.packages.${getSystem pkgs})
        ++ tools.packages
        ++ [scripts];

      shellHook = ''
        ${spec.shell.shellHook or ""}
        ${tools.aliases}
        cp ${writeText "tools-aliases" tools.aliases} .direnv/tools-aliases
        export TOOLS_ALIASES=".direnv/tools-aliases"
      '';
      env = (spec.shell.env or {}) // tools.vr3n;
      shell = spec.shell // {inherit env packages shellHook;};
    in
      spec // {inherit shell;};

    variants = {
      default = mk {};
      stable = mk {channel = "stable";};
      full = mk {
        preset = "full";
        includeWorkflow = true;
        includeWeb = true;
      };
      minimal = mk {
        preset = "minimal";
        includeAnalytics = false;
        includeEditor = false;
        includeInfo = true;
      };
    };
    namespaced = combined.mkSuite {inherit pkgs;};
  in {
    devShells = mkShells {
      inherit inputs;
      default = variants.minimal;
      shells = namespaced // variants;
    };
  };
in {
  inherit mkSpec mkSuite;
  mkDevShells = mkSuite;
}
