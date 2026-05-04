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
    modules = mkCfg "modules";
    templates = mkCfg "templates";
    scripts = {
      fmtRust = mkScr + "fmt-rust.sh";
      getIcons = mkScr + "get-icons.sh";
      initDatabase = mkScr + "init-db.sh";
      deployTemplates = mkTScr "deploy-templates.sh";
      resetFlake = mkTScr "reset-flake.sh";
      update = mkScr "update.sh";
    };
  };

  libraries = import paths.libraries {inherit paths lib;};
  packages = libraries.packages.mkPkgs {inherit inputs system;};
in {
  inherit description paths;
  pkgs = packages;
  lib = libraries;
}
