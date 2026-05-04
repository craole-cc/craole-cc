{
  description ? "Rust development environment with AI Tools",
  lib ? null,
  inputs ? null,
  system ? null,
}: let
  paths = let
    src = ./.;
    mkCfg = path: src + "/.nix" + "/${path}";
    mkScr = path: src + "/scripts" + "/${path}";
    mkTScr = path: paths.templates + "/${path}";
  in {
    config = mkCfg "config";
    downloads = mkCfg "downloads";
    environment = mkCfg "environment";
    libraries = mkCfg "libraries";
    repl = mkCfg "repl.nix";
    modules = mkCfg "modules";
    templates = mkCfg "templates";
    scripts = {
      deployTemplates = mkTScr "deploy-templates.sh";
      fmtRust = mkScr "fmt-rust.sh";
      gcp = mkScr "gcp.sh";
      getIcons = mkScr "get-icons.sh";
      initDatabase = mkScr "init-db.sh";
      resetFlake = mkTScr "reset-flake.sh";
      update = mkScr "update.sh";
    };
  };

  libraries = import paths.libraries {
    inherit paths;
    lib =
      if inputs != null && inputs ? NixPackages
      then inputs.NixPackages.lib
      else (import <nixpkgs> {}).lib;
  };
  packages = libraries.packages.mkPkgs {inherit inputs system;};
  repl = import paths.repl {
    inherit libraries packages;
  };
in {
  inherit description paths repl;
  pkgs = packages;
  lib = libraries;
}
