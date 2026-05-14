# supply-depot

Sovereign registry proxy/cache and supply-chain policy POC backed by Starmetal.

> Status: `0.0.0` skeleton — public repo created so the boundary, upstream policy,
> and CI gates are explicit before implementation starts.

## Why it exists

A sovereign stack needs cached, inspectable dependency supply lines across ecosystems without hard-locking to public registries.

## Upstream relationship

- Upstream: [Starmetal](https://github.com/Goldziher/starmetal)
- Policy: upstream-first, pinned releases/commits, no permanent fork.
- Fork rule: fork only for a blocking security/build/sovereignty patch; open the upstream PR and remove the fork once merged.

## Contract shape

- native package-manager proxy endpoints where supported
- policy reports for licenses, yanks, and vulnerability severity
- storage backends compatible with filesystem and S3/Cellar-style object storage

## Non-goals

- no immediate production-critical registry dependency
- no opaque binary mirroring without checksums
- no policy bypass for AGPL/SSPL/proprietary packages

## Development

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features
cargo test --workspace --all-features
```

## License

MIT © Constantin Jais
