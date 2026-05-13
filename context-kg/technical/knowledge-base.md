---
name: "knowledge-base"
description: "Highest-authority rules — all Skills defer to this on conflicts"
---
# Knowledge Base

> This is the highest-weight file in the knowledge base. All Skills that support
> context-kg defer to this file when conflicts arise with general AI knowledge.

## Data Type Constraints

[Rule] `crates/hypercube/src/chip.rs`: `MAX_CONSTRAINT_DEGREE` must always be 3 — all AIR polynomial constraints must have degree <= 3. — Reason: enforced by assertion in `Chip::new()`

[Rule] `crates/primitives/src/lib.rs`: `SP1Field` must always be `KoalaBear` (prime order 2130706433) — Reason: field choice is baked into circuit constants, Poseidon2 round constants, and recursion programs

[Rule] `crates/primitives/src/lib.rs`: `SP1ExtensionField` must always be `BinomialExtensionField<KoalaBear, 4>` (degree-4 extension, polynomial x^4 - 3) — Reason: challenge field used throughout sumcheck and LogUp-GKR protocols

[Rule] `crates/hypercube/src/septic_extension.rs`: `SepticExtension` must always be degree-7 extension F_p[z]/(z^7 - 3z - 5) — Reason: used for `SepticCurve` global interaction accumulation; changing the irreducible polynomial breaks all existing proofs

[Rule] `crates/hypercube/src/septic_curve.rs`: `SepticCurve` equation must be y^2 = x^3 + 45x + 41z^3 over F_{p^7} — Reason: curve parameters are hardcoded and used in `CURVE_CUMULATIVE_SUM_START`, `DIGEST_SUM_START`, `CURVE_WITNESS_DUMMY_POINT`

[Rule] `crates/core/executor/src/lib.rs`: `CLK_INC` must be 8 and `PC_INC` must be 4 — Reason: clock and PC increments are embedded in AIR constraints and memory timestamp ordering

[Rule] `crates/core/executor/src/lib.rs`: `BYTE_NUM_ROWS` must be 1 << 16 (65536) and `RANGE_NUM_ROWS` must be 1 << 17 (131072) — Reason: lookup table sizes are fixed in AIR chip dimensions

## Naming Constraints

[Rule] `crates/prover/src/types.rs`: Proving stage names must follow the pipeline: Core / Deferred / Compress / Shrink / Wrap — Reason: `SP1CircuitWitness` enum variants and recursion program builders are named accordingly

[Rule] `crates/sdk/src/prover.rs`: Proof mode names must be Core / Compressed / Plonk / Groth16 — Reason: `SP1ProofMode` discriminant enum is used in serialization and network protocol

[Rule] `crates/build/src/lib.rs`: zkVM ELF target must always be `riscv64im-succinct-zkvm-elf` — Reason: executor, JIT compiler, and constraint system all expect this exact target triple

[Rule] `Cargo.toml`: All workspace crate versions must always be `6.1.0` matching `workspace.package.version` — Reason: version is used in circuit version checking and proof compatibility

## Dependency Constraints

[Rule] `Cargo.toml`: All `p3-*` (Plonky3) crates must be pinned to version `0.3.3-succinct` (Succinct fork) — Reason: upstream Plonky3 is incompatible; the fork includes SP1-specific modifications

[Rule] `slop/crates/air/src/lib.rs`: `slop-air` must re-export `p3_air` without adding custom base AIR traits — Reason: all AIR builder traits derive from Plonky3's trait hierarchy

[Rule] `slop/crates/commit/src/lib.rs`: `slop-commit` must re-export `p3_commit` without overriding PCS traits — Reason: PCS commitment interface is defined by Plonky3

[Rule] `crates/sdk/Cargo.toml`: `mprotect` feature must propagate through `sp1-core-executor` → `sp1-core-executor-runner` → `sp1-core-machine` → `sp1-hypercube` → `sp1-prover` → `sp1-recursion-executor` — Reason: page protection columns and constraints must be consistently present or absent across the entire proving pipeline

[Rule] `crates/recursion/gnark-ffi/Cargo.toml`: `bindgen` must only be a build-dependency activated under `feature=native` — Reason: Docker-based Gnark invocation is the default; native FFI requires separate Go toolchain setup

[Rule] `crates/sdk/Cargo.toml`: `network` feature must be enabled for gRPC/HTTP network proving — Reason: without it, `NetworkProver` is unavailable

[Rule] `crates/prover/src/worker/client/local.rs`: Adding a new `TaskType` variant requires updating `LocalWorkerClient::init()` channel initialization — Reason: channels are pre-created for all known task types at startup

## Security Constraints

[Rule] `crates/sdk/src/prover.rs`: Proof verification must always check `SP1_CIRCUIT_VERSION` before any proof-type-specific validation — Reason: prevents accepting proofs from incompatible circuit versions

[Rule] `crates/sdk/src/prover.rs`: Public values verification must accept both SHA2 and Blake3 hashes (dual-hash tolerance) — Reason: SP1 V4 does not embed which hash function was used; security derives from collision resistance of both algorithms

[Rule] `crates/sdk/src/mock/mod.rs`: `MockProver` must never perform full cryptographic verification — Reason: it is designed for testing only; overrides `verify()` with hash-only checks

[Rule] `crates/sdk/src/light/mod.rs`: `LightProver` must never override `Prover::verify()` — Reason: it uses full cryptographic verification unlike MockProver; this is intentional for read-only verification nodes

[Rule] `crates/sdk/src/network/client.rs`: All network proof requests must be signed via `NetworkSigner` before transmission — Reason: unsigned requests are rejected by the Succinct prover network

[Rule] `crates/sdk/src/network/grpc.rs`: TLS must always be configured when `rpc_url` starts with `https://` — Reason: prevents man-in-the-middle attacks on proof submission

[Rule] `crates/sdk/src/network/client.rs`: Artifact content must be bincode-serialized and zstd-compressed (level 3) before upload — Reason: network protocol requires this encoding; uncompressed uploads are rejected

[Rule] `crates/core/executor/src/vm.rs`: Register X0 must never be written with a non-zero value — Reason: RISC-V specification requires x0 to always read as zero; `rw()` enforces this
