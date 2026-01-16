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

## [8.2.1] - Unreleased

### Planned
-   **Backend:**
    -   Profile SQLite service and add indices based on access patterns
-   **Monitoring:**
    -   Set up circular logging
    -   Integrate with Grafana


## [8.2.0] - Unreleased

### Added
-   **Frontend:**
    -   **Automatic player:** A read only client bot for playing the most liked songs after each other with song timeouts (`0fe803c`).
    -   **Search Bar:** Add an "X" to the search bar to allow for easily clearing text (`3d47337`).
    -   **State Sync:** Implement full state sync and improve audio loading (`0824a79`).
-   **Backend:**
    -   **Stability:** Add integration tests that run the backend and frontend tests together
    -   **Media Scanner:** Behind a cargo feature, implemented a music scanner which keeps the posts table up to date with songs from media (`ee2079e`).
-   **Thumbnails:**
    -   **Backend:** The media scanner now identifies embedded cover art and exposes a dedicated `/thumbnail/:filename` endpoint to serve it on-demand (`938c692`).
    -   **Frontend:** Music posts now display the embedded thumbnail. If no thumbnail is present, a unique, dynamically generated SVG placeholder is shown instead (`938c692`).

### Fixed
-   **Frontend:** The log collector now ignores 502 Bad Gateway errors to prevent unnecessary retries when the server is temporarily unavailable (`13d1dfc`).

### Planned
-   **Frontend:**
    -   **Voting UX:**
        -   **Hold-to-Vote:** Hold the left side of a post to downvote or the right side to upvote.
        -   **Dynamic Scoring:** Score accelerates non-linearly(tanh), requiring more effort to cast extreme votes. max or min score in 3 s.
        -   **Visual Feedback:** Post score is visualized as a progress bar between thumbnails and titles, with haptic feedback.
        -   **Interaction Safety:** Content redraws are paused during voting to prevent flickering and accidental taps.
        -   **Mobile Layout:** On mobile devices, post thumbnails are enlarged to 80% of the screen width, with the artist and title displayed below.
    -   **Detail View:** Double tapping on posts opens a detail view, offering song previews (e.g., starting at 30%).
    -   **General:**
        -   Prevented resizing and text selection during mobile interactions.
    -   Search bar on top
    -   Removed text-based score feedback from the user
    -   Ensured broad CSS support for most browsers and mobile devices
-   **Backend:**
    -   Updated post handling to dynamically generate a content hash
    -   Changed default logging level to info
-   **Build System:**
    -   Improved the build.rs to provide detailed `npm` error logs
-   **Posts:** Initial support for music posts defined by JSON, with content accessible via URL

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
    -   Improved compatibility with replacing hashing with package for non-secure contexts
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
