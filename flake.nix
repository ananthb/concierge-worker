{
  description = "Cloudflare Worker in Rust for concierge services (forms, calendars, bookings) with HTMX";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          targets = [ "wasm32-unknown-unknown" ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            wrangler
            worker-build
            wasm-pack
            nodejs_22
            nodePackages.npm
          ];
          shellHook = ''
            echo "Concierge Worker Rust dev environment"
            echo "Commands:"
            echo "  wrangler dev    - Start local dev server"
            echo "  wrangler deploy - Deploy to Cloudflare"
          '';
        };
      }
    );
}
