{
  description = "UniQue (uq) and Democratic Tier (dt) Monorepo";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };

      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Rust
            rustToolchain
            pkg-config
            openssl

            # Node
            nodejs_22
            
            # Proto
            buf
            protobuf # protoc

            # Tools
            sqlite
          ];

          shellHook = ''
            export PATH=$PWD/node_modules/.bin:$PWD/uq/client/node_modules/.bin:$PATH
          '';
        };
      }
    );
}
