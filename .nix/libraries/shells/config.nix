{lib}: let
  inherit (lib.attrsets) attrValues;
  inherit (lib.packages) getSystem;
  inherit (lib.shells) mergeNamespaces mkShells mkTools rust ai;

  combined = mergeNamespaces {inherit rust ai;};
  inherit (combined) mkSpec;

  mkSuite = {
    inputs,
    pkgs,
    fmt,
  }: let
    mk = args: let
      tools = mkTools {inherit pkgs;};
      spec = mkSpec ({inherit pkgs;} // args);
      # shellHook = spec.shell.shellHook or "";
      shellHook = "";
      env = spec.shell.env or {};

      packages =
        spec.shell.packages
        ++ (attrValues fmt.packages.${getSystem pkgs})
        ++ tools.packages;

      shell =
        spec.shell
        // {inherit env packages shellHook;};
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
