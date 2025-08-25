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
