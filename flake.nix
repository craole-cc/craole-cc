{
  description = "Craole.CC";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {inherit system overlays;};

        #╔═══════════════════════════════════════════════════════════╗
        #║ Rust Toolchain                                            ║
        #╚═══════════════════════════════════════════════════════════╝

        #? Prefer rust-toolchain.toml if present, otherwise latest nightly
        rustToolchain =
          if builtins.pathExists ./rust-toolchain.toml
          then pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml
          else
            pkgs.rust-bin.selectLatestNightlyWith (
              toolchain:
                toolchain.default.override {
                  extensions = [
                    "clippy"
                    "rust-analyzer"
                    "rust-src"
                    "rustfmt"
                  ];
                  targets = ["wasm32-unknown-unknown"];
                }
            );

        #╔═══════════════════════════════════════════════════════════╗
        #║ Tool Groups                                               ║
        #╚═══════════════════════════════════════════════════════════╝

        #? Rust / Leptos build chain
        rustTools = with pkgs; [
          rustToolchain
          binaryen
          cargo-bloat
          cargo-edit
          cargo-generate
          cargo-leptos
          cargo-seek
          cargo-sweep
          cargo-watch
          cargo-wipe
          leptosfmt
          trunk
          wasm-bindgen-cli
        ];

        #? Frontend assets
        frontendTools = with pkgs; [
          dart-sass
          deno
          pnpm
          tailwindcss_4
        ];

        #? Formatting and linting — must stay in sync with .treefmt.toml
        formatTools = with pkgs; [
          actionlint
          fd
          shellcheck
          shfmt
          taplo
          treefmt
        ];

        #? General dev utilities
        devTools = with pkgs; [
          lsd
        ];
      in {
        #╔═══════════════════════════════════════════════════════════╗
        #║ Dev Shell                                                 ║
        #╚═══════════════════════════════════════════════════════════╝

        devShells.default = pkgs.mkShell {
          buildInputs = rustTools ++ frontendTools ++ formatTools ++ devTools;
        };

        #╔═══════════════════════════════════════════════════════════╗
        #║ Formatter                                                 ║
        #╚═══════════════════════════════════════════════════════════╝

        #? Exposes treefmt as the nix formatter so `nix fmt` works
        formatter = pkgs.treefmt;

        #╔═══════════════════════════════════════════════════════════╗
        #║ Packages                                                  ║
        #╚═══════════════════════════════════════════════════════════╝

        packages.default = pkgs.stdenv.mkDerivation {
          name = "craole-cc";

          #? `self` rather than `./.` proves the output is fully self-contained
          src = self;

          buildInputs = [rustToolchain pkgs.trunk];

          buildPhase = ''
            export HOME=$TMPDIR
            trunk build --release
          '';

          installPhase = ''
            mkdir -p $out
            cp -r dist/* $out/
          '';
        };
      }
    );
}
