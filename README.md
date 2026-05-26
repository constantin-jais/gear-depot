# gear-depot

> Sovereign registry proxy/cache with supply-chain policy enforcement across Cargo, npm, and PyPI — cached, inspectable dependency supply lines without hard-locking to public registries.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Rust 1.95+](https://img.shields.io/badge/Rust-1.95%2B-orange.svg)](https://www.rust-lang.org)
[![CI](https://github.com/constantin-jais/gear-depot/actions/workflows/ci.yml/badge.svg)](https://github.com/constantin-jais/gear-depot/actions/workflows/ci.yml)

> **Status:** `0.0.0` skeleton — boundary, upstream policy, and CI gates are explicit before implementation starts.

## Why it exists

A sovereign stack needs cached, inspectable dependency supply lines across ecosystems without hard-locking to public registries. `gear-depot` adds a controllable mirror layer: cache what you rely on, enforce license and vulnerability policy, and eliminate the single-registry SPOF.

## Ecosystem

```mermaid
graph TB
    subgraph products["🎯 Rumble products"]
        RL["rumble-lm<br/>Collaborative learning platform"]
        RC["rumble-cos<br/>Public knowledge site"]
    end
    subgraph tools["🛠️ Sovereign tooling"]
        CM["cos-matic<br/>Agent config + autonomous code-ops"]
        WL["wrench-loader<br/>Document ingestion worker"]
        GM["gear-memory<br/>Local agent context"]
    end
    subgraph infra["⚙️ Infrastructure"]
        GC["gear-cable<br/>Distribution substrate"]
        GD["gear-depot<br/>Registry proxy/cache"]
        VI["vault-inspector<br/>Postgres security audit"]
    end
    RL --> WL
    RL --> GM
    RL --> VI
    RL --> GD
    RL --> GC
    RC --> RL
    CM --> GC
    WL --> GM
    style GD fill:#dbeafe,stroke:#2563eb,stroke-width:2px
```

## Contract

|             |                                                       |
| ----------- | ----------------------------------------------------- |
| **Proxy**   | Native package-manager endpoints for Cargo, npm, PyPI |
| **Policy**  | License, yank, and vulnerability severity reports     |
| **Storage** | Filesystem or S3/Cellar-compatible object storage     |

## Non-goals

- No immediate production-critical registry dependency
- No opaque binary mirroring without checksums
- No policy bypass for AGPL/SSPL/proprietary packages

## Upstream

|               |                                                                                                            |
| ------------- | ---------------------------------------------------------------------------------------------------------- |
| **Project**   | [Starmetal](https://github.com/Goldziher/starmetal)                                                        |
| **Policy**    | Upstream-first, pinned releases/commits, no permanent fork                                                 |
| **Fork rule** | Only for a blocking security/build/sovereignty patch; open the upstream PR and remove the fork once merged |

## Development

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features
cargo test --workspace --all-features
```

## Related projects

| Repo                                                                  | Role                                                |
| --------------------------------------------------------------------- | --------------------------------------------------- |
| [rumble-lm](https://github.com/constantin-jais/rumble-lm)          | Sovereign learning platform — supply chain consumer |
| [cos-matic](https://github.com/constantin-jais/cos-matic)     | Config compiler and autonomous orchestrator         |
| [wrench-loader](https://github.com/constantin-jais/wrench-loader)         | Document ingestion worker                           |
| [gear-memory](https://github.com/constantin-jais/gear-memory)         | Local agent context layer                           |
| [gear-cable](https://github.com/constantin-jais/gear-cable)           | Multi-platform distribution substrate               |
| [vault-inspector](https://github.com/constantin-jais/vault-inspector) | Postgres security audit                             |

## License

MIT © Constantin Jais
