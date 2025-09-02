# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [8.3.0] - Unreleased

### Planned
-   **CI/CD:** Create static built binary releases for various platforms using GitHub Actions
-   **Build System:** Implement a build system using Nix
-   **Deployment:** Create Docker containers from the Nix build
-   **Architecture:** Abstract database, hosting, and content delivery for future flexibility (e.g., PostgreSQL, cloud functions)
-   **Backend:** Implement a dynamic media scanner that detects and processes music from newly mounted drives.

## [8.2.1] - Unreleased

### Planned
-   **Frontend:**
    -   Implement a search feedback mechanism. When a search is active, a button will appear offering assistance. If clicked, it will log the user's search query to the server for analysis and display a helpful message to the user.
-   **Backend:**
    -   Profile SQLite service and add indices based on access patterns
    -   Add tests
-   **Monitoring:**
    -   Set up circular logging
    -   Integrate with Grafana


## [8.2.0] - Work in Progress

### Planned
-   **Backend:** 
    -   Change default logging level to info
    -   Behind cargo feature implement music scanner which keeps posts table up to date with songs from media
-   **Frontend:** 
    -   Apply UX experiences from previuos versions:
        -   Disable content redrawing when user interacts with cooldown, add some indicator of new content instead
        -   Prevent resizing, text selection when interacting from mobile
        -   Low code size by minifying for deployed CSS, JS
        -   Add X to search bar to clear text
        -   Change scoring from slider to something with effort-to-reach extremes
    -   Song posts:
        -   Handle song previews. If a new preview starts, stop the previous

### Added
-   **Posts:** Initial support for music posts defined by JSON, with content accessible via URL
-   **Frontend:** 
    -   Removed posting from the UI.
    -   Added a simple design with a search bar on top
    -   Hid text-based score feedback from the user
    -   Ensured broad CSS support for most browsers and mobile devices
-   **Backend:**
    -   Updated post handling to dynamically generate a content hash
-   **Build System:**
    -   Improved the build.rs to provide detailed `npm` error logs


## [8.1.0] - 2025-08-28

### Added

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

### Added

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
