{
  lib,
  paths ? {},
  ...
}: let
  inherit (lib.attrsets) attrNames mapAttrsToList;
  inherit (lib.packages) mkPkgs;
  inherit (lib.strings) concatLines escapeShellArg;
  inherit (lib.trivial) readFile;

  scripts =
    (paths.scripts or {})
    // {missionControl = ./mission-control.sh;};

  mkEnvLines = env:
    concatLines (mapAttrsToList (name: value: "${name}=${escapeShellArg value}") env);

  mkPackage = {
    pkgs,
    name,
    file ? null,
    script ? null,
    env ? {},
  }:
    pkgs.writeShellScriptBin name ''
      ${mkEnvLines env}
      ${readFile (
        if script != null
        then scripts.${script}
        else file
      )}
    '';

  mkAlias = {
    pkgs,
    name,
    target,
  }:
    pkgs.writeShellScriptBin name ''
      exec ${target} "$@"
    '';

  mkMissionControl = {
    pkgs,
    shellName,
    commands,
  }: let
    names = attrNames commands;
    commandList = concatLines (
      map (name: "  ${name}  ${commands.${name}.description}") names
    );
    commandCases = concatLines (
      map (name: ''
        ${name})
          shift
          ${commands.${name}.run}
          ;;
      '')
      names
    );
  in
    pkgs.writeShellScriptBin "mission-control" ''
            mission_list() {
              cat <<'EOF'
      Mission Control: ${shellName}

      Available commands:
      ${commandList}
      EOF
            }

            mission_run() {
              cmd="''${1:-}"

              case "$cmd" in
                ${commandCases}
                *)
                  printf 'Unknown command: %s\n' "$cmd" >&2
                  mission_list >&2
                  exit 1
                  ;;
              esac
            }

            ${readFile scripts.missionControl}
    '';

  mkFlakeReset = {pkgs ? mkPkgs {}}:
    mkPackage {
      inherit pkgs;
      name = "reset-flake";
      file = scripts.resetFlake;
    };

  mkCommands = {pkgs ? mkPkgs {}}:
    mkMissionControl {
      inherit pkgs;
      shellName = "rusti";
      commands = {
        deploy-templates = {
          description = "Sync config templates into the project";
          run = "${mkPackage {inherit pkgs;}}/bin/deploy-templates";
        };
        reset-flake = {
          description = "Reset the flake lock and generated files";
          run = "${mkFlakeReset {inherit pkgs;}}/bin/reset-flake";
        };
      };
    };
in {
  inherit
    mkAlias
    mkEnvLines
    mkMissionControl
    mkFlakeReset
    mkPackage
    scripts
    mkCommands
    ;
}
