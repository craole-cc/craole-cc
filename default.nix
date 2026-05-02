{
  description ? "Rust development environment with AI Tools",
  lib ? null,
  inputs ? null,
  system ? null,
  config ? {allowUnfree = true;},
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

  inherit (builtins) attrNames filter head isAttrs match pathExists tail;
  findFirst = {
    pred,
    default,
    list,
  }:
    if list == []
    then default
    else if pred (head list)
    then head list
    else
      findFirst {
        inherit pred default;
        list = tail list;
      };

  nixpkgs = let
  isUnstable = n: match ".*(unstable|master).*" n != null;
  name =
    if isAttrs inputs
    then let
      candidates = filter
        (n: n != "self" && inputs.${n} ? legacyPackages)
        (attrNames inputs);
      unstable = filter isUnstable candidates;
    in
      if unstable != [] then head unstable
      else if candidates != [] then head candidates
      else null
    else null;
  args = {inherit config;};
in
  if name != null
  then import inputs.${name} args
  else import <nixpkgs> args;

  # nixpkgs = let
  #   name =
  #     if isAttrs inputs
  #     then
  #       findFirst
  #       (n: inputs.${n} ? legacyPackages)
  #       null
  #       (attrNames inputs)
  #     else null;
  #   args = {inherit config;};
  # in
  #   if name != null
  #   then import inputs.${name} args
  #   else import <nixpkgs> args;

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

  packages = mkPkgs {inherit inputs system;};
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
