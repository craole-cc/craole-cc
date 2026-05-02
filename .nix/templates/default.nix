{lib, ...}: let
  inherit (lib.attrsets) attrNames;
  inherit (lib.lists) head toList;
  inherit (lib.packages) mkPkgs;
  # inherit (lib.shells) mkScriptPackage mkMissionControl;
  inherit (lib.strings) concatStringsSep mkStyledOutput mkSection mkHeader;
  inherit (lib.trivial) readFile;

  scripts = {
    deployTemplates = ./deploy-templates.sh;
    resetFlake = ./reset-flake.sh;
  };

  entries = {
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

  #> Generates one `deploy_entry` call per template entry
  deployTemplate = name: {
    source,
    target,
  }: let
    targets = toList target;
    preferred = head targets;
    list = concatStringsSep " " (map (target: "\"${target}\"") targets);
  in ''deploy_entry "${name}" "${source}" "${preferred}" ${list}'';

  deployCalls =
    concatStringsSep "\n"
    (map (name: deployTemplate name entries.${name}) (attrNames entries));

  mkPackage = {pkgs ? mkPkgs {}}: let
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
  in
    pkgs.writeShellScriptBin "deploy-templates" ''
      ${banner}
      ${section}
      CMD_GUM="${print.gum}"
      ${readFile scripts.deployTemplates}
      ${deployCalls}
    '';
in {
  templates = {
    inherit
      entries
      mkPackage
      # mkFlakeReset
      # mkCommands
      scripts
      ;
  };
}
