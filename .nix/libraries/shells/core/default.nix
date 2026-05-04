{
  lib,
  paths ? {},
  ...
}: let
  inherited = lib.shells or {};
in
  lib.assembly.assemble {
    start = inherited;
    entries = lib.assembly.collectPaths {path = ./.;};
    scope = acc: lib // {shells = inherited // acc;};
    priority = ["scripts.nix"];
    ignore = ["default.nix"];
    extraArgs = {inherit lib paths;};
  }
