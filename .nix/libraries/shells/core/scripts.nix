{
  lib,
  paths ? {},
  ...
}: let
  inherit (lib.attrsets) attrNames mapAttrsToList;
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
    file,
    env ? {},
  }:
    pkgs.writeShellScriptBin name ''
      ${mkEnvLines env}
      ${readFile file}
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
in {
  inherit
    mkAlias
    mkEnvLines
    mkMissionControl
    mkPackage
    scripts
    ;
}
