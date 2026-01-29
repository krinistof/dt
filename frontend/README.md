# Democratic Tier Frontend

A Vanilla TypeScript frontend for the Democratic Tier application, using ConnectRPC for backend communication.

## Project Structure

- `src/main.ts`: Application entry point.
- `dist/`: Build output directory.
- `../uq/client`: Local dependency for generated Protobuf clients.

## Technologies

- **Language**: TypeScript
- **Build Tool**: esbuild
- **Environment**: nix flake
- **RPC**: ConnectRPC (Web)
- **Crypto**: @noble/ed25519, @noble/hashes
