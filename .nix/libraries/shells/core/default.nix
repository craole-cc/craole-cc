{lib}:
lib.assembly.importAttrs {
  inherit lib;
  path = ./.;
  priority = ["scripts.nix"];
}
