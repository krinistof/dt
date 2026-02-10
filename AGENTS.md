# Developer Guide for AI Agents

This repository contains a full-stack application with a Rust backend (Axum) and a Vanilla TypeScript frontend.

## Critical Instructions for Agents

- **Environment**: This project uses **Nix Flakes** to manage development dependencies.

## Project Structure
- `backend/`: Rust server application.
- `frontend/`: TypeScript frontend application.
- `uq/`: Submodule of UniQue dependency. 
- `flake.nix`: Nix environment definition.

## Development Environment (Nix)
Ensure you are running inside the Nix shell. Check `flake.nix` for supported tools.

## Backend (Rust)

### Commands
Run these commands from the `backend/` directory. All standard rust tools are present.

Here are the key commands:
- **Build**: `cargo build` (This builds every dependency as well)
- **Run**: Never call run, it's the developer's responsibility.

### Code Style & Conventions
There are linters and other validator tools integrated. Those are automatically ran by both git hooks and CI! 

## Frontend (TypeScript)

### Commands
Run these commands from the `frontend/` directory.

- **Install**: `npm install`
- **Build**: `npm run build` (uses `esbuild`, outputs to `dist/`)
- **Lint/Format**: `npm run lint` (uses `biome check --write .`)
- **Test**: `npm test`

### Code Style & Conventions
- **Framework**: Vanilla TypeScript (no frameworks).
- **Imports**: Use `.js` extension for local imports (ESM requirement).
  - Example: `import { initializePlatform } from "./platform.js";`
- **Platform**: `src/platform.ts` handles platform-specific initialization.
- **DOM**: Direct DOM manipulation (e.g., `document.getElementById`).

## Testing Strategy
- **Integration Tests**: `backend/tests/integration_test.rs` spins up the backend and tries to run frontend tests.
- **Unit Tests**: Place Rust unit tests in the same file within a `mod tests` block or in `backend/tests/` for integration.
- **Running Integration**: `cargo test --test integration_test` (Requires `frontend` environment setup if it runs `npm`).
