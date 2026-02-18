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

        # Prefer rust-toolchain.toml if present, otherwise latest nightly
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
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            actionlint
            binaryen
            cargo-bloat
            cargo-edit
            cargo-generate
            cargo-leptos
            cargo-seek
            cargo-sweep
            cargo-watch
            cargo-wipe
            dart-sass
            deno
            tailwindcss_4
            leptosfmt
            pnpm
            rustToolchain
            taplo
            treefmt
            trunk
            wasm-bindgen-cli
          ];
        };

        # Use `self` to expose the built site as the default package
        packages.default = pkgs.stdenv.mkDerivation {
          name = "portfolio";
          src = self; # or `./.`; using `self` proves the output is self-contained

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
