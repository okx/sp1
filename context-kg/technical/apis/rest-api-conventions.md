---
name: "rest-api-conventions"
description: "SDK API conventions, proof types, builder patterns, and prover trait interface"
---
# API Conventions

## SDK Prover Trait Interface

SP1 is a library/SDK, not a REST service. The primary API is the `Prover` trait in `crates/sdk/src/prover.rs`:

| Method | Signature | Description |
|--------|-----------|-------------|
| `setup` | `fn setup(&self, elf: Elf) -> Future<ProvingKey>` | Compile ELF into proving/verifying keys |
| `prove` | `fn prove(&self, pk: &PK, stdin: SP1Stdin) -> ProveRequest` | Build and submit prove request (builder pattern) |
| `execute` | `fn execute(&self, elf: Elf, stdin: SP1Stdin) -> ExecuteRequest` | Execute without proving (builder pattern) |
| `verify` | `fn verify(&self, proof: &SP1ProofWithPublicValues, vkey: &SP1VerifyingKey, status_code: Option<StatusCode>) -> Result<(), SP1VerificationError>` | Verify a proof |

## Proof Types

| Type | Description | Fields |
|------|-------------|--------|
| `SP1ProofWithPublicValues` | User-facing proof bundle | `proof: SP1Proof`, `public_values: SP1PublicValues`, `sp1_version: String`, `tee_proof: Option<Vec<u8>>` |
| `SP1Proof::Core` | Uncompressed shard proofs | `Vec<ShardProof<SP1GlobalContext, ...>>` |
| `SP1Proof::Compressed` | Single recursion proof | `SP1RecursionProof` |
| `SP1Proof::Plonk` | PLONK BN254 proof | `PlonkBn254Proof` |
| `SP1Proof::Groth16` | Groth16 BN254 proof | `Groth16Bn254Proof` |

## Builder Pattern

Prove and execute requests use the builder pattern:
- `prover.prove(pk, stdin).mode(SP1ProofMode::Groth16).run().await`
- `prover.execute(elf, stdin).max_cycles(1_000_000).run().await`

## Prover Variants

| Variant | Proves | Verifies | Use Case |
|---------|--------|----------|----------|
| `CpuProver` | Yes (CPU) | Yes (full) | Local development and testing |
| `CudaProver` | Yes (GPU) | Yes (full) | High-performance local proving |
| `NetworkProver` | Yes (remote) | Yes (full) | Production proving via Succinct network |
| `MockProver` | No (fake) | Hash-only | Unit testing |
| `LightProver` | No (error) | Yes (full) | Verification-only nodes |

## Versioning

| Strategy | Pattern | Breaking Change Rule |
|----------|---------|---------------------|
| Embedded version string | `SP1_CIRCUIT_VERSION` const loaded from `SP1_CIRCUIT_VERSION` file | Must be bumped when any verification step changes; checked before proof validation |
| Workspace version | `6.1.0` in all `Cargo.toml` | All crates must use same version |
