{lib, ...}: let
  inherit (lib.attrsets) mapAttrs recursiveUpdate;
  inherit (lib.lists) elem toList;
  inherit (lib.packages) defineSystems resolvePackages perSystem;

  mkTreefmt = {
    flake,
    inputs,
    programs ? {},
    settings ? {},
  }: let
    inherit (resolvePackages {inherit inputs;}) treefmt;

    packages = perSystem {
      inherit inputs;
      fn = pkgs: let
        module = treefmt.lib.evalModule pkgs (
          mkTreefmtConfig {inherit pkgs programs settings;}
        );
        inherit (module.config.build) wrapper;
        formatting = module.config.build.check flake;
      in {inherit wrapper formatting;};
    };

    formatter = mapAttrs (_: v: v.wrapper) packages;
    checks = mapAttrs (_: v: {inherit (v) formatting;}) packages;
  in {
    inherit formatter checks;
  };

  mkTreefmtConfig = {
    pkgs,
    programs,
    settings,
  }: let
    inherit (pkgs.stdenv.hostPlatform) system;
    for = {
      allBut = systems: !(elem system (toList systems));
      all = {systems ? defineSystems}: (elem system (toList systems));
    };
  in
    recursiveUpdate {
      projectRootFile = "flake.nix";

      programs = {
        actionlint.enable = true;
        alejandra.enable = for.all {};
        deno.enable = for.allBut "riscv64-linux";
        leptosfmt.enable = true;
        rustfmt.enable = true;
        shellcheck.enable = true;
        shfmt.enable = true;
        sqruff.enable = for.allBut ["riscv64-linux" "aarch64-darwin"];
        statix.enable = true;
        taplo.enable = true;
        yamlfmt.enable = true;
      };

      settings = {
        excludes = ["node_modules" ".git" "target" "dist"];
        formatters = {};
      };
    }
    {inherit programs settings;};
in {inherit mkTreefmt mkTreefmtConfig;}
