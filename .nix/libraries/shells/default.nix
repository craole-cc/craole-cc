# {
#   lib,
#   paths ? {},
#   ...
# }:
# lib.assembly.importLibs {
#   inherit lib;
#   path = ./.;
#   priority = ["scripts.nix" "tools.nix"];
#   extraArgs = {inherit paths;};
# }
{
  lib,
  paths ? {},
  ...
}: let
  inherited = lib.shells or {};
in
  lib.assembly.assemble {
    start = inherited // {inherit paths;};
    entries = ./.;
    scope = acc: lib // {shells = inherited // acc;};
    priority = ["scripts.nix" "tools.nix"];
    extraArgs = {inherit paths;};
  }
