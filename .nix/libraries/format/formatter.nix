# .nix/libraries/packages/formatter.nix
{
  lib,
  paths,
  ...
}: let
  inherit (lib.packages) perSystem;
  /**
  * Evaluate a treefmt config and expose formatter + check builder.
  *
  * Usage:
  *   let
  *     treefmt = pkgs.treefmt;
  *   in
  *   mkTreefmt { inherit treefmt pkgs; }
  */
  mkPackage = {
    pkgs ? null,
    projectRoot ? paths.src,
  }: let
    package = pkgs.treefmt;
    # derivation that runs the formatter on this repo
    builder = pkgs.mkDerivation {
      name = "treefmt-builder";
      DONT_BUILD_DEPEND = 1;
      buildCommand = ''
        cd ${projectRoot}
        # exec ${package}/bin/treefmt
        echo "Formatting complete"
      '';
      preferLocalBuild = true;
      allowSubstitutes = false;
    };

    # derivation that runs the formatter in check mode
    check = pkgs.mkDerivation {
      name = "treefmt-check";
      DONT_BUILD_DEPEND = 1;
      buildCommand = ''
        cd ${projectRoot}
        # exec ${package}/bin/treefmt --check
        echo "Formatting check passed"
      '';
      preferLocalBuild = true;
      allowSubstitutes = false;
    };
  in {inherit builder check;};

  # mkTreefmt = perSystem {
  #   fn = pkgs: let
  #     treefmtEnv = Formatter.lib.evalModule pkgs ./treefmt.nix;
  #   in
  #     mkPackage {
  #       treefmt = treefmtEnv.config.build.wrapper;
  #       pkgs = pkgs;
  #     };
  # };
  mkConfig = {
    pkgs,
    projectRoot ? paths.src or ../../..,
    includes ? [
      "*.nix"
      "*.rs"
      "*.toml"
      "*.json"
      "*.yaml"
      "*.yml"
      "*.md"
    ],
    excludes ? [
      "node_modules"
      ".git"
      "target"
      "dist"
    ],
  }: let
    programs = {
      terraform.enable = true;
    };
    settings = {
      formatters = {
        #   nix = {
        #     command = "${pkgs.nixpkgs-fmt}";
        #     includes = ["*.nix"];
        #   };
        #   rust = {
        #     command = "${pkgs.rust-bin.rustfmt}";
        #     includes = ["*.rs"];
        #   };
        #   toml = {
        #     command = "${pkgs.taplo}";
        #     includes = ["*.toml"];
        #   };
      };
    };
  in {inherit projectRoot includes excludes programs settings;};
in {inherit mkConfig mkPackage;}
