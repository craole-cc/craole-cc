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
}:
lib.assembly.importLibs {
  inherit lib;
  path = ./.;
  priority = ["scripts.nix" "tools.nix"];
  args = {inherit paths;};
}
