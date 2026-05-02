{lib, ...}: let
  inherit (lib.attrsets) mapAttrs recursiveUpdate;
  inherit (lib.lists) elem toList;
  inherit (lib.packages) resolvePackages perSystem;

  # mkTreefmt = {
  #   inputs,
  #   programs ? {},
  #   settings ? {},
  # }: let
  #   packages = perSystem {
  #     inherit inputs;
  #     fn = pkgs: (let
  #       inherit (resolvePackages {inherit inputs;}) treefmt;
  #       module = treefmt.lib.evalModule pkgs (
  #         mkTreefmtConfig {inherit pkgs programs settings;}
  #       );
  #     in {inherit (module.config.build) wrapper check;});
  #   };
  #   formatter = mapAttrs (_: v: v.wrapper) packages;
  #   checks = mapAttrs (_: v: {formatting = v.check;}) packages;
  # in {inherit formatter checks;};
  mkTreefmt = {
    flake,
    programs ? {},
    settings ? {},
  }: let
    packages = perSystem {
      inherit (flake) inputs;
      fn = pkgs: (let
        inherit (resolvePackages {inherit (flake) inputs;}) treefmt;
        module = treefmt.lib.evalModule pkgs (
          mkTreefmtConfig {inherit pkgs programs settings;}
        );
      in {
        inherit (module.config.build) wrapper;
        formatting = module.config.build.check flake;
      });
    };
    formatter = mapAttrs (_: v: v.wrapper) packages;
    checks = mapAttrs (_: v: {formatting = v.formatting;}) packages;
  in {inherit formatter checks;};

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
in {inherit mkTreefmt mkTreefmtConfig;}
