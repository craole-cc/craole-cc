{lib}: let
  inherit (lib.attrsets) genAttrs isAttrs optionalAttrs;
  inherit (lib.lists) findFirst;
  inherit (lib.packages) getSystemOrDefault defineSystems;
  inherit (lib.strings) isString isPath;
  inherit (lib.trivial) isFunction isNotEmpty isEmpty;

  /**
  Construct a `pkgs` set with the project overlays applied.

  # Type
  ```nix
  mkPkgs :: { inputs :: AttrSet; } -> { system :: string; } -> AttrSet
  ```

  # Examples
  ```nix
  mkPkgs { inherit inputs; } { system = "x86_64-linux"; }
  # => import inputs.NixPackages { ... }
  ```

  # Returns
  A `pkgs` set imported from `inputs.NixPackages` with the project overlays applied.
  */
  mkPkgs = {
    inputs ? null,
    system ? null,
    config ? null,
    extraOverlays ? [],
  }: let
    packages = resolvePackages inputs;
    args =
      {
        config =
          if config != null
          then config
          else {allowUnfree = true;};
        system =
          if isNotEmpty system
          then system
          else getSystemOrDefault {};
      }
      // optionalAttrs (isNotEmpty inputs) {
        overlays = mkOverlays {inherit inputs extraOverlays;};
      };
  in
    if isEmpty inputs
    then import <nixpkgs> args
    else import packages.nix args;

  mkPkgsPerSystem = {inputs, ...}:
    (genAttrs (defineSystems {})) (
      system: mkPkgs {inherit inputs system;}
    );

  parseInput = {
    inputs ? null,
    names,
    error ? "",
  }: let
    inputs' = optionalAttrs (inputs != null) inputs;
    foundName = findFirst (name: inputs' ? ${name}) null names;
    result =
      if foundName != null
      then inputs'.${foundName}
      else null;
  in
    if (isEmpty result) && (isNotEmpty error)
    then throw error
    else result;

  /**
    Resolve specific package set inputs from the flake input attribute set.
    This allows for flexible naming conventions in flake.nix while maintaining
    a consistent internal API.

    # Inputs
    - `inputs`: The attribute set of flake inputs, typically passed from a Nixpkgs overlay or devShell.

    # Type
    ```nix
    resolvePackages :: AttrSet -> AttrSet
    ```

  # Examples
  ```nix
  let
    resolved = resolvePackages inputs;
  in
  resolved.nix # => returns inputs.nixpkgs-unstable or null
  ```

  # Returns
  - An attribute set containing resolved package inputs or null if not found.
  */
  resolvePackages = inputs: {
    ai = parseInput {
      inherit inputs;
      names = [
        "AIAgents"
        "ai-agents"
        "ai-tooling"
        "llm"
        "llm-agents"
        "AI"
        "ai"
      ];
    };

    nix = parseInput {
      inherit inputs;
      names = [
        "NixPackagesUnstable"
        "nixpkgs-unstable"
        "NixPackages"
        "nixpkgs-stable"
        "nixpkgs"
      ];
      error = "mkPkgs: Critical dependency 'nixpkgs' not found in inputs.";
    };

    openclaw = parseInput {
      inherit inputs;
      names = [
        "claw"
        "OpenClaw"
        "openclaw"
      ];
    };

    rust = parseInput {
      inherit inputs;
      names = [
        "oxalica"
        "rust-overlay"
        "Rust"
        "RustOverlay"
      ];
      error = "mkPkgs: Critical dependency 'rust-overlay' not found in inputs.";
    };

    treefmt = parseInput {
      inherit inputs;
      names = [
        "fmTree"
        "Formatter"
        "treefmt-nix"
        "TreeFmt"
        "treefmt"
        "TreeFormatter"
      ];
    };
  };

  /**
  Safely resolves an input into a Nixpkgs overlay.
  Acts as a firewall against broken functors or missing attributes.
  */
  resolveOverlay = input: let
    noop = _: _: {};
  in
    #? 1. Immediate Safety
    if isEmpty input
    then noop
    #? 2. Check for Modern Flake Overlay (overlays.default)
    else if isAttrs input
    then input.overlays.default or noop
    #? 3. Check if the input is already a function
    else if isFunction input
    then input
    #? 4. Fallback for paths (old-school imports)
    else if isPath input || isString input
    then let
      src = import input;
    in
      if isFunction src
      then src
      else noop
    else noop;

  mkOverlays = {
    inputs ? null,
    extraOverlays ? [],
  }: let
    packages = resolvePackages inputs;
  in
    [
      (resolveOverlay packages.ai)
      (resolveOverlay packages.openclaw)
      (resolveOverlay packages.rust)
    ]
    ++ extraOverlays;

  perSystem = {
    fn,
    pkgs ? null,
    inputs ? null,
  }: let
    set =
      if pkgs != null && pkgs?legacyPackages
      then pkgs
      else mkPkgs {inherit inputs;};
  in
    genAttrs (defineSystems {})
    (system: fn set.legacyPackages.${system});
in {inherit mkOverlays mkPkgs mkPkgsPerSystem perSystem;}
