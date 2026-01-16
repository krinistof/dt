# Project Overview

This is the local-first client for "Democratic Tier" (dt). It is written in TypeScript and uses Vite for building. It communicates with the backend using gRPC (Connect RPC).

The client is responsible for all business logic, cryptography, and state management.

## Architecture

### Key Concepts

*   **Local-First:** Data is stored locally (e.g., in memory or IndexedDB) and synced with the server.
*   **Cryptography:**
    *   **GroupKey:** Used to encrypt/decrypt content. Shared among valid group members.
    *   **UserKey:** User's private identity key. Used to sign events.
*   **Event Loop:**
    1.  **Fetch:** Download encrypted events from the server.
    2.  **Decrypt & Verify:** Decrypt using GroupKey, verify signature matches the sender.
    3.  **Reduce:** Apply events to a local Reducer to derive state (e.g., list of posts, vote tallies).

### State Management

The application state is derived purely from the event log.
`Events -> Decrypt -> Reducer -> Store -> UI`

*   **Posts:** Created by encrypting a JSON payload `{type: "post", txt: "..."}`.
*   **Votes:** Created by encrypting a JSON payload `{type: "vote", ref: "signature_of_post", val: 1}`.

# Development Conventions

*   The project uses TypeScript with strict linting rules.
*   Modules are resolved using bundler mode, and path aliases are configured for `@/*` to point to the root directory.
*   The application uses global error handlers to catch and log all uncaught exceptions and unhandled promise rejections.
