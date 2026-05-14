---
name: "verification-pipeline"
description: "Core flow: verification pipeline — from SDK verify call through mode-specific verification"
---
# Verification Pipeline Flow

## Entry Point

`Prover::verify(proof, vkey, status_code)` -> `verify_proof()`

## Primary Entities

`SP1ProofWithPublicValues`, `SP1VerifyingKey`, `StatusCode`, `SP1VerificationError`

## State Transitions

| Current State | Trigger | Target State |
|--------------|---------|-------------|
| Proof received | verify called | Version Check |
| Version Check | Version matches | Mode Dispatch |
| Mode Dispatch (Core) | Shard verification | Core Verify |
| Mode Dispatch (Compressed) | Recursion verification | Recursion Verify |
| Mode Dispatch (Plonk) | PLONK verification | Terminal Verify |
| Mode Dispatch (Groth16) | Groth16 verification | Terminal Verify |
| Any Verify | All checks pass | Verified |

**[Rule] Terminal states must never be reversed**: Verified, Failed

## Normal Flow Steps

| Step | Action | Module |
|------|--------|--------|
| 1 | Check `bundle.sp1_version` matches `SP1_CIRCUIT_VERSION` | `sp1-sdk::prover` |
| 2 | Check `StatusCode` (default SUCCESS=0) matches proof exit code | `sp1-sdk::prover` |
| 3 | Dispatch by proof mode: Core, Compressed, Plonk, Groth16 | `sp1-sdk::prover` |
| 4a (Core) | `MachineVerifier::verify()` iterates all shard proofs | `sp1-hypercube::verifier` |
| 4a.1 | Per shard: clone challenger, `ShardVerifier::verify_shard()` | `sp1-hypercube::verifier` |
| 4a.2 | Verify public values: committed_value_digest via dual-hash (SHA2 or Blake3) | `sp1-prover::verify` |
| 4b (Compressed) | Verify recursion proof via `MachineVerifier` on recursion machine | `sp1-prover::verify` |
| 4b.1 | Verify recursion public values consistency | `sp1-prover::verify` |
| 4c (Plonk) | `PlonkVerifier::verify()` with embedded PLONK_VK_BYTES | `sp1-verifier::plonk` |
| 4c.1 | Check 5 public inputs: vkey_hash, committed_values_digest, exit_code, vk_root, proof_nonce | `sp1-verifier::plonk` |
| 4d (Groth16) | `Groth16Verifier::verify()` with embedded GROTH16_VK_BYTES | `sp1-verifier::groth16` |
| 4d.1 | Check 5 public inputs (same layout as PLONK) | `sp1-verifier::groth16` |
| 5 | Return `Ok(())` or `Err(SP1VerificationError)` | `sp1-sdk::prover` |

## Exception Branches

| Trigger | State Change | Compensation |
|---------|-------------|-------------|
| Version mismatch | Version Check -> Failed | `SP1VerificationError::VersionMismatch` |
| Public values hash mismatch (both SHA2 and Blake3 fail) | Any Verify -> Failed | `SP1VerificationError::InvalidPublicValues` |
| Shard proof invalid | Core Verify -> Failed | `SP1VerificationError::Core` |
| Recursion proof invalid | Recursion Verify -> Failed | `SP1VerificationError::Recursion` |
| BN254 proof invalid | Terminal Verify -> Failed | `SP1VerificationError::Plonk` or `::Groth16` |
| Unexpected exit code | Status Check -> Failed | `SP1VerificationError::UnexpectedExitCode` |

## Flow-Specific Pitfalls

[Pitfall] Dual-hash tolerance: SP1 V4 accepts either SHA2 or Blake3 for public values — both are checked; neither alone is sufficient to reject

[Pitfall] `MockProver` overrides `verify()` to skip cryptographic checks — never rely on MockProver verification in production

[Pitfall] PLONK/Groth16 public inputs layout is fixed: [0]=vkey_hash, [1]=committed_values_digest, [2]=exit_code, [3]=vk_root, [4]=proof_nonce — reordering breaks verification
