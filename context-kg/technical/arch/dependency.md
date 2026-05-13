---
name: "dependency"
description: "Upstream callers, inter-module deps, storage/middleware, external services"
---
# Dependency Map

## Upstream Callers

| Caller | Protocol | Entry Point |
|--------|----------|------------|
| NetworkClient (sp1-sdk) | gRPC | `ProverNetworkClient` (auction/base variants) via tonic for proof requests, status, nonce, balance |
| NetworkClient (sp1-sdk) | HTTP | Uploads/downloads artifacts via presigned URLs using reqwest |
| sp1-cli (cargo-prove) | HTTP | Downloads toolchain/release artifacts from external URLs |
| sp1-cuda | HTTP | Communicates with local GPU proving server via reqwest (JSON, rustls-tls) |

## Inter-Module Dependencies

| From | To | Mechanism |
|------|----|-----------|
| `sp1-sdk` | `sp1-prover` | in-process; core proving logic |
| `sp1-sdk` | `sp1-verifier` | in-process; proof verification |
| `sp1-sdk` | `sp1-core-executor` | in-process; RISC-V execution |
| `sp1-sdk` | `sp1-hypercube` | in-process; hypercube proof system |
| `sp1-sdk` | `sp1-primitives` | in-process; core primitive types |
| `sp1-sdk` | `sp1-cuda` | in-process; optional GPU (feature=cuda) |
| `sp1-sdk` | `sp1-recursion-gnark-ffi` | in-process; optional Gnark FFI (feature=native-gnark) |
| `sp1-prover` | `sp1-recursion-compiler` | in-process; recursion circuit compilation |
| `sp1-prover` | `sp1-recursion-circuit` | in-process; recursion circuit definitions |
| `sp1-prover` | `sp1-recursion-machine` | in-process; recursion machine |
| `sp1-prover` | `sp1-recursion-gnark-ffi` | in-process; Gnark FFI for Groth16/PLONK |
| `sp1-prover` | `sp1-hypercube` | in-process; shard proving |
| `sp1-prover` | `sp1-core-executor` | in-process; RISC-V execution |
| `sp1-prover` | `slop-basefold` | in-process; BaseFold PCS |
| `sp1-prover` | `slop-jagged` | in-process; Jagged PCS |
| `sp1-prover` | `slop-stacked` | in-process; Stacked PCS |
| `sp1-hypercube` | `slop-algebra` | in-process; field algebra |
| `sp1-hypercube` | `slop-basefold` / `slop-basefold-prover` | in-process; BaseFold PCS |
| `sp1-hypercube` | `slop-sumcheck` | in-process; sumcheck protocol |
| `sp1-hypercube` | `slop-jagged` | in-process; Jagged PCS |
| `sp1-hypercube` | `slop-merkle-tree` | in-process; Merkle tree TCS |
| `sp1-hypercube` | `slop-multilinear` | in-process; MLE polynomials |
| `sp1-hypercube` | `slop-tensor` | in-process; tensor operations |
| `sp1-hypercube` | `slop-whir` | in-process; WHIR PCS |
| `sp1-hypercube` | `slop-stacked` | in-process; Stacked PCS |
| `sp1-core-executor` | `sp1-primitives` | in-process; field types |
| `sp1-core-executor` | `sp1-curves` | in-process; elliptic curve precompiles |
| `sp1-core-executor` | `sp1-jit` | in-process; JIT compilation |
| `sp1-core-machine` | `sp1-core-executor` | in-process; execution types |
| `sp1-core-machine` | `sp1-hypercube` | in-process; AIR types |
| `sp1-recursion-circuit` | `sp1-recursion-compiler` | in-process; circuit DSL |
| `sp1-recursion-circuit` | `sp1-recursion-machine` | in-process; recursion chips |
| `sp1-recursion-circuit` | `slop-jagged` / `slop-basefold` / `slop-whir` | in-process; PCS verification |
| `sp1-primitives` | `slop-koala-bear` / `slop-bn254` / `slop-poseidon2` | in-process; field and hash |
| `slop-jagged` | `slop-multilinear` / `slop-sumcheck` / `slop-basefold` / `slop-stacked` | in-process; core crypto |
| `slop-basefold` | `slop-algebra` / `slop-multilinear` / `slop-merkle-tree` / `slop-tensor` | in-process; core crypto |
| `slop-stacked` | `slop-basefold` / `slop-basefold-prover` / `slop-merkle-tree` / `slop-multilinear` | in-process; core crypto |
| `slop-algebra` | `p3-field` | in-process; upstream Plonky3 |
| `slop-commit` | `p3-commit` | in-process; upstream Plonky3 |
| `slop-air` | `p3-air` | in-process; upstream Plonky3 |

## Storage and Middleware

| Component | Type | Usage |
|-----------|------|-------|
| In-memory task channels | tokio mpsc/watch | `LocalWorkerClient` uses `BTreeMap<TaskType, channel>` for task queues and status watchers |
| Presigned URL object store | S3-compatible HTTP | `NetworkClient` uploads/downloads artifacts (ELF, stdin, proofs) via presigned URLs |
| LRU cache | in-memory | `sp1-prover` uses `lru` crate for caching |
| File system | dirs/tempfile | `sp1-sdk`, `sp1-build`, `sp1-prover`, `sp1-recursion-gnark-ffi` use dirs/tempfile for config and temporary proving artifacts |

## External Services

| Service | SDK/Client | Purpose |
|---------|-----------|---------|
| Succinct Prover Network | gRPC via tonic (`NetworkClient`) | Submit proof requests, poll status, manage programs; ECDSA-signed; nonce-based replay protection |
| Succinct Artifact Store | HTTP/S3 presigned URLs | Upload/download ELF programs, stdin, proofs; zstd level 3 compression |
| Gnark proving system | Native FFI (bindgen) or Docker | Groth16/PLONK proof wrapping over BN254 |
| GPU proving server | HTTP/local via reqwest | GPU-accelerated shard proving (sp1-cuda) |
| AWS KMS | aws-sdk-kms | Optional signing via alloy-signer-aws (feature=network) |
| Plonky3 (p3-*) | in-process, v0.3.3-succinct | Upstream fork pinned to Succinct-specific version; field, AIR, commit, merkle traits |

## Prohibited Patterns

[Rule] SDK layer must never directly call slop cryptographic primitives — go through sp1-hypercube or sp1-prover

[Rule] SLOP_CRYPTO layer must never call any sp1 crate — it is the lowest layer providing pure cryptographic primitives

[Rule] CORE_EXECUTION must never call recursion or SDK layers — it only knows about execution and trace generation

[Rule] All gRPC calls in `NetworkClient` must be wrapped in `with_retry`/`with_retry_timeout` — direct RPC calls without retry are prohibited
