# Reader fixtures

`base-metadata.json` pins six canonical AFF4Std volumes at ReferenceImages commit
`84773b088bf6cce551a515d8ebb486bad69b58b8`. Tests check the entire container's
SHA-256 before reading it. Expected volume identities and primary RDF graph
counts, image ARNs and sizes were independently obtained with Python's zipfile
and RDFLib 7.1.4 on 2026-11-06. This is a metadata oracle, not an AFF4 content or
hash verifier. No Python bindings or runtime dependency are part of the aff4 library.

The Rust integration test reads the manifest directly. Synthetic tests generate
small ZIP64 containers in memory and exercise malformed input, CRC checking,
resource limits, RDF values and both recognized version profiles.

## Run the canonical regression

The reference repository is a git submodule at `fixtures/downloads/reference`,
pinned to commit `84773b088bf6cce551a515d8ebb486bad69b58b8`. Initialize it and
run the test:

```sh
git submodule update --init fixtures/downloads/reference
AFF4_REFERENCE_IMAGES="$PWD/fixtures/downloads/reference" \
  cargo test --locked -p aff4 --test canonical -- --ignored
```

A missing checkout, altered fixture or absent environment variable fails this
explicit test run. Ordinary `cargo test` marks it ignored; it does not count it
as passed. CI runs the canonical test separately against the pinned submodule
checkout. Fixture binaries are not committed here because no explicit
redistribution license was found in the reference repository.

At that revision, AFF4-L/README.md says canonical logical images are forthcoming;
its existing images are under `deprecated/`. They are not evidence of conformance
to the 2026 draft. The reader deliberately reports version 1.1 as unsupported.
Version 2.1 bootstrap support is tested synthetically only.
