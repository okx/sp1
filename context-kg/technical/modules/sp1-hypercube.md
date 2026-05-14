---
name: "sp1-hypercube"
description: "Module design for sp1-hypercube: per-shard STARK proving with LogUp-GKR and zerocheck"
---
# sp1-hypercube Module

## Responsibilities

- Define AIR constraint system: `MachineAir` trait, `AirInteraction`, `InteractionKind`, `InteractionScope`
- Implement `Chip` wrapper with resolved interactions and constraint count enforcement
- Manage `Machine` (collection of Chips) and `MachineShape` (chip clustering)
- Implement `ShardProver`: trace gen -> PCS commit -> LogUp-GKR -> zerocheck sumcheck -> eval proof
- Implement `MachineVerifier` and `ShardVerifier` for shard proof verification
- Define `SepticCurve`, `SepticExtension`, `SepticDigest` for global interaction accumulation
- Define `PublicValues` struct for cross-shard state
- Provide IR compiler for constraint expressions

## NOT Responsible For

- Multi-shard coordination or recursion (that is sp1-prover)
- RISC-V instruction set definition (that is sp1-core-machine)
- VM execution (that is sp1-core-executor)

## Core Entities

| Entity | Key Fields | Description |
|--------|-----------|-------------|
| `Chip<F, A>` | air, sends, receives, constraint_count | Wraps MachineAir with interactions |
| `Machine<F, A>` | chips, shape, num_public_values | Collection of chips |
| `ShardProver` | (trait methods) | Per-shard STARK proof generation |
| `ShardVerifier` | (trait methods) | Per-shard verification |
| `MachineVerifier` | verifier | Iterates shard proofs |
| `SepticDigest` | SepticCurve point | Global cumulative sum |
| `PublicValues` | timestamps, pc, digests, exit_code | Cross-shard state |

## Dependencies

- Require to reference arch/dependency.md for full dependency details

## Relevant Flows

- Require to reference core-flows/ for flows involving this module

## Module-Specific Pitfalls

[Pitfall] `MAX_CONSTRAINT_DEGREE` is 3 — adding constraints of degree > 3 causes assertion failure in `Chip::new()`

[Pitfall] Preprocessed traces must not be empty when committing — runtime assertion enforced

[Pitfall] `SepticDigest::zero()` is NOT the curve identity but `CURVE_CUMULATIVE_SUM_START` — using actual zero point breaks Weierstrass addition

[Pitfall] `sum_as_poly` currently only supports degree-3 constraint polynomials — degree-2 flexibility is TODO
