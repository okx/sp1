---
name: "prove-pipeline"
description: "Core flow: prove pipeline — from SDK prove call through execution to shard STARK proofs"
---
# Prove Pipeline Flow

## Entry Point

`Prover::prove(pk, stdin)` -> `ProveRequest` builder -> `.mode(SP1ProofMode)` -> `.run().await`

## Primary Entities

`SP1ProvingKey`, `SP1Stdin`, `ExecutionRecord`, `ShardProof`, `SP1ProofWithPublicValues`

## State Transitions

| Current State | Trigger | Target State |
|--------------|---------|-------------|
| ProveRequest configured | `.run().await` | Executing |
| Executing | VM completes all cycles | Sharding |
| Sharding | `ExecutionRecord::defer()` splits trace | Core Proving |
| Core Proving | All shard proofs generated | Recursion (if Compressed/Plonk/Groth16) |
| Core Proving | All shard proofs generated | Complete (if Core mode) |
| Recursion complete | Gnark wrapping done | Complete |

**[Rule] Terminal states must never be reversed**: Complete

## Normal Flow Steps

| Step | Action | Module |
|------|--------|--------|
| 1 | SDK creates `ProveRequest` with builder pattern, sets mode | `sp1-sdk` |
| 2 | `SP1Controller::run()` downloads artifacts, submits `CoreExecute` task | `sp1-prover` |
| 3 | Worker executes RISC-V program via `CoreVM`, produces `ExecutionRecord` per shard | `sp1-core-executor` |
| 4 | Records are split: precompile/memory events deferred to separate shards | `sp1-core-executor` |
| 5 | `Machine::generate_dependencies()` resolves inter-chip dependencies across records | `sp1-hypercube` |
| 6 | `ShardProver::prove_shard_with_pk()` for each shard: trace gen (async) then blocking proof | `sp1-hypercube` |
| 7 | Per-shard proof: commit traces -> LogUp-GKR -> zerocheck sumcheck -> PCS eval proof | `sp1-hypercube` |
| 8 | All shard proofs collected as `SP1Proof::Core(Vec<ShardProof>)` | `sp1-prover` |
| 9 | If mode != Core: enter recursion pipeline (compress -> shrink -> wrap) | `sp1-prover` |
| 10 | Bundle into `SP1ProofWithPublicValues` with version string | `sp1-sdk` |

## Exception Branches

| Trigger | State Change | Compensation |
|---------|-------------|-------------|
| Execution error (cycle limit, invalid memory, etc.) | Executing -> Failed | Return `ExecutionError` to caller |
| Shape mismatch during proving | Core Proving -> Failed | `SP1CoreProverError` returned |
| Task error (retryable) | Any worker task -> Retrying | Auto-retry for network/transport errors |
| Task error (fatal) | Any worker task -> Failed | Propagate error to caller |

## Flow-Specific Pitfalls

[Pitfall] `ShardProver::prove_shard_with_pk` uses `tokio::task::spawn_blocking` for the blocking proof work — do not call from a synchronous context without a runtime

[Pitfall] Preprocessed traces must not be empty — assertion enforced; ensure shape selection includes all required chips

[Pitfall] Shard trace generation and proof construction order is fixed: commit_traces -> prove_logup_gkr -> zerocheck -> prove_trusted_evaluations — reordering breaks the Fiat-Shamir transcript
