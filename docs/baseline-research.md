# Baseline research for AFF4-001

Inspected on 2026-11-06 using web retrieval. This is an evidence inventory, not a conformance claim. No fixture was downloaded, hashed, decoded, or executed during this research. Cached branch views do not establish current branch identity or content digests.

## Implementation follow-up — 2026-11-06

Network access through an approved tool call now permits real source retrieval.
The standards commit `da883fe48b15dec933a373f40e85e48b7b6c9214` was verified by
`git ls-remote` and cloned. Both draft document hashes and their licenses are
recorded in [standards.lock.json](standards.lock.json). Earlier retrieval notes
below describe the preceding web-only research stage.

ReferenceImages commit `84773b088bf6cce551a515d8ebb486bad69b58b8` was cloned.
Its AFF4-L README explicitly says canonical logical images are forthcoming and
places earlier examples under `deprecated/`; the earlier cached directory view
was stale. Six AFF4Std volumes were hashed and independently inspected with
zipfile and RDFLib 7.1.4. Rust regression expectations are in
[base-metadata.json](../fixtures/base-metadata.json). This validates metadata
inspection only, not content hashes or full AFF4 conformance. Library licensing
is Apache-2.0 and initial implementation is Rust-only, per maintainer direction.

Remaining issue 001 work includes pins and license review for comparison
implementations, the complete normative matrix, independent image-content
verification, and resolution of draft ambiguities. Metadata-reader development
can proceed without claiming those broader gates complete.

## Documents and revision evidence

The actual base draft is [inprogress/AFF4StandardSpecification-v1.0a.md](https://github.com/aff4/Standard/blob/master/inprogress/AFF4StandardSpecification-v1.0a.md). Its revision table describes February 2022 changes to chunk padding, block-map hashing, mapPath, and striped images. It specifies the base namespace `http://aff4.org/Schema#` and gives canonical images precedence over conflicting prose. Its license is GFDL 1.3 or later, with no invariant sections or cover texts. The `dataStream` property refers to an image's Map or ImageStream. This draft must be reviewed independently from the older PDF.

The [standards commit history](https://github.com/aff4/Standard/commits/master) exposes [da883fe48b15dec933a373f40e85e48b7b6c9214](https://github.com/aff4/Standard/commit/da883fe48b15dec933a373f40e85e48b7b6c9214), dated September 3, 2026, as the draft release. The commit page and revision-addressed raw document could not be fetched. Treat this as a candidate revision, not a verified content pin. No SHA-256 digest was obtained.

## Logical draft observations

The following observations refer to the inspected [AFF4-L draft](https://github.com/aff4/Standard/blob/master/inprogress/AFF4-L-StandardSpecification-v1.0.md), also available as [raw Markdown](https://raw.githubusercontent.com/aff4/Standard/master/inprogress/AFF4-L-StandardSpecification-v1.0.md).

- It identifies itself as v1.0-ALPHA, extends AFF4 v1.0a, uses container version 2.1, and carries Apache-2.0 terms.
- Canonical images override conflicting prose.
- Section 4.1 specifies `https://aff4.org/Schema/2022/#`; section 9 examples use HTTP. Imports prose alternates `aff4l:imports` and `aff4:imports`.
- Section 6.2 examples use `dataSteam`; section 6.3 uses `aff4l:dataStream` for indirect streams.
- The inline limit is written `1Kb`, without a byte definition.
- Section 4.3 lists SHAKE128/256 without output lengths.
- Section 10.2 requires PEM signatures and certificate chains but does not identify a signature container or algorithm encoding.
- Readers must support all four storage forms; writers need at least one. HDT and signing are optional.
- Metadata hashing is mandatory; SHA256, SHA512, or stronger supported algorithms are required.

These observations leave namespace selection, inline spelling and size, SHAKE lengths, and signature serialization unresolved.

## Fixture evidence

The [reference repository](https://github.com/aff4/ReferenceImages) distinguishes `AFF4Std`, `AFF4PreStd`, and `Errata`. Its README describes the standard images as ground truth and identifies the pre-standard set separately. The [AFF4-L directory](https://github.com/aff4/ReferenceImages/tree/master/AFF4-L) lists `dream.aff4`, `dream.txt`, `information.turtle`, `unicode.aff4`, `unicode.zip`, `utf8segment-macos.zip`, and a Japanese-named text file. This is a different path from the logical draft's `aff4l` link. Retrieving the standalone Turtle bytes failed; no conclusions about those files' namespaces or draft compatibility follow from their names. The directory listing alone does not establish redistribution rights or coverage of the 2026 draft.

## Proposed implementation and release gates

These are project proposals, not additional rules inferred from the standard:

1. Fetch source bytes at verified commits, calculate content digests locally, and retain per-artifact provenance and license records. A failed fetch must remain blocked.
2. Preserve complete RDF IRIs. Add compatibility aliases only when fixture evidence documents their meaning; keep strict writer behavior distinct from tolerant reads.
3. Model an inline byte payload separately from a resource reference. Never globally replace `dataSteam` with `dataStream` without considering RDF term type and namespace.
4. Require boundary fixtures around every candidate inline limit before adopting one. Use ZIP storage for initial logical writes while the inline limit remains unresolved.
5. Track variable-output digest length explicitly. Do not derive SHAKE output length from the algorithm name or silently choose a default for verification.
6. Report signing as unsupported until an independently produced signed fixture establishes the exact bytes to sign, signature container, encoding, and algorithm identifiers. Metadata hash verification and signer trust should have separate report fields.
7. Verify that each purported logical reference image actually exercises the intended draft revision before promoting it to a normative test. Retain historical fixtures with explicit profile labels.
8. Keep every unresolved item visible in the release report. Reader completeness requires inline support even if the first writer uses only ZIP segments.

Issue 001 still needs verified source pins and digests, fixture licensing, an exhaustive normative requirement matrix, independent oracle execution with expected object identifiers/content/metadata, and maintainer decisions on the release baseline. This report resolves discovery of the base draft only; it does not satisfy those remaining gates.
