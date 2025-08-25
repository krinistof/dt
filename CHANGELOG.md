# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [8.1.0] - Unreleased

### Planned

-   **Voting System:**
    -   Follow live polls
-   **Syncing:**
    -   Implement initial state sync for posts.
    -   Implement delta sync for new events.
-   **Backend:**
    -   Add `votes` table to the database.
    -   Add tests

### Finished

-   **Backend:**
    -   Implement event submission service
-   **Frontend:**
    -   Update frontend to use the new event submission service.
-   **Social Media Features:**
    -   Posting
-   **Voting System:**
    -   Voting on polls
-   **Frontend:**
    -   Implement both UI state and event queue in IndexedDB
-   **Backend:** Singe server process for serving static assets and gRPC.
-   **Syncing:** Events are synced from client to server.

## [8.0.1] - 2025-08-25

### Finished

-   **Frontend:**
    -   TypeScript application with Vite.
    -   gRPC communication with the backend using Connect RPC.
    -   Capture and log errors and warnings to IndexedDB.
    -   Sync logs with the server.
    -   Global error handler to catch all uncaught exceptions and unhandled promise rejections.
    -   Override console `warn` and `error` to send logs to the server.
-   **Backend:**
    -   Rust application with tonic.
    -   gRPC service for collecting logs.
    -   CORS configured to allow requests from any origin.
-   **Protobuf:**
    -   gRPC services are defined in the `.proto` files.
    -   `buf` tool is used to generate the necessary code for both the frontend and the backend.

## [7.0.1] - 2025-06-30

### Removed

-   Old implementation (v7.0) has been removed.