# Design and evidence

## Product scope

Build an embeddable Rust reader, writer and verifier for physical and logical evidence. Start with synchronous, seekable input and incremental output. Consumers control filesystem access, resource budgets and cancellation. Expose owned handles, typed metadata, streaming reads and structured errors. Preserve unknown RDF data where possible. Avoid a mandatory CLI, runtime or Python dependency in the core.

The proposed layout is a Rust workspace with the `aff4` core and a separate PyO3 adapter. Python wrappers expose context managers, file-like streams and structured reports. PyO3 generates extension glue for annotated wrappers; it does not automatically translate any Rust API or generate bindings for arbitrary languages. Maturin handles Python packaging. Type stubs and another language adapter have explicit backlog coverage. [PyO3 guide](https://pyo3.rs/main/)

## Standards and precedence

The supplied [AFF4-L draft](https://github.com/aff4/Standard/blob/master/inprogress/AFF4-L-StandardSpecification-v1.0.md) is v1.0-ALPHA and references AFF4 v1.0a. Canonical images take precedence over conflicting prose. Logical container versioning is 2.1. Readers cover ZIP, inline, Map and ImageStream storage; writers need at least one representation. Metadata imports and integrity are part of the plan; accelerated HDT and signing are optional. Namespace examples, `dataSteam` spelling, inline size units and signature encoding need fixture-backed resolution. This is a moving draft, not a stable compatibility promise.

[Standard repository](https://github.com/aff4/Standard) and [base specification PDF](https://github.com/aff4/Standard/blob/master/AFF4StandardSpecification-v1.0.pdf): the repository listing was inspected; the PDF contents and complete in-progress base inventory could not be retrieved in this session. Issue 001 must establish the exact base revision and requirements before implementation.

[Canonical reference images](https://github.com/aff4/ReferenceImages) include AFF4Std, AFF4-L and Errata directories. The draft's example image link uses a different path spelling; resolve the actual fixtures and pin their checksums. No fixture was downloaded or tested in this session.

## Implementation evidence

[pyaff4](https://github.com/aff4/pyaff4) is the primary implementation comparison requested by the project owner. Its README describes physical and logical operations, hashing, deduplication and experimental encryption, while also warning about write support. Pin actual source revisions and verify behavior per feature; the README is insufficient to certify compatibility.

[c-aff4](https://github.com/Velocidex/c-aff4) is a secondary comparison for overlapping base-format behavior. Its repository landing page was inspected, but implementation internals were not audited. Do not assume current logical-draft coverage.

[aff4tools](https://github.com/jbolas/aff4tools) offers a Rust library and CLI, including verification and a conformance command targeting v1.0a. Its README describes logical support as experimental and lists limitations. Use it as an independent cross-check, not the sole authority for AFF4-L compliance. Reuse decisions require source and license review in issue 001.

Sources were accessed through web retrieval on 2026-10-30. No immutable revisions were obtained; pinning them is deliberately a first issue acceptance criterion.

## Delivery rules

Every capability runs through Rust and Python after the initial adapter exists. Tests compare known bytes and semantic metadata, not merely writer/reader self-consistency. Writers finalize explicitly and propagate errors. Verification reports distinguish mismatches, unsupported algorithms, missing evidence and unchecked content. Never call an image fully verified when coverage is incomplete.

Start malformed-input testing with the first parser; the hardening issue expands this into fuzzing and release gates. Keep memory proportional to configured buffers wherever practical and measure metadata scaling separately. Logical names are evidence metadata, never implicit extraction destinations.

Full filesystem acquisition, a desktop UI, network-backed streams, encryption, legacy compatibility and in-place append are outside the initial scope. Legacy reads may be added explicitly; current-profile writers must not emit legacy forms accidentally. Optional HDT, signatures and another language adapter are planned separately. Mandatory-profile coverage is confirmed by baseline review rather than inferred from these exclusions.
