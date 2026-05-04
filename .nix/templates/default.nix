{lib, ...}: let
  inherit (lib.attrsets) attrNames;
  inherit (lib.lists) head toList;
  inherit (lib.packages) mkPkgs;
  inherit (lib.shells) mkPackage;
  inherit (lib.strings) concatStringsSep escapeShellArg mkHeader mkSection mkStyledOutput replaceStrings;
  inherit (lib.trivial) readFile;

  scripts = {
    deployTemplates = ./deploy-templates.sh;
    resetFlake = ./reset-flake.sh;
  };

  entries = {
    template-state = {
      source = ./template-state;
      target = ".nix-template-state";
    };
    cargo = {
      source = ./cargo.toml;
      target = ".cargo/config.toml";
    };

    envrc = {
      source = ./envrc;
      target = ".envrc";
    };

    gitignore = {
      source = ./gitignore;
      target = ".gitignore";
    };

    markdownlint = {
      source = ./markdownlint-cli2.yaml;
      target = [".markdownlint-cli2.yaml" "markdownlint-cli2.yaml"];
    };

    rust-analyzer = {
      source = ./rust-analyzer.toml;
      target = "rust-analyzer.toml";
    };

    rust-toolchain = {
      source = ./rust-toolchain.toml;
      target = "rust-toolchain.toml";
    };

    rustfmt = {
      source = ./rustfmt.toml;
      target = "rustfmt.toml";
    };

    shellcheck = {
      source = ./shellcheckrc;
      target = [".shellcheckrc" "shellcheckrc"];
    };

    treefmt = {
      source = ./treefmt.toml;
      target = [".treefmt.toml" "treefmt.toml"];
    };
  };

  deployTemplate = name: {
    source,
    target,
  }: let
    targets = toList target;
    preferred = head targets;

    quotedTargets =
      concatStringsSep " "
      (map escapeShellArg targets);
  in ''
    deploy_entry ${escapeShellArg name} ${escapeShellArg (toString source)} ${escapeShellArg preferred} ${quotedTargets}
  '';

  deployCalls =
    concatStringsSep "\n"
    (map (name: deployTemplate name entries.${name}) (attrNames entries));

  mkDeployTemplates = {pkgs ? mkPkgs {}}: let
    print = mkStyledOutput {inherit pkgs;};

    banner = mkHeader {
      inherit print;
      title = "Template Deployment";
      content = "Syncing project configuration files into your workspace";
    };

    section = mkSection {
      inherit print;
      title = "Entries";
      content = map (name: name) (attrNames entries);
    };

    deployBody = ''
      ${banner}
      ${section}
      ${deployCalls}
    '';

    source =
      replaceStrings
      ["#__DEPLOY_TEMPLATE_CALLS__"]
      [deployBody]
      (readFile scripts.deployTemplates);
  in
    mkPackage {
      inherit pkgs;
      name = "deploy-templates";
      file = null;
      text = source;
    };

  mkResetFlake = {pkgs ? mkPkgs {}}:
    mkPackage {
      inherit pkgs;
      name = "reset-flake";
      file = scripts.resetFlake;
    };
in {
  templates = {inherit entries scripts mkDeployTemplates mkResetFlake;};
}
