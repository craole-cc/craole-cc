{lib, ...}: let
  inherit (lib.attrsets) mapAttrs recursiveUpdate;
  inherit (lib.lists) elem toList;
  inherit (lib.packages) mkBins perSystem;

  mkFormatter = {
    inputs ? null,
    pkgs ? null,
    programs ? {},
    settings ? {},
  }: let
    packages = perSystem {
      inherit inputs pkgs;
      fn = pkgs:
        mkTreefmt {inherit pkgs programs settings;};
    };
  in {
    formatter = mapAttrs (_: v: v.formatter) packages;
    checks = mapAttrs (_: v: {formatting = v.check;}) packages;
  };

  mkTreefmt = {
    pkgs,
    programs,
    projectRoot,
    settings,
  }: let
    package = pkgs.treefmt;
    module = package.lib.evalModule pkgs (
      mkTreefmtConfig {inherit pkgs programs settings;}
    );
    inherit (module.config.build) wrapper;

    bins = mkBins {treefmt = wrapper;};
    command = bins.treefmt;

    common = {
      DONT_BUILD_DEPEND = 1;
      preferLocalBuild = true;
      allowSubstitutes = false;
    };

    builder = pkgs.mkDerivation (common
      // {
        name = "treefmt-builder";
        buildCommand = ''
          cd ${projectRoot}
          exec ${command}
        '';
      });

    check = pkgs.mkDerivation (common
      // {
        name = "treefmt-check";
        buildCommand = ''
          cd ${projectRoot}
          exec ${command} --check
        '';
      });
  in {
    inherit wrapper builder check;
  };

  mkTreefmtConfig = {
    pkgs,
    programs,
    settings,
  }: let
    unless = systems:
      !(elem pkgs.stdenv.hostPlatform.system (toList systems));
  in
    recursiveUpdate {
      projectRootFile = "flake.nix";
      # projectRootFile = ".git/config";

      programs = {
        actionlint.enable = true;
        alejandra.enable = true;
        deno.enable = unless "riscv64-linux";
        leptosfmt.enable = true;
        rustfmt.enable = true;
        shellcheck.enable = true;
        shfmt.enable = true;
        sqruff.enable = unless ["riscv64-linux" "aarch64-darwin"];
        statix.enable = true;
        taplo.enable = true;
        yamlfmt.enable = true;
      };

      settings = {
        excludes = [
          "node_modules"
          ".git"
          "target"
          "dist"
        ];

        formatters = {};
      };
    }
    {inherit programs settings;};
in {inherit mkFormatter mkTreefmtConfig mkTreefmt;}
