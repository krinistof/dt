# Project Overview

This repository contains the source code for "Democratic Tier" (dt), a local-first, privacy-focused social media and voting system. The project is divided into a frontend application and a backend service, communicating via gRPC.

## Frontend

The frontend is a TypeScript application built with Vite. It uses the Connect RPC library to communicate with the backend. For more details, see `frontend/GEMINI.md`.

### Building and Running the Frontend

*   **Install dependencies:** `npm install`
*   **Run development server:** `npm run dev`
*   **Build for production:** `npm run build`
*   **Preview production build:** `npm run preview`
*   **Generate protobuf code:** `npm run gen-proto`

## Backend

The backend is a Rust application that uses the `tonic` library to provide a gRPC service. For more details, see `backend/GEMINI.md`.

### Building and Running the Backend

*   **Prerequisites:**
    *   Rust: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
    *   Protobuf Compiler: [https://grpc.io/docs/protoc-installation/](https://grpc.io/docs/protoc-installation/)
*   **Build:** `cargo build` (this also builds the frontend)
*   **Run:** `cargo run`

## Development Conventions

### Protobuf

The gRPC services are defined in the `.proto` files in the `proto` directory. The `buf` tool is used to generate the necessary code for both the frontend and the backend.

*   **Frontend:** Run `npm run gen-proto` to generate the TypeScript code.
*   **Backend:** The Rust code is generated automatically by the `build.rs` script when you build the project with `cargo build`.

### Extensible Content with JSON

To allow for flexible and rapid development of new features (e.g., polls, multimedia posts), the project follows a convention of embedding structured data as JSON within generic string fields, such as `Post.content`.

-   **Backend:** When creating content that requires special handling, the backend should serialize a JSON object into the appropriate string field.
-   **Frontend:** The frontend is responsible for attempting to parse these string fields.
    -   If the string is valid JSON and contains the expected keys for a special content type, it should be rendered using a custom component or handler.
    -   If the string is not valid JSON or lacks the required structure, it must be treated as plain text and displayed as-is.

This approach allows for the introduction of new content types without requiring immediate changes to the protobuf schema, facilitating faster iteration.
