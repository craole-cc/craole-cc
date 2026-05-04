{lib, ...}:
lib.assembly.importLibs {
  inherit lib;
  path = ./.;
  priority = ["scripts.nix" "tools.nix"];
}
