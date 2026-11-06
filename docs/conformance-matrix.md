# Preliminary conformance coverage plan

Status: **unverified plan**. This section inventory is not an exhaustive normative requirement matrix. No row represents a passing implementation test. Each row needs expansion into independently identified reader/writer requirements after retrieving and hashing the pinned source documents. Every such requirement must ultimately name an executable test and fixture, or an explicit unsupported capability and its release consequence.

Sources inspected: [in-progress AFF4 v1.0a](https://github.com/aff4/Standard/blob/master/inprogress/AFF4StandardSpecification-v1.0a.md) and [AFF4-L v1.0-ALPHA](https://github.com/aff4/Standard/blob/master/inprogress/AFF4-L-StandardSpecification-v1.0.md). See [baseline research](baseline-research.md) for revision uncertainty and unresolved ambiguities. Issue identifiers below are local backlog identifiers; they are not assumed GitHub numbers.

## Base document allocation

| Section | Reader issues | Writer issues | Planned evidence |
| --- | --- | --- | --- |
| 1, versioning | [002](issues/002.md) | [007](issues/007.md) | Accepted and rejected version files |
| 2, information model | [002](issues/002.md) | [007](issues/007.md) | Typed RDF inventory and retained unknown terms |
| 3, image streams | [003](issues/003.md), [004](issues/004.md) | [007](issues/007.md), [008](issues/008.md) | Known bytes across chunk and bevy boundaries |
| 4, maps | [005](issues/005.md) | [008](issues/008.md) | Gaps, targets, symbolic content and random reads |
| 5, storage | [002](issues/002.md) | [007](issues/007.md) | Independent ZIP and resource-name inspection |
| 6, hashing | [006](issues/006.md) | [007](issues/007.md), [008](issues/008.md) | Independent linear and structural digest vectors |
| 6.3, map order | [005](issues/005.md), [006](issues/006.md) | [008](issues/008.md) | Optional ordering data present and absent |
| Duplicate section 6, example | [002](issues/002.md), [006](issues/006.md) | [007](issues/007.md) | Cross-check examples against canonical bytes |
| 7, multiple ZIPs | [016](issues/016.md) | [017](issues/017.md) | Missing, duplicated and reordered volumes |

## Logical document allocation

| Section | Reader issues | Writer issues | Planned evidence |
| --- | --- | --- | --- |
| 1–2, names | [009](issues/009.md) | [013](issues/013.md) | Resource identity and ZIP member names |
| 3, version | [002](issues/002.md), [009](issues/009.md) | [013](issues/013.md) | Profile selection |
| 4, model | [009](issues/009.md), [010](issues/010.md), [012](issues/012.md) | [013](issues/013.md), [014](issues/014.md) | Namespace and typed-value fixtures |
| 5, normalization | [009](issues/009.md) | [013](issues/013.md) | Raw bytes, control characters and Unicode |
| 6.1, ZIP | [009](issues/009.md), [012](issues/012.md) | [013](issues/013.md) | External extraction and digest comparison |
| 6.2, inline | [010](issues/010.md) | [014](issues/014.md) | Encoding and size boundaries |
| 6.3–6.4, maps/streams | [011](issues/011.md), [012](issues/012.md) | [015](issues/015.md) | Shared storage and independent hash checks |
| 7–8, filenames | [016](issues/016.md) | [017](issues/017.md) | Container identity independent of filenames |
| 9, imports | [018](issues/018.md) | [018](issues/018.md) | Combined metadata, missing stores and cycles |
| 9a, HDT | [019](issues/019.md) | [019](issues/019.md) | Semantic equality against Turtle |
| 10.1, metadata hashes | [012](issues/012.md) | [013](issues/013.md) | Raw metadata bytes and tampering |
| First 10.2, signatures | [020](issues/020.md) | [020](issues/020.md) | External signer/verifier and trust cases |
| Second 10.2, imported hashes | [018](issues/018.md) | [018](issues/018.md) | Independently hashed secondary stores |

## Completion rules

- [001](issues/001.md) owns source provenance, the complete normative audit, oracle evidence, ambiguity resolution, and maintainer baseline decisions. Every table row is currently pending that evidence.
- [021](issues/021.md) adds adversarial cases throughout; malformed-input tests should begin with each parser rather than wait for that issue.
- [022](issues/022.md) owns release documentation and packaging. [023](issues/023.md) explores another binding. Neither establishes format conformance.
- Each initial capability must be exercised through Rust. Python bindings are deferred to AFF4-024 after the Rust release candidate; future binding tests will reuse independent fixture expectations.
- Track optional capability selection separately from conditional obligations once that capability is enabled. The unresolved namespace, inline, SHAKE, and signature questions must remain visible until evidence resolves them.
- Audit directory storage, codec identifiers, hash construction, and multipart scope explicitly before claiming comprehensive base support. Their inclusion in a broad issue title is insufficient proof of coverage.

## Implemented metadata reader evidence

The Rust reader now covers a subset of the versioning, storage and information
model rows through public API tests. It passes six canonical base metadata cases
with independently recorded expectations (see fixtures/base-metadata.json).
Synthetic tests additionally cover logical 2.1 bootstrap, malformed input and
resource limits. These are implementation checks, not completion of the full
section-level requirements or the outstanding baseline gate.
