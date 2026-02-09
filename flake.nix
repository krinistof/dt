{
  description = "democratic tier (dt) repo";

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

        # Common build inputs
        commonInputs = with pkgs; [
          # Proto
          buf
          protobuf
          nix

          # Tools
          sqlite
        ];

        rustInputs = with pkgs; [
          rustToolchain
          pkg-config
          openssl
        ] ++ commonInputs;

        nodeInputs = with pkgs; [
          nodejs_22
          typescript
          esbuild
          biome
        ] ++ commonInputs;

        # The unified check script
        checkScript = pkgs.writeShellApplication {
          name = "check";
          runtimeInputs = rustInputs ++ nodeInputs;
          text = ''
            # Colors
            GREEN='\033[0;32m'
            BLUE='\033[0;34m'
            NC='\033[0m' # No Color

            MODE="all"
            if [ "$#" -gt 0 ]; then
                MODE="$1"
            fi
            
            echo -e "''${BLUE}[Check] Running in mode: $MODE''${NC}"

            # Ensure UQ is built for full check/build
            if [ "$MODE" = "all" ]; then
                 echo -e "''${BLUE}[UQ] Building dependency...''${NC}"
                 (cd uq && nix run .#build)
            fi

            # --- Frontend Checks ---
            if [ "$MODE" = "all" ] || [ "$MODE" = "lint" ]; then
                echo -e "''${BLUE}[Frontend] Linting...''${NC}"
                (cd frontend && npm install && npm run lint)
            fi

            if [ "$MODE" = "all" ]; then
                 echo -e "''${BLUE}[Frontend] Building...''${NC}"
                 (cd frontend && npm run build)
                 
                 echo -e "''${BLUE}[Frontend] Testing...''${NC}"
                 (cd frontend && npm test)
            fi

            # --- Backend Checks ---
            if [ "$MODE" = "all" ] || [ "$MODE" = "lint" ]; then
                 echo -e "''${BLUE}[Backend] Formatting & Linting...''${NC}"
                 (cd backend && cargo fmt --all -- --check)
                 (cd backend && cargo clippy -- -D warnings)
            fi

            if [ "$MODE" = "all" ]; then
                 echo -e "''${BLUE}[Backend] Building...''${NC}"
                 (cd backend && cargo build)
                 
                 echo -e "''${BLUE}[Backend] Testing...''${NC}"
                 (cd backend && cargo test)
            fi
            
            echo -e "''${GREEN}All checks passed!''${NC}"
          '';
        };

        buildScript = pkgs.writeShellApplication {
          name = "build";
          runtimeInputs = rustInputs ++ nodeInputs;
          text = ''
            BLUE='\033[0;34m'
            NC='\033[0m'
            
            echo -e "''${BLUE}[UQ] Building dependency...''${NC}"
            (cd uq && nix run .#build)

            echo -e "''${BLUE}[Frontend] Building...''${NC}"
            (cd frontend && npm install && npm run build)
            
            echo -e "''${BLUE}[Backend] Building...''${NC}"
            (cd backend && cargo build)
          '';
        };

      in
      {
        apps = {
          default = flake-utils.lib.mkApp { drv = checkScript; };
          check = flake-utils.lib.mkApp { drv = checkScript; };
          build = flake-utils.lib.mkApp { drv = buildScript; };
        };

        devShells = {
          backend = pkgs.mkShell {
            buildInputs = rustInputs;
          };

          frontend = pkgs.mkShell {
            buildInputs = nodeInputs;
            shellHook = ''
              export PATH=$PWD/node_modules/.bin:$PWD/uq/client/node_modules/.bin:$PATH
            '';
          };

          default = pkgs.mkShell {
            buildInputs = rustInputs ++ nodeInputs ++ [ checkScript ];
            shellHook = ''
              export PATH=$PWD/node_modules/.bin:$PWD/uq/client/node_modules/.bin:$PATH
            '';
          };
        };
      }
    );
}
