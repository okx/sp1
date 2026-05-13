---
name: "sp1-prover"
description: "Module design for sp1-prover: proving pipeline orchestration and recursion program building"
---
# sp1-prover Module

## Responsibilities

- Define `SP1ProverComponents` trait (associates CoreProver, RecursionProver, WrapProver)
- Build recursion programs: normalize, compose, shrink, wrap
- Manage `RecursionVks` merkle tree for vk verification
- Orchestrate proving pipeline via `SP1Controller` (task-based)
- Coordinate worker nodes: `LocalWorkerClient` (in-process channels), remote worker tasks
- Manage proving artifacts (shapes, circuit version)

## NOT Responsible For

- Defining AIR constraint systems (that is sp1-hypercube and sp1-core-machine)
- Executing the RISC-V VM (that is sp1-core-executor)
- User-facing API (that is sp1-sdk)

## Core Entities

| Entity | Key Fields | Description |
|--------|-----------|-------------|
| `SP1ProverComponents` | CoreProver, RecursionProver, WrapProver | Trait bundling prover type params |
| `SP1Controller` | worker_client, tasks | Task-level pipeline orchestration |
| `SP1CircuitWitness` | Core/Deferred/Compress/Shrink/Wrap | Witness input per stage |
| `SP1ProofWithMetadata<P>` | proof, stdin, public_values, cycles | Internal proof bundle |
| `RecursionVks` | vk_map, merkle_root | Verifying key merkle tree |
| `TaskError` | Retryable/Fatal/Execution | Worker task error classification |

## Dependencies

- Require to reference arch/dependency.md for full dependency details

## Relevant Flows

- Require to reference core-flows/ for flows involving this module

## Module-Specific Pitfalls

[Pitfall] Shrink/wrap tasks must not be submitted until compress completes — gated by `compress_complete_rx` channel; submitting early causes hang or panic

[Pitfall] Adding a new `TaskType` variant requires updating `LocalWorkerClient::init()` — channels are pre-created for all known task types at startup

[Pitfall] `RecursionVks` can be loaded in dummy mode (vk_verification=false) for dev — never use dummy mode in production
