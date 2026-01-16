# Sequence of events between server and client

```mermaid
sequenceDiagram
    autonumber
    participant Client
    box rgba(31, 161, 250, .5) Democratic Tier
        participant Server
        participant Database@{type: "database"}
    end

    Note over Client: User scans QR.<br/>Loads GroupKey & UserKey.<br/>Derives UserPubKey.

    loop Sync
        Client->>+Server: Get events since `last_timestamp`
        Server<<->>Database: Select * from events > timestamp
        Server-->>-Client: Returns `SignedEvents[]`
        
        Note over Client: Client iterates events:<br/>1. Check signature in whitelist<br/>2. Decrypt with GroupKey<br/>3. Update UI
    end

    Note left of Client: User posts "Hello"
    
    Note over Client: 1. Create JSON: {type: "post", txt: "Hello"}<br/>2. Encrypt with GroupKey -> `CipherBytes`<br/>3. Sign `CipherBytes` with UserPrivKey
    
    Client->>+Server: Push EventRequest:<br/>(UserPubKey, Signature, CipherBytes)
    
    Note right of Server: 1. Check if UserPubKey in Whitelist<br/>2. Verify Signature matches CipherBytes<br/>3. DO NOT DECRYPT
    
    alt Verification Failed
        Server-->>Client: 403 Go Away
    else Verified
        Server->>Database: INSERT INTO events (pubkey, sig, blob, timestamp)
        Server-->>-Client: 200 OK (EventID)
    end
    
    Note left of Client: User votes
    
    Note over Client: 1. Create JSON:<br/>{type: "vote", ref: signature, val: 1234}<br/>2. Encrypt with GroupKey -> `CipherBytes`<br/>3. Sign `CipherBytes` with UserPrivKey
    
    Client->>Server: Push EventRequest:<br/>(UserPubKey, Sig, CipherBytes)
    Note right of Server: Server has NO IDEA this is a vote.<br/>It just sees another valid blob.
    Server->>Database: if valid insert...
```

# From events to rendering

```mermaid
graph LR
    subgraph "Group Events (Decrypted)"
        E1("{Type: Post, ID: A, Author: Bob,...}")
        E2("{Type: Vote, Ref: A, Val: 1, Author: Alice}")
        E3("{Type: Vote, Ref: A, Val: 1, Author: Bob}")
        E4("{Type: Vote, Ref: A, Val: 2, Author: Alice}")
    end

    subgraph "Aggregator"
        Reducer[Reducer Function<br/>'Map&lt;ContentHash, State&gt;']
    end

    subgraph "Derived UI"
        Store[Local State Store]
        S1("Post A State:<br/>- Content: '...'<br/>- Votes: {Alice: 2, Bob: 1}<br/>- Score: 3")
    end


    E1 & E2 & E3 & E4 --> Reducer
    Reducer --> Store
```
