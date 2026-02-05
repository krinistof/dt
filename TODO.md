# Verifiable queue with white list

## Architecture & Security
- [ ] **URL Security:** Use URL fragments (`#key=...`) to pass private keys to frontend without transmitting them to the server.
- [ ] **Trust Model:** First event in a topic (Event #0) defines the immutable whitelist of allowed authors.
    - Payload format: `{"whitelist": ["<pubkey_b58>", ...]}`.

## Shared Tests (`uq/test-data`)
- [ ] **Test Vectors:** Generate JSONs containing:
    - Valid Queue: Event #0 (Whitelist), Events #1..N (Valid signatures from allowed authors).
    - Invalid Queue: Events signed by authors not in Event #0.
    - Invalid Queue: Event #0 malformed or missing (for new topics).

## Backend (`uq/server`)
- [ ] **Validation Logic (Push):**
    - **New Topics:** Reject the first event if its payload is not a valid whitelist configuration.
    - **Existing Topics:** Reject event if the author is not in the topic's whitelist.
- [ ] **Sync Logic (Pull):**
    - Filter response: Only return events from topics where the requesting `user_pk` is in the whitelist.
- [ ] **Tests:** Create `tests/queue_tests.rs` to validate `uq/test-data/` queue vectors.

## Frontend (`dt`)
- [ ] **Identity:**
    - Remove local key generation.
    - Implement `initIdentityFromHash()`: Parse `#s=<user_sk>`, derive public key, clear hash from URL.
- [ ] **Cleanup:** Remove testing-only identity function (`setIdentity`) from public exports.

## UQ Client (`uq-client`)
- [ ] **Refactor:**
    - Remove `generateKeyPair` from public API (move to internal test utils).
    - Add `verifyEvent` and `validateQueue` functions to `index.ts`.
- [ ] **Tests:** Create `index.test.ts` to validate `uq/test-data/` queue vectors.

# Decentralization 

## DAG & Sync
- [ ] **Proto Definition:**
    - Update `uq.proto` Event message:
        - Add `repeated bytes parent_hashes = 6;` (DAG links).
        - Replace existing timestamp with  `int64 logical_timestamp = 7;` (Hybrid Logical Clock).
- [ ] **Backend (SQLite):**
    - Update schema to store `parent_hashes` (likely as JSON or concatenated blob) and `logical_timestamp`.
    - Create sync index: `CREATE INDEX idx_sync ON events (topic_pk, logical_timestamp);`.
- [ ] **Sync Logic:**
    - Implement HLC logic: `Event.timestamp = max(SystemTime, Parent.timestamp + 1)`.
    - Update `SyncRequest` to handle "Graph Sync" (handling gaps/missing parents).
