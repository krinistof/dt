# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned

## [8.3.0] - Unreleased

### Planned
-   **CI/CD:** Create static built binary releases for various platforms using GitHub Actions
-   **Build System:** Implement a build system using Nix
-   **Deployment:** Create Docker containers from the Nix build
-   **Architecture:** Abstract database, hosting, and content delivery for future flexibility (e.g., PostgreSQL, cloud functions)

## [8.2.1] - Unreleased

### Planned
-   **Backend:**
    -   Profile SQLite service and add indices based on access patterns
    -   Add tests
-   **Monitoring:**
    -   Set up circular logging
    -   Integrate with Grafana


## [8.2.0] - Work in progress

### Planned
-   **Posts:** Support multi-media posts defined by JSON, content accessible via URL
-   **Frontend:** Apply UX experiences from previuos versions:
    -   Simple to use design with search bar on the top
    -   Broad support of deployed CSS for most browsers, mobiles 
    -   Low code size for deployed CSS, JS
    -   Disable content redrawing when user interacts with cooldown
    -   Prevent resizing, text selection when interacting from mobile

## [8.1.0] - 2025-08-28

### Finished

-   **Network Syncing:**
    -   Implemented differential sync for new events
    -   Implemented initial state sync for posts
-   **Backend:**
    -   Added `votes` table to the database
    -   Updated to monolith server process for serving static assets and gRPC in parallel
-   **Frontend:**
    -   Addressed lint findings for browser compatibility and best practices
    -   Improved compatibility with replacting hashing with package for non-secure contexts
    -   Implemented both UI state and event queue in IndexedDB
-   **Platform:**
    -   Added text posting
    -   Implemented voting with anonymized public results

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
