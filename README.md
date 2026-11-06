# aff4

A Rust library for AFF4 physical and AFF4-L logical evidence containers, licensed
under [Apache-2.0](LICENSE). Python bindings are deferred until the Rust core is ready.

## Implemented

The first reader opens ZIP/ZIP64 containers from files or any `Read + Seek`
source. It reads volume identity and version, parses primary Turtle metadata,
enumerates image identifiers, and exposes owned RDF terms with structured errors.
Caller-configurable limits cover bootstrap data, metadata bytes, parsed triples
and member count. The ZIP dependency reads the central directory before the
member-count limit is applied; these limits are not a total memory cap.

```rust,no_run
use aff4::Container;

let container = Container::open("evidence.aff4")?;
println!("{}", container.volume_iri());
for image in container.images() {
    println!("{image}");
}
# Ok::<(), aff4::Error>(())
```

## Current limits

Opening a container does not verify its integrity or establish conformance.
Image-stream and map content reads, metadata imports, writing, multipart assembly,
and hash verification are not implemented. RDF-star is unsupported; unknown
ordinary RDF predicates and datatypes are retained. Queries use exact IRIs and
perform no type inference. Bootstrap versions 1.0 and 2.1 are recognized; other
versions, including legacy logical 1.1, return an unsupported-version error.

The reader matches independently checked metadata from six canonical base
volumes. Logical 2.1 bootstrap tests are synthetic: the upstream logical reference
set at the pinned revision contains only deprecated images.

## Development

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
```

See [fixture instructions](fixtures/README.md) for the separate canonical suite.
The minimum supported Rust version is 1.85. Crates.io publication is disabled
while the API is under development.

- [Architecture and scope](docs/design.md)
- [Implementation backlog](docs/backlog.md)
- [Standards research](docs/baseline-research.md)
- [Preliminary conformance coverage](docs/conformance-matrix.md)
