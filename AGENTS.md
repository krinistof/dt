# Developer Guide for AI Agents

This repository contains a full-stack application with a Rust backend (Axum) and a Vanilla TypeScript frontend.

## Critical Instructions for Agents

- **Environment**: This project uses **Nix Flakes** to manage development dependencies.

## Project Structure
- `backend/`: Rust server application.
- `frontend/`: TypeScript frontend application.
- `uq/`: **(Submodule)** The UniQue identity and protocol library.
  - *Note: This is now a separate repository. See `uq/README.md` for details.*
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
There are linters and other validator tools interaded. Those are automatically ran by both CI and git hooks. 

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

## UniQue (uq) SDK
The `uq` library is a separate dependency. It handles identity, sync, and protocol definitions.
If you need to modify `uq`, please refer to its own repository/documentation.

- **Protocols**: Defined in `uq/proto`.
- **Clients**: Generated code is consumed by `frontend` and `backend`.

### Updating Protocols
Since `uq` is separated:
1.  Make changes in the `uq` repository (or submodule).
2.  Re-generate/Publish the `uq` libraries.
3.  Update dependencies in `backend/` (`cargo update`) and `frontend/` (`npm update`).


## Testing Strategy
- **Integration Tests**: `backend/tests/integration_test.rs` spins up the backend and tries to run frontend tests.
- **Unit Tests**: Place Rust unit tests in the same file within a `mod tests` block or in `backend/tests/` for integration.
- **Running Integration**: `cargo test --test integration_test` (Requires `frontend` environment setup if it runs `npm`).
