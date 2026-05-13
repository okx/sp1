---
name: "slop-crypto"
description: "Module design for slop cryptographic primitives: fields, polynomials, PCS, hash, commitment"
---
# slop-crypto Module

Collective name for all `slop/crates/*` — low-level cryptographic primitives.

## Responsibilities

- Field arithmetic: `slop-algebra` (re-exports p3-field), `slop-baby-bear`, `slop-koala-bear`, `slop-bn254`
- Polynomial operations: `slop-multilinear` (Mle, Point, PaddedMle), `slop-sumcheck` (sumcheck protocol)
- Polynomial commitment schemes: `slop-basefold` (FRI-based), `slop-stacked` (Ligero-style), `slop-jagged` (variable-size), `slop-whir`
- Merkle tree and tensor commitment: `slop-merkle-tree` (MerkleTreeTcs, TensorCsProver)
- Hash functions: `slop-poseidon2`, `slop-symmetric`
- AIR/commit interfaces: `slop-air` (re-exports p3-air), `slop-commit` (re-exports p3-commit)
- Data structures: `slop-tensor`, `slop-matrix`
- Utilities: `slop-alloc`, `slop-maybe-rayon`, `slop-futures`, `slop-utils`
- Advanced protocols: `slop-spartan` (R1CS), `slop-veil`, `slop-pgspcs`

## NOT Responsible For

- Any SP1-specific logic (RISC-V, recursion, SDK)
- Must not call any `sp1-*` crate

## Core Entities

| Entity | Key Fields | Description |
|--------|-----------|-------------|
| `Mle<T, A>` | Tensor (rows=evals, cols=polys) | Batch of multilinear polynomials |
| `Point<T, A>` | Vec of field elements | n-dimensional evaluation point |
| `MleEval<T, A>` | Tensor | Batch evaluation result |
| `TensorCsProver` | commit_tensors, prove_openings | Tensor commitment scheme trait |
| `MerkleTreeTcs` | hasher, compressor | Poseidon2-based Merkle tree TCS |
| `PartialSumcheckProof<K>` | univariate_polys, claimed_sum, point_and_eval | Sumcheck proof without eval proofs |
| `IopCtx` | F, EF, Challenger, Hasher, Compressor, Digest | Interactive oracle proof context bundle |

## Dependencies

- Require to reference arch/dependency.md for full dependency details

## Relevant Flows

- Require to reference core-flows/ for flows involving this module

## Module-Specific Pitfalls

[Pitfall] `slop-spartan` R1CS lacks proper error handling — TODO noted in `r1cs.rs`

[Pitfall] `slop-spartan` sparse_matrix parallelization is missing — TODO noted in `sparse_matrix.rs`

[Pitfall] `slop-pgspcs` batching not implemented — TODO in `prover.rs`

[Pitfall] `slop-multilinear` restrict operations could be optimized with pre-cached partial Lagrange evals — TODO in `restrict.rs`
