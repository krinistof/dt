{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    # Rust
    rustc
    cargo
    rustfmt
    clippy
    openssl
    pkg-config

    # Javascript
    nodejs_24
    biome

    # Protobuf
    buf

    # Gemini
    gemini-cli
  ];
}
