---
name: "architecture-overview"
description: "Layer definitions, allowed/prohibited call directions, service responsibilities"
---
# Architecture Overview

## Layer Definitions

| Layer | Responsibilities | Allowed Calls | Prohibited Calls |
|-------|-----------------|---------------|-----------------|
| SDK_API (`crates/sdk/`) | User-facing Prover trait, proof modes, execute/prove/verify API | sp1-prover, sp1-core-executor, sp1-core-machine, sp1-hypercube, sp1-primitives | Must not directly call slop cryptographic primitives |
| PROVER_ORCHESTRATION (`crates/prover/`) | Proving pipeline orchestration, recursion program compilation, worker task management | sp1-hypercube, sp1-core-machine, sp1-core-executor, sp1-recursion-circuit, sp1-recursion-compiler | Must not call SDK layer |
| HYPERCUBE_STARK (`crates/hypercube/`) | Per-shard STARK proving, AIR constraint system, LogUp-GKR, zerocheck sumcheck | slop-air, slop-algebra, slop-jagged, slop-multilinear, slop-sumcheck, slop-challenger, slop-commit, slop-stacked | Must not call prover orchestration or SDK layers |
| CORE_EXECUTION (`crates/core/`) | RISC-V VM execution, trace generation, instruction/syscall events, machine AIR definitions | sp1-hypercube (air types), sp1-jit, sp1-primitives | Must not call recursion or SDK layers |
| RECURSION_CIRCUIT (`crates/recursion/`) | Recursion circuit DSL, recursive verifiers, recursion machine chips | sp1-hypercube, sp1-core-machine, sp1-recursion-executor, slop-algebra, slop-challenger, slop-bn254 | Must not call SDK or prover orchestration layers |
| SLOP_CRYPTO (`slop/crates/`) | Low-level cryptographic primitives: fields, polynomials, PCS, hash, commitment | Re-exports p3-* (Plonky3); provides algebraic primitives | Must not call any sp1 crate |

## Service Responsibilities

| Module | Responsibility | NOT Responsible For |
|--------|---------------|---------------------|
| `sp1-sdk` | User-facing API; exposes `Prover` trait with five variants (CpuProver, CudaProver, NetworkProver, MockProver, LightProver); routes setup/prove/verify/execute | Does not implement cryptographic proving logic |
| `sp1-prover` | Central prover orchestration; `SP1ProverComponents` trait; recursion program builders (normalize, compose, shrink, wrap); `RecursionVks` merkle tree management | Does not define AIR constraint systems; does not execute RISC-V VM |
| `sp1-prover::SP1Controller` | Task-level orchestration: artifact download, CoreExecute task, compress/shrink/wrap pipeline via WorkerClient | Does not prove individual shards; delegates to worker tasks |
| `sp1-hypercube::ShardProver` | Per-shard STARK proofs: trace gen, PCS commit, LogUp-GKR, zerocheck sumcheck, evaluation proof | Does not handle multi-shard coordination or recursion |
| `sp1-hypercube::MachineVerifier` | Verifies `MachineProof` (shard proof collection) by iterating `ShardVerifier::verify_shard` | Does not generate proofs; does not know about recursion |
| `sp1-core-executor::CoreVM` | Cycle-accurate RISC-V VM (64-bit); executes ALU, branch, jump, load/store, ecall; supports SupervisorMode and UserMode | Does not generate AIR traces directly; does not do ZK proving |
| `sp1-core-machine` | AIR constraint columns (`TrustMode` trait, `RiscvAir` chips); instruction column types | Does not execute VM; does not perform PCS or sumcheck |
| `sp1-recursion-circuit` | Circuit configs (inner/outer DSL); recursive verifiers (`SP1RecursiveVerifier`, `SP1CompressWithVKeyVerifier`, `SP1DeferredVerifier`) | Does not execute recursion programs; does not build witness values |
| `sp1-recursion-machine::RecursionAir` | Recursion chip enum; `compress_machine()`, `shrink_machine()`, `wrap_machine()` factory methods | Does not handle outer Groth16/PLONK wrapping |
| `sp1-recursion-compiler` | DSL IR and circuit compiler (`AsmCompiler`); `Builder<C>`, `DslIrProgram` | Does not prove or verify; only compiles DSL to RecursionProgram |
| `sp1-verifier` | no_std compatible; `Groth16Verifier`, `PlonkVerifier`; embeds compiled vk artifacts | Only handles terminal Groth16/PLONK verification |
| `sp1-build` | Build-script helper; compiles programs to RISC-V ELF targeting `riscv64im-succinct-zkvm-elf`; `include_elf!` macro | Purely build-time utility; does not run proofs |
| `sp1-cuda` | Client-server wrapper for GPU proving via reqwest (JSON/rustls-tls) | Does not implement GPU kernels; only IPC to GPU server |
| `slop-air` | Re-export of `p3_air`; base AIR builder traits | Does not implement specific AIRs |
| `slop-commit` | Re-export of `p3_commit` plus custom `Rounds` and message types | Does not implement specific PCS |
| `sp1-sdk::MockProver` | Executes program only (no ZK); `create_mock_proof`; verify checks only public input hashes | Does not generate real cryptographic proofs |
| `sp1-sdk::LightProver` | Execute and verify only; `prove()` always returns error; uses full cryptographic verify | Does not generate any proofs |
| `sp1-sdk::NetworkProver` | Submits proof requests to remote Succinct network over gRPC; supports Mainnet/Reserved modes | Does not run local proving |
