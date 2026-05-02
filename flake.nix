{
  description = "Rust development environment with AI Tools";

  inputs = {
    NixPackages.url = "github:NixOS/nixpkgs/nixos-unstable";
    AI.url = "github:numtide/llm-agents.nix";
    Rust = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "NixPackages";
    };
    OpenClaw = {
      url = "github:Scout-DJ/openclaw-nix";
      inputs.nixpkgs.follows = "NixPackages";
    };
    Formatter = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "NixPackages";
    };
  };

  outputs = inputs @ {self, ...}: let
    flake = self;
    src = import ./. {
      inherit inputs;
      inherit (inputs.NixPackages) lib;
    };
    inherit (src) lib pkgs;
    inherit (lib.shells) mkDevShells;
    inherit (lib.packages) mkPkgsPerSystem mkTreefmt;
  in
    {
      inherit lib;
      legacyPackages = mkPkgsPerSystem {inherit inputs;};
    }
    // mkTreefmt {inherit inputs flake;}
    // mkDevShells {inherit inputs pkgs;};
}
