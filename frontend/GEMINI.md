# Project Overview

This is a local-first client for a voting system named "dt". It is written in TypeScript and uses Vite for building. It communicates with a backend using gRPC (protobuf and connectrpc). The application is set up to capture and log errors and warnings, storing them locally in IndexedDB and syncing them with a server.

# Building and Running

*   **Install dependencies:** `npm install`
*   **Run development server:** `npm run dev`
*   **Build for production:** `npm run build` (Note: this is typically handled automatically by the backend's build process)
*   **Preview production build:** `npm run preview`
*   **Generate protobuf code:** `npm run gen-proto`

# Development Conventions

*   The project uses TypeScript with strict linting rules.
*   Modules are resolved using bundler mode, and path aliases are configured for `@/*` to point to the root directory.
*   The application uses a global error handler to catch and log all uncaught exceptions and unhandled promise rejections.
*   Console `warn` and `error` are overridden to send logs to the server.
*   **Extensible Content:** The application handles special content types (like music posts or polls) by parsing JSON embedded in string fields from the backend. If parsing fails or the structure is incorrect, the content is rendered as plain text. See the main `GEMINI.md` for more details on this convention.