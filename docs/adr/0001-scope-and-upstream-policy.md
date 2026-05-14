# ADR-0001 — Scope and upstream policy

- Status: Accepted
- Date: 2026-06-29
- Upstream: [Starmetal](https://github.com/Goldziher/starmetal)

## Context

`supply-depot` is a companion repository in the Presto-Matic / Agent-O-Matic ecosystem. Its role is **sovereign supply-chain depot**. It is intentionally separate from the Presto-Matic product repo so heavy dependencies, operational lifecycle, and upstream tracking stay isolated.

## Decision

Build `supply-depot` as an upstream-first, sovereign Rust project:

- track upstream releases/tags/commits explicitly;
- keep local patches small and temporary;
- expose stable contracts rather than leaking upstream internals to consumers;
- enforce permissive OSS licensing and vulnerability gates in CI;
- default to self-hosted/EU-resident operation and avoid US hyperscaler requirements.

## Integration contract

- native package-manager proxy endpoints where supported
- policy reports for licenses, yanks, and vulnerability severity
- storage backends compatible with filesystem and S3/Cellar-style object storage

## Non-goals

- no immediate production-critical registry dependency
- no opaque binary mirroring without checksums
- no policy bypass for AGPL/SSPL/proprietary packages

## Consequences

- The companion can iterate independently from Presto-Matic.
- Presto-Matic avoids accidental dependency bloat and can roll back integration by switching contracts off.
- Upstream changes are absorbed deliberately through version bumps, changelog review, and contract tests.
