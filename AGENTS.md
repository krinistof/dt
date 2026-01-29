# Developer Guide for AI Agents

This repository contains a full-stack application with a Rust backend (Axum) and a TypeScript frontend.

## Critical Instructions for Agents

- **Environment**: This project uses **Nix** to manage development dependencies. Assume tools like `cargo`, `node`, `npm`, `buf`, and `esbuild` are available. Do not try to install system packages manually.

## Project Structure
- `backend/`: Rust server application.
- `frontend/`: TypeScript frontend application (Vanilla + ConnectRPC).
- `uq/proto/`: Protocol Buffer definitions.
- `uq/proto/buf.yaml`: Buf configuration for Protobuf.
- `flake.nix`: Nix environment definition.

## Development Environment (Nix)
Ensure you are running inside the Nix shell. The environment provides:
- **Rust**: `rustup` (latest stable), `rustfmt`, `clippy`.
- **Node.js**: Version 24, including `esbuild` and `biome`.
- **Protobuf**: `buf` CLI and plugins.
- **Tools**: `pkg-config`, `openssl`.

## Backend (Rust)

### Commands
Run these commands from the `backend/` directory or use `workdir="backend/"`.

- **Build**: `cargo build`
- **Test**: `cargo test`
- **Run Single Test**: `cargo test <test_name>` (e.g., `cargo test run_frontend_integration_tests`)
- **Lint**: `cargo clippy`
- **Format**: `cargo fmt`
- **Run**: never run the project, it's the developer's responsibility.
  - Default DB: `sqlite://db/dt.db?mode=rwc`.
  - Environment overrides: `DATABASE_URL`

### Code Style & Conventions
- **Formatting**: Strictly follow `cargo fmt`.
- **Error Handling**: 
  - Use `anyhow::Result` for application-level errors (e.g., in `main.rs`, handlers).
- **Logging**: Use `tracing` crate (`info!`, `warn!`, `error!`). Do not use `println!`.
- **Database**: 
  - Uses `sqlx` with SQLite.
  - Migrations are applied automatically on startup via `sqlx::migrate!().run()`.

## Frontend (TypeScript)

### Commands
Run these commands from the `frontend/` directory or use `workdir="frontend/"`.

- **Install**: `npm install`
- **Build**: `npm run build` (uses `esbuild`, outputs to `dist/`)
- **Generate Proto Clients**: `npm run generate` in `uq/client`
- **Lint/Format**: `npm run lint` (uses `biome check --write .`)
- **Test**: `npm test`

### Code Style & Conventions
- **Framework**: Vanilla TypeScript (no React/Vue/Angular).
- **Imports**: Use `.js` extension for local imports (ESM requirement).
  - Example: `import { initializePlatform } from "./platform.js";`
- **Platform**: `src/platform.ts` handles platform-specific initialization.
- **DOM**: Direct DOM manipulation (e.g., `document.getElementById`).

## Protocol Buffers
- Definitions are in `uq/proto/`.
- `buf` is used for code generation.
- To update protos:
  1. Modify files in `uq/proto/`.
  2. Run `npm run generate` in `uq/client`.
  3. Rebuild backend (build script handles proto compilation).

## Testing Strategy
- **Integration Tests**: `backend/tests/integration_test.rs` spins up the backend and tries to run frontend tests.
- **Unit Tests**: Place Rust unit tests in the same file within a `mod tests` block or in `backend/tests/` for integration.
- **Running Integration**: `cargo test --test integration_test` (Requires `frontend` environment setup if it runs `npm`).
