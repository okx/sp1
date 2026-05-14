---
name: "sp1-sdk"
description: "Module design for sp1-sdk: user-facing API with five prover variants"
---
# sp1-sdk Module

## Responsibilities

- Provide the `Prover` trait as the unified API for proving and verification
- Implement five prover variants: `CpuProver`, `CudaProver`, `NetworkProver`, `MockProver`, `LightProver`
- Provide builder patterns for prove (`ProveRequest`) and execute (`ExecuteRequest`) operations
- Handle proof serialization/deserialization (`SP1ProofWithPublicValues`)
- Manage network communication with Succinct prover network (gRPC + HTTP artifact store)
- Verify proofs with dual-hash tolerance (SHA2/Blake3)

## NOT Responsible For

- Implementing cryptographic proving logic (delegates to `sp1-prover`)
- Defining AIR constraints or chip circuits
- RISC-V VM execution internals (delegates to `sp1-core-executor`)

## Core Entities

| Entity | Key Fields | Description |
|--------|-----------|-------------|
| `SP1ProofWithPublicValues` | proof, public_values, sp1_version, tee_proof | User-facing proof bundle |
| `SP1Stdin` | (re-exported from sp1-core-machine) | Input stream for zkVM program |
| `SP1PublicValues` | (re-exported from sp1-primitives) | Output/committed values |
| `SP1ProvingKey` | vk, elf | Proving key holding VK and ELF |
| `SP1VerifyingKey` | (re-exported from sp1-hypercube) | Verifying key |
| `SP1VerificationError` | 8 variants | Verification error enum |
| `StatusCode` | u32 | Exit code wrapper (SUCCESS=0) |

## Dependencies

- Require to reference arch/dependency.md for full dependency details

## Relevant Flows

- Require to reference core-flows/ for flows involving this module

## Module-Specific Pitfalls

[Pitfall] `MockProver` silently accepts any Core/Compressed proof without cryptographic verification — only use for testing; never use MockProver in production verification paths

[Pitfall] `LightProver::prove()` always returns an error — it is verify-only; do not attempt to generate proofs with it

[Pitfall] Version mismatch between proof's `sp1_version` and local `SP1_CIRCUIT_VERSION` causes verification failure — ensure prover and verifier use the same SP1 version
