---
name: "feature-types"
description: "Base patterns, builder patterns, and feature type conventions in SP1"
---
# Feature Types and Base Patterns

## Builder Pattern

[Convention] **Prove/Execute builder**: SDK uses builder pattern for prove (`ProveRequest`) and execute (`ExecuteRequest`) requests. Chain configuration methods before `.run().await`.
- `prover.prove(pk, stdin).mode(SP1ProofMode::Groth16).run().await`
- `prover.execute(elf, stdin).max_cycles(1_000_000).run().await`

## Prover Trait Pattern

[Convention] **Prover variants via trait**: All prover backends implement the `Prover` trait. New backends must implement `setup`, `prove`, `execute`, and optionally override `verify`.
- Associated types: `ProvingKey`, `Error`, `ProveRequest`
- `inner()` returns `&SP1NodeCore` for delegation

## Machine/Chip Pattern

[Convention] **Machine as chip collection**: A `Machine<F, A>` is a collection of `Chip<F, A>` instances with a `MachineShape`. Chips are created by wrapping `MachineAir` implementations via `Chip::new()`.
- Shape selection via `MachineShape::smallest_cluster()`
- Interactions resolved at chip construction time

## TrustMode Pattern

[Convention] **Static trust dispatch**: `TrustMode` trait with `SupervisorMode` and `UserMode` implementations enables compile-time selection of trusted vs. untrusted execution paths. Each mode provides distinct column types.

## Recursion Program Pattern

[Convention] **DSL -> Compile -> Prove**: Recursion programs are built with `Builder<C>` (DSL), compiled with `AsmCompiler`, and proven using the appropriate recursion machine (compress/shrink/wrap).

## IopCtx Pattern

[Convention] **Type bundle for IOP**: `IopCtx` trait bundles field F, extension field EF, Challenger, Hasher, Compressor, and Digest types. Two concrete instances:
- `SP1GlobalContext` (KoalaBearDegree4Duplex) for inner stages
- `SP1OuterGlobalContext` (BNGC) for outer/wrap stages

## Feature Flag Pattern

[Convention] **Cascading feature flags**: Features like `mprotect`, `cuda`, `network`, `native-gnark` must propagate through the entire dependency chain. Check Cargo.toml feature gates when adding new optional functionality.

## Pagination / Streaming Pattern

Not applicable — SP1 is a proving system, not a web service. Data is streamed via `SP1Stdin`/`SP1PublicValues` I/O primitives.
