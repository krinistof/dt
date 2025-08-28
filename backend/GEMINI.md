# Project Overview

This project is the backend service for Democratic Tier (dt). It is built with Rust and uses `tonic` to implement gRPC-web services and `axum` to serve the frontend application.

The backend provides two main gRPC services:
- **LogCollectorService**: A simple service for collecting client-side logs.
- **DtService**: The core service for the social media and voting system, handling posts, votes, and synchronization of events with clients.

## Building and Running

### Prerequisites

*   Rust: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
*   Protobuf Compiler: [https://grpc.io/docs/protoc-installation/](https://grpc.io/docs/protoc-installation/)

### Building

To build the project, run the following command:

```bash
cargo build
```

This command also builds the frontend assets.

### Running

To run the service, use the following command:

```bash
cargo run
```

The service will start and listen on `0.0.0.0:80`.

## Development Conventions

The project follows standard Rust conventions. The code is formatted using `rustfmt` and checked for errors using `clippy`.

### Protobuf

The gRPC services are defined in the `.proto` files in the `proto` directory. The Rust code for the services is generated automatically by the `build.rs` script. The main service definition is in `proto/dt/v1/dt.proto`. The build script also builds the frontend via npm commands ready to be served.

To regenerate the Rust code after changing the protobuf definition, simply build the project again:

```bash
cargo build
```
