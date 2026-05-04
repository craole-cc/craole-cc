{lib, paths ? {}, ...}:
lib.assembly.importAttrs {
  inherit lib;
  path = ./.;
  priority = ["scripts.nix"];
  args = {inherit paths;};
}
