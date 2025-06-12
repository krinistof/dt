{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    # Rust toolchain
    npm
    rustc
    cargo
    rustfmt
    clippy

    # Protobuf tools
    #protobuf
    #protoc-gen-js
    #protoc-gen-grpc-web

    # Frontend (Bun)
    bun       # Bun JS runtime and package manager

    # Common build dependencies for Rust crates (like openssl-sys, etc.)
    openssl
    pkg-config
    zlib # Example, add others if needed by specific Rust crates
  ];
}
