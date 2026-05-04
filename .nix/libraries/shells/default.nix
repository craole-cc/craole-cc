{
  lib,
  paths ? {},
  ...
}: let
  inherit (lib.assembly) assemble;
  inherited = lib.shells or {};
in
  assemble {
    start = inherited;
    entries = ./.;
    scope = acc: lib // {shells = inherited // acc;};
    priority = ["scripts.nix" "tools.nix"];
    extraArgs = {inherit lib paths;};
  }
