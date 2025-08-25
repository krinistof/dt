# Project Overview

This project is a backend service for collecting logs. It is built with Rust and uses the tonic library to implement a gRPC-web service.

The service exposes a single endpoint, `log`, which accepts a log message and returns a success status. The service is configured with CORS to allow requests from any origin, which is useful for development but should be restricted in production.

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

The service will start and listen on `[::1]:50051`.

### Testing

There are no tests in this project yet.

## Development Conventions

The project follows standard Rust conventions. The code is formatted using `rustfmt` and checked for errors using `clippy`.

### Protobuf

The gRPC service is defined in the `proto/log/v1/log.proto` file. The Rust code for the service is generated automatically by the `build.rs` script.

To regenerate the Rust code after changing the protobuf definition, simply build the project again:

```bash
cargo build
```