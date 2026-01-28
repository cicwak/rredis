# rredis

A personal Rust practice project: building a small Redis-like in-memory data store in Rust.

The main goal is to learn Rust by implementing real-world systems concepts (network protocol design, concurrency, persistence, replication, clustering) in a focused, incremental way.

## Project Goals

### Current / Next up
- **TTL (time-to-live)** for keys
- **Automatic expiration cleanup** (background eviction / lazy + active strategies)
- **Persistence to disk** (enabled via a flag), with three modes:
  - **Snapshot**: full in-memory dump every **N** seconds interval
  - **Command log** (append-only log of commands)
  - **Hybrid**: snapshot every **N** seconds/commands + command log in between
- **More data types**
    - Arrays / lists
    - Hash map / hash (key-value fields)
- **Third-party module integration**
    - Allow external modules/plugins to extend commands and/or data types
- **Transactions**
    - Execute a group of commands with **all-or-nothing** semantics
- **Replication**
    - Master–slave
    - Master–master
- **Clustering**
    - Partitioning / sharding
    - Basic coordination and routing

[//]: # (## Non-Goals &#40;for now&#41;)

[//]: # (- Full Redis command compatibility)

[//]: # (- Perfect performance parity with mature databases)

[//]: # (- Production-grade security hardening)

[//]: # (## Why this exists)

[//]: # (- Practice Rust on a project that naturally grows in complexity)

[//]: # (- Explore trade-offs in database design:)

[//]: # (    - TTL and eviction strategies)

[//]: # (    - Persistence formats and recovery)

[//]: # (    - Consistency models for replication)

[//]: # (    - Cluster topology and rebalancing)

## Status
This project is under active development and may change frequently as features are added.
