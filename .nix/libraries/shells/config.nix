{lib}: let
  inherit (lib.attrsets) genAttrs isDerivation mapAttrs;
  inherit (lib.packages) mkPkgsPerSystem;
  inherit (lib.shells) mkShell rust;

  mkSuite = {
    inputs,
    ...
  }:
    let
      packages = mkPkgsPerSystem {inherit inputs;};

      mkSystemShells = system: pkgs: let
        specs = rust.mkSuite {inherit pkgs;};
        process = spec:
          if isDerivation spec
          then spec
          else mkShell {
            inherit inputs pkgs system;
            shell = spec.shell or spec;
          };
      in
        (mapAttrs (_: process) specs)
        // {
          default = process specs.rust-nightly-lean;
        };
    in {
      devShells = genAttrs (builtins.attrNames packages) (
        system: mkSystemShells system packages.${system}
      );
    };
in {
  mkDevShells = mkSuite;
}
