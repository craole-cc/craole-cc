{
  description ? "Rust development environment with AI Tools",
  lib ? null,
  inputs ? null,
  system ? null,
  config ? null,
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
    };
  };

  inherit (builtins) hasAttr head isAttrs pathExists tail;
  findFirst = names: set:
    if names == []
    then null
    else let
      name = head names;
    in
      if hasAttr name set
      then name
      else findFirst (tail names) set;

  nixpkgs = let
    name =
      if isAttrs inputs
      then
        findFirst [
          "NixPkgsUnstable"
          "NixPackagesUnstable"
          "nixpkgs-unstable"
          "NixPackages"
          "NixPkgs"
          "nixpkgs-stable"
          "nixpkgs"
        ]
        inputs
      else null;
    cfg = {
      config =
        if config != null
        then config
        else {allowUnfree = true;};
      system =
        if system != null
        then system
        else builtins.currentSystem or "x86_64-linux";
    };
  in
    if name != null
    then import inputs.${name} cfg
    else import <nixpkgs> cfg;

  libraries = let
    lib' =
      if isAttrs lib
      then lib
      else nixpkgs.lib;
  in
    if paths ? libraries && pathExists paths.libraries
    then
      import paths.libraries {
        inherit paths;
        lib = lib';
      }
    else lib';
  inherit (libraries.attrsets) optionalAttrs;
  inherit (libraries.packages) mkPkgs getSystemOrDefault;
  inherit (libraries.shells) mkDevShells;

  packages = mkPkgs {inherit inputs;};
  pkgs = packages;

  environment =
    optionalAttrs (inputs != null)
    (mkDevShells {inherit inputs pkgs;});

  templates =
    libraries.optionalAttrs
    (paths ? templates && pathExists paths.templates)
    import
    paths.templates {
      inherit pkgs;
      lib = libraries;
    };
in
  {
    inherit description templates paths pkgs;
    lib = libraries;
    system = getSystemOrDefault {inherit pkgs;};
  }
  // environment
