{
  description ? "Rust development environment with AI Tools",
  lib ? null,
  inputs ? null,
  system ? null,
}: let
  paths = let
    src = ./.;
    scr = src + "/scripts";
    mkCfg = path: src + "/.nix" + "/${path}";
    mkScr = path: scr + "/${path}";

    templates = mkCfg "templates";
    mkTScr = path: templates + "/${path}";
  in {
    inherit src;
    config = mkCfg "config";
    downloads = mkCfg "downloads";
    environment = mkCfg "environment";
    libraries = mkCfg "libraries";
    repl = mkCfg "repl.nix";
    modules = mkCfg "modules";
    inherit templates;
    scripts = {
      src = scr;
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
      if lib != null
      then lib
      else if inputs != null && inputs ? NixPackages
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
