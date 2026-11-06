# Implementation backlog

Published as issues #1–#23, then revised on 2026-11-06 for Apache-2.0 and Rust-first delivery; these are local IDs, not GitHub issue numbers. Each issue delivers a verifiable consumer capability. AFK means implementation can proceed after its blockers; HITL means an explicit maintainer decision is needed. No originating PRD issue exists.

Suggested labels: `type:afk`, `type:hitl`, `status:planned`, and the area shown below. Do not mark blocked issues ready.

1. **[Pin the conformance baseline and runnable reference corpus](issues/001.md)** — AFF4-001; HITL; blocked by: —; area: foundation.
2. **[Open a container and inspect its metadata from Rust](issues/002.md)** — AFF4-002; AFK; blocked by: AFF4-001; area: reader.
3. **[Read and seek uncompressed AFF4 image streams](issues/003.md)** — AFF4-003; AFK; blocked by: AFF4-002; area: reader.
4. **[Read compressed AFF4 image streams](issues/004.md)** — AFF4-004; AFK; blocked by: AFF4-003; area: reader.
5. **[Read sparse and mapped physical images](issues/005.md)** — AFF4-005; AFK; blocked by: AFF4-003; area: reader.
6. **[Verify physical image content and structural hashes](issues/006.md)** — AFF4-006; AFK; blocked by: AFF4-004, AFF4-005; area: integrity.
7. **[Write and reopen an uncompressed physical image](issues/007.md)** — AFF4-007; AFK; blocked by: AFF4-003, AFF4-006; area: writer.
8. **[Write compressed and sparse physical images](issues/008.md)** — AFF4-008; AFK; blocked by: AFF4-004, AFF4-005, AFF4-007; area: writer.
9. **[Browse logical acquisitions and read ZIP-backed files](issues/009.md)** — AFF4-009; AFK; blocked by: AFF4-002; area: logical.
10. **[Read inline data, alternate streams and extended attributes](issues/010.md)** — AFF4-010; AFK; blocked by: AFF4-009; area: logical.
11. **[Read logical files backed by maps and image streams](issues/011.md)** — AFF4-011; AFK; blocked by: AFF4-004, AFF4-005, AFF4-009; area: logical.
12. **[Verify logical content and metadata integrity](issues/012.md)** — AFF4-012; AFK; blocked by: AFF4-006, AFF4-010, AFF4-011; area: integrity.
13. **[Create a logical container with ZIP-backed files](issues/013.md)** — AFF4-013; AFK; blocked by: AFF4-007, AFF4-009, AFF4-012; area: writer.
14. **[Write inline content and file substreams](issues/014.md)** — AFF4-014; AFK; blocked by: AFF4-010, AFF4-013; area: writer.
15. **[Write large logical files and shared storage](issues/015.md)** — AFF4-015; AFK; blocked by: AFF4-008, AFF4-011, AFF4-013; area: writer.
16. **[Read multipart container sets](issues/016.md)** — AFF4-016; AFK; blocked by: AFF4-011; area: multipart.
17. **[Write multipart physical and logical containers](issues/017.md)** — AFF4-017; AFK; blocked by: AFF4-015, AFF4-016; area: multipart.
18. **[Read and write imported metadata stores](issues/018.md)** — AFF4-018; AFK; blocked by: AFF4-012, AFF4-013; area: scaling.
19. **[Add optional accelerated metadata support](issues/019.md)** — AFF4-019; AFK; blocked by: AFF4-018; area: optional.
20. **[Add optional metadata signing and trust evaluation](issues/020.md)** — AFF4-020; HITL; blocked by: AFF4-012, AFF4-018; area: optional.
21. **[Harden public readers against malformed containers](issues/021.md)** — AFF4-021; AFK; blocked by: AFF4-014, AFF4-015, AFF4-016, AFF4-018; area: quality.
22. **[Ship a documented Rust release candidate](issues/022.md)** — AFF4-022; AFK; blocked by: AFF4-017, AFF4-018, AFF4-021; area: release.
23. **[Demonstrate a generated binding for another language](issues/023.md)** — AFF4-023; HITL; blocked by: AFF4-022; area: optional.

24. **[Add Python bindings after the Rust core is ready](issues/024.md)** — AFF4-024; AFK; blocked by: AFF4-022; area: bindings; deferred.

25. **[Restructure the workspace into library and executable packages per the Cargo book layout](issues/025.md)** — AFF4-025; AFK; blocked by: —; area: foundation.

## Milestones

1. Reader preview: 001–006 and 009–012.
2. Single-volume writing: 007–008 and 013–015.
3. Complete mandatory-profile candidate: 016–018 and 021–022.
4. Optional Rust extensions: 019–020.
5. Deferred language bindings, after the Rust core is ready: 024 (Python) and 023 (another language).

Reader and writer support are tracked separately. An optional feature cannot block the mandatory-profile release unless baseline review finds it mandatory. Missing canonical evidence remains visible.

## Publication status

The user published the original 23 issues; their URLs are recorded in github-issues.json. The Rust-first revisions and new local issue AFF4-024 have not been synced to GitHub because this session cannot reach its API.
