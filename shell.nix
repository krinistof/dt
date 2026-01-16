{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    # Rust
    rustup
    rustfmt
    clippy
    openssl
    pkg-config

    # Javascript
    nodejs_24
    esbuild
    biome

    # Protobuf
    buf
    protoc-gen-es

    # Gemini
    gemini-cli
  ];
}
