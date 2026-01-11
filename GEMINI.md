# Project Overview

This repository contains the source code for "Democratic Tier" (dt), a local-first, privacy-focused social media and voting system. The project is divided into frontend applications and a backend service, communicating via ConnectRPC.

## Architecture

The system follows a "smart client, dumb server" architecture to ensure privacy and censorship resistance within the group.

*   **Protocol:** Event sourcing with end-to-end encryption.
*   **Clients:** Handle all business logic, decryption, and state aggregation.
*   **Server:** Acts as an agnostic store for encrypted event blobs. It verifies signatures against a whitelist but has no knowledge of the content (posts, votes, etc.).

### Data Flow
1.  **Sync:** Clients pull encrypted events from the server since their last sync.
2.  **Decrypt & Reduce:** Clients decrypt events using a shared GroupKey, verify signatures, and pass them through a local reducer to build the application state.
3.  **Action:** When a user acts, the client creates a JSON payload, encrypts it, signs it, and pushes the blob to the server.
4.  **Storage:** The server verifies the publisher's signature and stores the blob if valid.

