# Project Overview

This project is the backend service for "Democratic Tier" (dt). It is built with Rust and uses the `tonic` library to implement a gRPC-web service.
The server's primary role is to be a verifiable, agnostic bulletin board. It stores and synchronizes encrypted events without knowing their contents.

## Core Responsibilities

1.  **Event Storage:** Accepts and stores encrypted event blobs.
2.  **Access Control:** Verifies that the publisher of an event is in the allowed `whitelist` using their Public Key.
3.  **Integrity:** Verifies that the event signature matches the encrypted blob.
4.  **Privacy:** **DOES NOT DECRYPT** any user content. It sees and stores only opaque byte arrays providing data breach resillence.

## Endpoints

The service primarily exposes service endpoints for:
*   **Push:** Receiving new encrypted events `(UserPubKey, Signature, CipherBytes)`.
*   **Sync:** Serving a list of events since a given timestamp.
*   **Log:** Receiving client logs for debugging.

## Building and Running

### Prerequisites

*   Rust: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
*   Protobuf Compiler: [https://grpc.io/docs/protoc-installation/](https://grpc.io/docs/protoc-installation/)

### Building

To build the project, run the following command:

```bash
cargo build
```

This command also builds the frontend assets, and fails if the frontend cannot be built.

### Running

The developer is responsible to run the server. If not asked otherwise, don't try to run it, ask the developer.

## Development Conventions

The project follows standard Rust conventions. The code is formatted using `rustfmt` and checked for errors using `clippy`.
