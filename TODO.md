# Plan: Project "UniQue" (uq) Extraction

## 1. `uq` Engine (Future Separate Repo)
The core sync engine and protocol, to be extracted into `uq/`.

### 1.1 Protocol (`uq/proto`)
- [ ] **Definitions**
    - Define generic `Event` with `topic_pk` (32 bytes), `pub_key` (32 bytes), `signature` (64 bytes), `payload` (bytes).
    - Define `SyncService` (Push/Pull).

### 1.2 Server Library (`uq/server`)
- [ ] **Core Logic**
    - Port `db.rs` logic (SQLite).
    - **Enforce Security**: Validate Ed25519 signatures on *all* writes.
    - Implement `Sync` logic (agnostic event synchronization).
- [ ] **API**
    - Expose `UqServer` struct/builder for easy embedding in `dt`.

### 1.3 Client Library (`uq/client`)
- [ ] **Core Logic**
    - Key Pair generation (Ed25519).
    - Event Signing.
    - ConnectRPC client wrapper.
    - "Log" feature (telemetry).

## 2. Democratic Tier (`dt`) Refactor
The specific application consuming `uq`.

### 2.1 Backend (`dt/backend`)
- [ ] **Integration**
    - Add `uq` as a local path dependency (temporary until published).
    - Replace internal DB logic with `uq::Server`.
    - Configure `uq` to store data in `db/dt.db`.

### 2.2 Frontend (`dt/frontend`)
- [ ] **Refactor**
    - Use `uq-client` for communication.
    - **Minimal UI**:
        - "Generate Identity" button (store in localStorage).
        - "Join Topic" input.
        - "Post Message" input.
        - "Stream" view (simple list).

## 3. Cleanup
- [ ] Remove legacy `dt/backend/src/db.rs`.
- [ ] Remove legacy proto files if fully replaced by `uq`.
