# libaff4

A planned Rust library for AFF4 physical and AFF4-L logical evidence containers, with a separate Python adapter using PyO3 and maturin.

**Status:** project scaffold and implementation backlog only. No container API, Python extension, or conformance claim exists yet. The local Git repository is initialized and the backlog is approved. GitHub setup and issue publication remain blocked by session access restrictions.

## Project plan

- [Design, sources and scope](docs/design.md)
- [23 implementation issue drafts and dependencies](docs/backlog.md)
- [Machine-readable issue manifest](docs/issues.json)
- [GitHub setup and publication](docs/github-setup.md)

## Local checks

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --offline -- -D warnings
cargo test --workspace --offline
```

The current crate deliberately contains no placeholder format implementation. A successful build validates the scaffold only. Minimum Rust version is provisionally 1.85 and must be tested before release.

License and publishing names are pending maintainer selection; crates.io publication is disabled.
