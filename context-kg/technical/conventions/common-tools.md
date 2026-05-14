---
name: "common-tools"
description: "Must-reuse components, shared utilities, and common tools across SP1"
---
# Common Tools

## Cryptographic Primitives (slop layer)

[Reuse] `slop-algebra` — Field algebra operations; re-exports `p3-field`. All field arithmetic must go through this crate.

[Reuse] `slop-multilinear::Mle` — Batch multilinear polynomial representation. Use for all MLE operations instead of implementing custom polynomial types.

[Reuse] `slop-multilinear::Point` — Evaluation point for multilinear polynomials. Use `Point::from_usize()` for Boolean hypercube points.

[Reuse] `slop-sumcheck` — Sumcheck protocol implementation. Use for all sumcheck-related proofs.

[Reuse] `slop-tensor::Tensor` — Generic multi-dimensional tensor. Use for all batch data operations instead of raw Vec<Vec<>>.

[Reuse] `slop-challenger` — Fiat-Shamir challenger and `IopCtx` trait. Use for all interactive-to-non-interactive protocol transformations.

[Reuse] `slop-merkle-tree::MerkleTreeTcs` — Poseidon2-based tensor commitment scheme. Use for all Merkle tree commitments.

[Reuse] `slop-maybe-rayon` — Optional parallelism toggle. Use instead of direct rayon dependency for parallel iteration.

[Reuse] `slop-futures` — Async utilities including pipeline and rayon integration. Use for async proving pipelines.

[Reuse] `slop-alloc` — Custom allocator and buffer management. Use for performance-critical memory allocation.

## SP1 Shared Types

[Reuse] `sp1-primitives::SP1Field` / `SP1ExtensionField` — Type aliases for KoalaBear and its degree-4 extension. Always use these aliases, not the underlying types directly.

[Reuse] `sp1-primitives::poseidon2_hash` / `POSEIDON2_HASHER` — Poseidon2 hash function singleton. Use for all Poseidon2 hashing.

[Reuse] `sp1-primitives::SP1GlobalContext` / `SP1OuterGlobalContext` — IopCtx type aliases. Use inner for core/compress/shrink, outer for wrap.

[Reuse] `sp1-hypercube::SepticDigest` — Global cumulative sum digest. Use for all global interaction accumulation.

[Reuse] `sp1-hypercube::Chip::new()` — Chip constructor with constraint degree validation. Always use this to create chips.

## SDK Shared Types

[Reuse] `sp1-sdk::SP1ProofWithPublicValues` — User-facing proof bundle. Use for all proof serialization/deserialization.

[Reuse] `sp1-core-machine::io::SP1Stdin` — Input stream. Use for all guest program input.

[Reuse] `sp1-primitives::SP1PublicValues` — Output values with dual-hash. Use for all public value handling.
