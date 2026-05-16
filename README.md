# supply-depot

> Sovereign registry proxy/cache with supply-chain policy enforcement across Cargo, npm, and PyPI — cached, inspectable dependency supply lines without hard-locking to public registries.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Rust 1.95+](https://img.shields.io/badge/Rust-1.95%2B-orange.svg)](https://www.rust-lang.org)
[![CI](https://github.com/constantin-jais/supply-depot/actions/workflows/ci.yml/badge.svg)](https://github.com/constantin-jais/supply-depot/actions/workflows/ci.yml)

> **Status:** `0.0.0` skeleton — boundary, upstream policy, and CI gates are explicit before implementation starts.

## Why it exists

A sovereign stack needs cached, inspectable dependency supply lines across ecosystems without hard-locking to public registries. `supply-depot` adds a controllable mirror layer: cache what you rely on, enforce license and vulnerability policy, and eliminate the single-registry SPOF.

## Ecosystem

```mermaid
graph TB
    subgraph product["🎯 Product"]
        RL["Presto-Matic · rumble-lm<br/>Collaborative Learning App"]
    end
    subgraph agentic["🤖 Agentic Tools"]
        AOM["agent-o-matic<br/>Config Compiler + Orchestrator"]
        DL["disc-loader<br/>Document Ingestion Worker"]
        MC["memory-card<br/>Local Agent Context"]
    end
    subgraph devops["🔧 DevOps Tools"]
        LC["link-cable<br/>Distribution Substrate"]
        SD["supply-depot<br/>Registry Proxy / Cache"]
        VI["vault-inspector<br/>Postgres Security Audit"]
    end
    RL --> DL
    RL --> MC
    RL --> VI
    RL --> SD
    RL --> LC
    AOM --> LC
    DL --> MC
    style SD fill:#dbeafe,stroke:#2563eb,stroke-width:2px
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
| [Presto-Matic](https://github.com/constantin-jais/Rumble-LM)          | Sovereign learning platform — supply chain consumer |
| [agent-o-matic](https://github.com/constantin-jais/Agent-O-Matic)     | Config compiler and autonomous orchestrator         |
| [disc-loader](https://github.com/constantin-jais/disc-loader)         | Document ingestion worker                           |
| [memory-card](https://github.com/constantin-jais/memory-card)         | Local agent context layer                           |
| [link-cable](https://github.com/constantin-jais/link-cable)           | Multi-platform distribution substrate               |
| [vault-inspector](https://github.com/constantin-jais/vault-inspector) | Postgres security audit                             |

## License

MIT © Constantin Jais
