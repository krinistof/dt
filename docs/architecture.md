# UniQue (uq) Architecture

The project is structured as a monorepo containing the core protocol libraries and the Democratic Tier application. The architecture separates the core synchronization logic (`uq`) from the specific application implementation.

## Components

### 1. Core Library (`crates/uq`)
-   **Role**: Core Rust library handling logic, storage, and cryptography.
-   **Functionality**: Implements the `Sync` protocol for agnostic event synchronization.
-   **Identity**: Manages identity and verification.

### 2. Server (`crates/uq-server`)
-   **Role**: Standalone server binary.
-   **Stack**: ConnectRPC / Axum.
-   **Features**:
    -   Exposes `SyncService` for clients to fetch/push events.
    -   Integrates `collect_log` for generic client telemetry.
    -   Configurable SQLite storage.

### 3. Client Library (`packages/uq-client`)
-   **Role**: TypeScript client library.
-   **Functionality**:
    -   ConnectRPC client generation.
    -   Cryptography (Key generation, Signing).
    -   State management helper ("Reducer" pattern).
    -   Built-in error reporting.

## Protocol & Cryptography

The protocol uses **Asymmetric Topics**, meaning all topic members have access to the shared asymmetric key, which they use to encrypt events.

### Data Model
Events are cryptographically bound to a Topic.

-   **Topic**: Identified by a unique Public Key (`topic_pk`).
-   **Event Structure**:
    -   `topic_pk`: The identifier of the topic.
    -   `blob`: The opaque data payload (handled by upper layers).
    -   `sig`: Signature verifying the event.

### Verification
To ensure integrity and prevent replay attacks across topics, the signature verification enforces:

`Verify(user_pk, signature, hash(blob) + topic_pk)`

## Sequence of Events

### Client-Server Sync

```mermaid
sequenceDiagram
    autonumber
    participant Client
    participant Server
    participant Database

    Note over Client: User loads Topic (`topic_pk`)

    loop Sync
        Client->>+Server: Get events for `topic_pk` since `last_timestamp`
        Server<<->>Database: Select * from events where topic_pk = ? AND timestamp > ?
        Server-->>-Client: Returns `SignedEvents[]`
        
        Note over Client: Client iterates events:<br/>1. Verify Signature (hash(blob) + topic_pk)<br/>2. Parse blob (decrypt if needed)<br/>3. Update UI via Reducer
    end
```

### Event Publication

```mermaid
sequenceDiagram
    autonumber
    participant Client
    participant Server
    participant Database

    Note left of Client: User creates content (Post/Vote)
    
    Note over Client: 1. Create Payload (blob)<br/>2. Sign(UserPrivKey, hash(blob) + topic_pk)
    
    Client->>+Server: Push EventRequest:<br/>(topic_pk, user_pk, signature, blob)
    
    Note right of Server: 1. Verify Signature matches blob + topic_pk<br/>2. DO NOT inspect blob content
    
    alt Verification Failed
        Server-->>Client: 403 Forbidden
    else Verified
        Server->>Database: INSERT INTO events (topic_pk, pubkey, sig, blob, timestamp)
        Server-->>-Client: 200 OK (EventID)
    end
```

### Data Flow (Reducer Pattern)

```mermaid
graph LR
    subgraph "Topic Events"
        E1("{TopicPK: T1, Type: Post...}")
        E2("{TopicPK: T1, Type: Vote...}")
    end

    subgraph "Client App"
        Reducer[Reducer Function]
        Store[Local State Store]
    end

    E1 & E2 --> Reducer
    Reducer --> Store
    Store --> UI
```
