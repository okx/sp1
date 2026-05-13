---
name: "sp1-verifier"
description: "Module design for sp1-verifier: no_std Groth16 and PLONK proof verification"
---
# sp1-verifier Module

## Responsibilities

- Provide `Groth16Verifier` and `PlonkVerifier` for terminal proof verification
- Embed compiled verifying key artifacts via `include_bytes!` and `lazy_static`
- Support `no_std` environments for on-chain verification
- Provide `VerifierRecursionVks` with merkle root
- Define proof types: `SP1Proof`, `PlonkBn254Proof`, `Groth16Bn254Proof`

## NOT Responsible For

- Core or recursion STARK proof verification (only terminal Groth16/PLONK)
- Proof generation of any kind
- SDK user interface

## Core Entities

| Entity | Key Fields | Description |
|--------|-----------|-------------|
| `SP1Proof` | Core/Compressed/Plonk/Groth16 variants | Proof mode enum |
| `Groth16Verifier` | GROTH16_VK_BYTES | Static Groth16 verification |
| `PlonkVerifier` | PLONK_VK_BYTES | Static PLONK verification |
| `verifier::Error` | InvalidWitness, InvalidProgramVkeyHash, etc. | Verification errors |

## Dependencies

- Require to reference arch/dependency.md for full dependency details

## Relevant Flows

- Require to reference core-flows/ for flows involving this module

## Module-Specific Pitfalls

[Pitfall] Verifying keys are embedded as compile-time byte arrays — updating vkeys requires rebuilding the crate

[Caution] After finalizing vkeys, vkey verification solution needs cleanup (TODO in `compressed/mod.rs`)
