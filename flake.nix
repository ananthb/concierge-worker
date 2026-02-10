{
  description = "Cloudflare Worker in Rust for concierge services (forms, calendars, bookings) with HTMX";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, crane }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "clippy" "rustfmt" ];
          targets = [ "wasm32-unknown-unknown" ];
        };
        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        # Common args for all crane builds
        commonArgs = {
          src = craneLib.cleanCargoSource ./.;
          strictDeps = true;
        };

        # Build dependencies separately for caching
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
      in
      {
        checks = {
          # Run cargo test
          tests = craneLib.cargoTest (commonArgs // {
            inherit cargoArtifacts;
          });

          # Run clippy with all warnings as errors
          clippy = craneLib.cargoClippy (commonArgs // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- -D warnings";
          });

          # Check formatting
          fmt = craneLib.cargoFmt {
            src = craneLib.cleanCargoSource ./.;
          };
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            wrangler
            worker-build
            wasm-pack
            nodejs_22
            nodePackages.npm
            mdbook
          ];
          shellHook = ''
            echo "Concierge Worker Rust dev environment"
            echo "Commands:"
            echo "  wrangler dev        - Start local dev server"
            echo "  wrangler deploy     - Deploy to Cloudflare"
            echo "  mdbook serve docs   - Preview documentation"
            echo "  mdbook build docs   - Build documentation"
            echo "  nix flake check     - Run CI checks (tests, clippy, fmt)"
          '';
        };
      }
    );
}
