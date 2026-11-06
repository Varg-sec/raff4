# Design and evidence

## Product scope

Build an embeddable Rust reader, writer and verifier for physical and logical evidence. Start with synchronous, seekable input and incremental output. Consumers control filesystem access, resource budgets and cancellation. Expose owned handles, typed metadata, streaming reads and structured errors. Preserve unknown RDF data where possible. Avoid a mandatory CLI, runtime or Python dependency in the core.

The workspace follows the [Cargo book project layout](https://doc.rust-lang.org/cargo/guide/project-layout.html) as a library-plus-executable split: `crates/aff4` is the pure library package (entry point `src/lib.rs`, no binary target), and `crates/aff4-cli` is the separate executable package (`src/main.rs`), which depends on the library and provides the inspection CLI. The library never depends on the CLI.

The first phase implements and tests only the Rust library. The project uses Apache-2.0, selected by the maintainer on 2026-11-06. Keep an embeddable API with owned handles, structured errors and explicit resource lifetimes. These properties will support future adapters without adding binding dependencies now.

Once the Rust reader, writer and verifier are ready, deferred issue AFF4-024 adds a separate PyO3 adapter, maturin packaging, type stubs and Python integration tests. Python support versions will be selected at that point. Other language bindings are likewise deferred until after the Rust release candidate.

## Standards and precedence

The supplied [AFF4-L draft](https://github.com/aff4/Standard/blob/master/inprogress/AFF4-L-StandardSpecification-v1.0.md) is v1.0-ALPHA and references AFF4 v1.0a. Canonical images take precedence over conflicting prose. Logical container versioning is 2.1. Readers cover ZIP, inline, Map and ImageStream storage; writers need at least one representation. Metadata imports and integrity are part of the plan; accelerated HDT and signing are optional. Namespace examples, `dataSteam` spelling, inline size units and signature encoding need fixture-backed resolution. This is a moving draft, not a stable compatibility promise.

[Standard repository](https://github.com/aff4/Standard) and [base specification PDF](https://github.com/aff4/Standard/blob/master/AFF4StandardSpecification-v1.0.pdf): the in-progress base document has now been inspected; see [baseline research](baseline-research.md) and [preliminary conformance matrix](conformance-matrix.md). Verified draft revisions and digests are recorded in [standards.lock.json](standards.lock.json). Issue 001 must establish the exact base revision and requirements before implementation.

[Canonical reference images](https://github.com/aff4/ReferenceImages) include AFF4Std, AFF4-L and Errata directories. The draft's example image link uses a different path spelling; resolve the actual fixtures and pin their checksums. Six base-format fixtures have now been tested; see [fixture provenance](../fixtures/README.md).

## Implementation evidence

[pyaff4](https://github.com/aff4/pyaff4) is the primary implementation comparison requested by the project owner. Its README describes physical and logical operations, hashing, deduplication and experimental encryption, while also warning about write support. Pin actual source revisions and verify behavior per feature; the README is insufficient to certify compatibility.

[c-aff4](https://github.com/Velocidex/c-aff4) is a secondary comparison for overlapping base-format behavior. Its repository landing page was inspected, but implementation internals were not audited. Do not assume current logical-draft coverage.

[aff4tools](https://github.com/jbolas/aff4tools) offers a Rust library and CLI, including verification and a conformance command targeting v1.0a. Its README describes logical support as experimental and lists limitations. Use it as an independent cross-check, not the sole authority for AFF4-L compliance. Reuse decisions require source and license review in issue 001.

Initial web research was followed by verified standards and reference-image checkouts on 2026-11-06. Comparison implementation pins are still pending; see the baseline research follow-up.

## Delivery rules

Every initial capability is implemented and tested through the public Rust API. Deferred bindings must later reuse the same independent fixture expectations. Tests compare known bytes and semantic metadata, not merely writer/reader self-consistency. Writers finalize explicitly and propagate errors. Verification reports distinguish mismatches, unsupported algorithms, missing evidence and unchecked content. Never call an image fully verified when coverage is incomplete.

Start malformed-input testing with the first parser; the hardening issue expands this into fuzzing and release gates. Keep memory proportional to configured buffers wherever practical and measure metadata scaling separately. Logical names are evidence metadata, never implicit extraction destinations.

Full filesystem acquisition, a desktop UI, network-backed streams, encryption, legacy compatibility and in-place append are outside the initial scope. Legacy reads may be added explicitly; current-profile writers must not emit legacy forms accidentally. Optional HDT, signatures and another language adapter are planned separately. Mandatory-profile coverage is confirmed by baseline review rather than inferred from these exclusions.
