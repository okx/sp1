---
name: "recursion-pipeline"
description: "Core flow: recursion pipeline — from core shard proofs through compress/shrink/wrap to Groth16/PLONK"
---
# Recursion Pipeline Flow

## Entry Point

Triggered by `SP1Controller` after core proving completes, when proof mode is Compressed, Plonk, or Groth16.

## Primary Entities

`ShardProof`, `SP1CircuitWitness`, `RecursionAir`, `SP1RecursionProof`, `Groth16Bn254Proof`, `PlonkBn254Proof`

## State Transitions

| Current State | Trigger | Target State |
|--------------|---------|-------------|
| Core proofs ready | Controller submits compress tasks | Normalizing |
| Normalizing | normalize_program processes each shard proof | Composing |
| Composing | compose_program merges normalized proofs pairwise | Compress Complete |
| Compress Complete | `compress_complete_tx` signal | Shrinking |
| Shrinking | shrink_program reduces proof size | Wrapping |
| Wrapping | wrap_program converts to outer context | Gnark Proving |
| Gnark Proving | Groth16/PLONK BN254 proof generated | Complete |

**[Rule] Terminal states must never be reversed**: Complete

## Normal Flow Steps

| Step | Action | Module |
|------|--------|--------|
| 1 | Controller receives core shard proofs and deferred proofs | `sp1-prover` |
| 2 | Build normalize recursion program (verifies one shard proof in recursion circuit) | `sp1-prover::recursion` |
| 3 | Compile normalize program via `AsmCompiler` | `sp1-recursion-compiler` |
| 4 | Prove normalize program using `compress_machine()` chips | `sp1-recursion-machine` |
| 5 | Build compose recursion program (merges pairs of normalized proofs) | `sp1-prover::recursion` |
| 6 | Iteratively compose until single compressed proof remains | `sp1-prover` |
| 7 | Wait for compress_complete signal via channel | `sp1-prover::controller` |
| 8 | Build shrink program (reduces field size for outer circuit) using `shrink_machine()` | `sp1-prover::recursion` |
| 9 | Build wrap program using `wrap_machine()` and `SP1OuterGlobalContext` | `sp1-prover::recursion` |
| 10 | If Plonk: invoke `PlonkBn254Prover` via Gnark FFI | `sp1-recursion-gnark-ffi` |
| 11 | If Groth16: invoke `Groth16Bn254Prover` via Gnark FFI | `sp1-recursion-gnark-ffi` |

## Exception Branches

| Trigger | State Change | Compensation |
|---------|-------------|-------------|
| Recursion proof verification fails | Any stage -> Failed | Error propagated as `SP1VerificationError::Recursion` |
| Gnark FFI failure | Wrapping -> Failed | Error from Docker/native bridge |
| Mismatch between SP1GlobalContext and SP1OuterGlobalContext | Wrapping -> Failed | Must use correct context per stage |

## Flow-Specific Pitfalls

[Pitfall] `wrap_machine()` uses different chip set than `compress_machine()`/`shrink_machine()` — swapping machines between stages produces invalid circuits

[Pitfall] Shrink/wrap tasks gated by `compress_complete_rx.await` — submitting early causes deadlock

[Pitfall] `SP1OuterGlobalContext` must be used for wrap stage — `SP1GlobalContext` is for compress/shrink only

[Pitfall] Recursion programs compiled with `new_unchecked` — circuit well-formedness is asserted by construction, not validated at compile time
