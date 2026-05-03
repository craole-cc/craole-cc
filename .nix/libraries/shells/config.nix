{lib}: let
  inherit (lib.attrsets) attrValues;
  inherit (lib.packages) getSystem;
  inherit (lib.shells) mergeNamespaces mkShells mkTools rust ai;
  inherit (lib.strings) concatStringsSep;

  combined = mergeNamespaces {inherit rust ai;};
  inherit (combined) mkSpec;

  mkSuite = {
    inputs,
    pkgs,
    fmt,
  }: let
    mk = args: let
      toolArgs = {
        inherit pkgs;
        minimal = args.minimal or false;
        includeEditor = args.includeEditor or true;
        includeWeb = args.includeWeb or false;
        includeInfo = args.includeInfo or true;
      };
      tools = mkTools toolArgs;
      spec = mkSpec ({inherit pkgs;} // args);
      packages =
        spec.shell.packages
        ++ (attrValues fmt.packages.${getSystem pkgs})
        ++ tools.packages;
      shellHook = ''
        ${spec.shell.shellHook or ""}
        ${concatStringsSep "\n" (attrValues tools.cmd)}
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
        includeInfo = false;
        minimal = true;
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
