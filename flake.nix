{
  # description = "Rust development environment with AI Tools";

  inputs = {
    NixPackages.url = "github:NixOS/nixpkgs/nixos-unstable";
    AI.url = "github:numtide/llm-agents.nix";
    Rust = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "NixPackages";
    };
    # OpenClaw = {
    #   url = "github:Scout-DJ/openclaw-nix";
    #   inputs.nixpkgs.follows = "NixPackages";
    # };
  };

  outputs = inputs: import ./. {inherit inputs;};
}
