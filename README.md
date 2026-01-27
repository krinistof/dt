## Democratic Tier: Local Social Multi-media

What if we can make social media find common goals easier, while keeping it privacy focused, yet critically stable?
What if we want to move past paper based votes, polls for smaller but still important decisions of local groups, even in emergencies? Paper has been used for thousands of years, for many reasons, mostly for it's robustness. Why don't we have stable, open source solutions for making fair group decisions? 

# Goals
## Wide support of smartphones
We need to let everyone express their opinion!

We want to create the next most stable polling system after paper. Support for wide variety of smartphones combined with great user experience for inclusiveness is key. To achieve usage on a wide scale we need to have an optimized, well supported technology to provide for even older, slower smartphones.

## Score based voting
Reject forced extremes!

Most social media only let the voter like content. Using paper for more detailed vote systems - like score based ones - weren't viable, because tallying results was a nightmare. Score based voting is superior in detailing everyone's opinion, and avoids common vote system pitfalls, paradoxes.

## Transparency for trust
Trust and visibility go hand in hand!

The system provides a fully transparent and auditable event log of all votes. Each vote is recorded with a unique, anonymous voter ID, the song ID, the score given, and a timestamp. This allows for complete transparency of the voting process, while still protecting the privacy of the individual voters. The event log is publicly accessible, allowing anyone to verify the results and ensure the fairness of the poll.

## Total locality for robustness
No third parties, external dependencies!

Social networks shouldn't always depend on internet access. Imagine polls in remote locations with limited network access. Setting up a local network with a WiFi access point and a local service, with pre-printed session cards as cookies, each with secret voter token can create a stable live polling system. With isolated networks and a secure way to verify voters without needing the internet we can guarantee only people present can cast their votes.

## Multi-media support
Vote on anything!

Why should we only have thoughts, images, songs, videos as options? Why not all of them? The scoring logic is the same regardless.

# Development & Architecture

This repository is being refactored into a **Monorepo** hosting both the core protocol and the application.

## Project Structure

-   **`crates/uq`**: Core Rust library for the **UniQue** protocol (Logic, Storage, Crypto).
-   **`crates/uq-server`**: Standalone backend server (ConnectRPC + Axum).
-   **`packages/uq-client`**: TypeScript client library.
-   **`apps/dt-frontend`**: The Democratic Tier frontend application.
-   **`proto/uq`**: Protocol Buffer definitions.

## The UniQue Protocol

**Democratic Tier** is built on top of **UniQue**, a decentralized event synchronization protocol. 
UniQue uses **Asymmetric Topics** where events are cryptographically bound to a topic via `topic_pk`.

For more details on the architecture, see [docs/architecture.md](docs/architecture.md).

## Roadmap

See [TODO.md](TODO.md) for the active development plan and refactoring status.
