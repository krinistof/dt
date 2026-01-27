# Plan: Project "UniQue" (uq) Refactor

## 1. Project Restructure (Monorepo)
- [ ] **Workspace Setup**
    - Create a Cargo workspace root (`Cargo.toml`).
    - Define workspace members: `crates/uq`, `crates/uq-server`.
- [ ] **Directory Layout**
    - `crates/uq`: Core Rust library (logic, storage, crypto).
    - `crates/uq-server`: Standalone server binary.
    - `packages/uq-client`: TypeScript client library.
    - `apps/dt-frontend`: The Democratic Tier frontend (consumer of `uq`).
    - `proto/uq`: Renamed and updated protocol definitions.

## 2. Core Library (`crates/uq`)
- [ ] **Move & Refactor Logic**
    - Port `db.rs` from legacy backend.
    - Implement `Sync` logic (agnostic event synchronization).
- [ ] **Cryptography Update (Asymmetric Topics)**
    - Update data model: `Event` struct to include `topic_pk`.
    - Update verification: Signature must cover `hash(blob) + topic_pk`.
    - Drop the old "GroupKey" symmetric encryption model in favor of topic-based keys (or keep encryption as a higher-layer concern, but `uq` enforces the signature structure).

## 3. Server (`crates/uq-server`)
- [ ] **Implementation**
    - ConnectRPC / Axum setup.
    - Expose `SyncService`.
    - **New Feature**: Integrate `collect_log` functionality directly (generic client telemetry).
    - Configurable storage path (SQLite).

## 4. Client Library (`packages/uq-client`)
- [ ] **Extraction**
    - Extract ConnectRPC client generation.
    - Extract Crypto logic (Key generation, Signing, Encryption).
    - Extract "Reducer" pattern / State management helper.
- [ ] **Features**
    - Support the new `topic_pk` scheme.
    - Built-in error reporting (logging) to the server.

## 5. Build System (Nix Flakes)
- [ ] **Flake Definition**
    - `packages.uq-server`: Rust binary.
    - `packages.uq-client`: NPM package (or just TS sources).
    - `packages.dt-frontend`: Web app build.
    - `devShells.default`: Unified dev environment (Rust + Node + Buf).
- [ ] **CI**
    - Update GitHub Workflows to use `nix flake check`.

## 6. Democratic Tier (Migration)
- [ ] **Frontend Update**
    - Update `dt-frontend` to use `uq-client`.
    - Update reducers to work with the new `topic` model.
- [ ] **Cleanup**
    - Remove legacy `backend/` directory.
    - Remove `shell.nix`.
