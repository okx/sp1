---
name: "terminology"
description: "Domain term glossary for unified terminology across backend skills"
---
# Domain Terminology

| Term | Description | References |
|------|-------------|------------|
| AIR | Algebraic Intermediate Representation — polynomial constraint system for a chip; defined via `p3_air` traits | `slop/crates/air/`, `crates/hypercube/src/air/` |
| AirInteraction | Builder-time description of a cross-table lookup interaction with values, multiplicity, and interaction kind | `crates/hypercube/src/air/interaction.rs` |
| Basefold PCS | FRI-based polynomial commitment scheme using code folding | `slop/crates/basefold/` |
| ByteOpcode | Operations on byte values via lookup tables (AND, OR, XOR, U8Range, LTU, MSB, Range) | `crates/core/executor/src/opcode.rs` |
| Chip | Wraps a `MachineAir` with resolved send/receive Interaction vectors and constraint count; unit that Machine operates on | `crates/hypercube/src/chip.rs` |
| CLK_INC | Default clock increment = 8 per instruction cycle; syscalls add 256 additionally | `crates/core/executor/src/lib.rs` |
| CoreVM | Cycle-accurate RISC-V VM (64-bit); parameterized on `ExecutionMode` (SupervisorMode or UserMode) | `crates/core/executor/src/vm.rs` |
| CURVE_CUMULATIVE_SUM_START | Fixed starting point for `SepticDigest` accumulation, derived from sqrt(2); avoids Weierstrass exceptions at origin | `crates/hypercube/src/septic_digest.rs` |
| ExecutionRecord | Complete trace of a program's shard execution; contains typed event vectors; implements `MachineRecord` | `crates/core/executor/src/record.rs` |
| ExecutionState | Runtime state: PC, clock, memory, page protections, I/O streams, proof stream, syscall counts | `crates/core/executor/src/state.rs` |
| ForkState | Snapshot for unconstrained block rollback: global_clk, clk, pc, memory_diff, page_prots_diff | `crates/core/executor/src/state.rs` |
| GlobalInteractionEvent | Event recording a cross-shard interaction for accumulation into `SepticDigest` | `crates/core/executor/src/events/global.rs` |
| Groth16Bn254Proof | Groth16 proof over BN254 with 5 public_inputs, encoded_proof, raw_proof, groth16_vkey_hash | `crates/verifier/src/proof.rs` |
| HALT_PC | Sentinel PC value = 1 (invalid, not 4-aligned) signaling program halt | `crates/core/executor/src/lib.rs` |
| Instruction | SP1 custom-encoded RISC-V instruction: opcode (u8), op_a (u8), op_b (u64), op_c (u64), imm_b, imm_c | `crates/core/executor/src/instruction.rs` |
| Interaction | Resolved interaction with `VirtualPairCol` weights, `InteractionKind`, and `InteractionScope` | `crates/hypercube/src/lookup/interaction.rs` |
| InteractionKind | Enum classifying lookup target table: Memory, Program, Byte, State, Syscall, Global, ShaExtend, ShaCompress, Keccak, etc. | `crates/hypercube/src/lookup/interaction.rs` |
| InteractionScope | Scoping: Local (within shard) or Global (across shards); affects LogUp-GKR randomness | `crates/hypercube/src/air/interaction.rs` |
| IopCtx | Trait bundling base field F, extension field EF, Challenger, Hasher, Compressor, Digest types for interactive oracle proofs | `slop/crates/challenger/src/lib.rs` |
| Jagged PCS | PCS for variable-size (jagged) polynomial batches with stacked evaluation support | `slop/crates/jagged/` |
| KoalaBear | The base prime field used in SP1 (order 2130706433); aliased as `SP1Field` | `slop/crates/koala-bear/`, `crates/primitives/src/lib.rs` |
| LogUpGkr | Lookup argument protocol for cross-table interactions; uses `SepticDigest` global accumulation | `crates/hypercube/src/logup_gkr/` |
| Machine | Collection of `Chip` instances with a `MachineShape` and public values count | `crates/hypercube/src/machine.rs` |
| MachineAir | Trait for AIR circuits in a Machine: `name()`, `generate_trace()`, `generate_dependencies()`, `included()` | `crates/hypercube/src/air/` |
| MachineRecord | Trait for witness/trace records: `stats()`, `append()`, `public_values()`, `eval_public_values()` | `crates/hypercube/src/record.rs` |
| MachineShape | Set of chip clusters (`BTreeSet<Chip>`); determines which chips are active for a shard | `crates/hypercube/src/machine.rs` |
| MAX_CONSTRAINT_DEGREE | Maximum polynomial constraint degree = 3; enforced in `Chip::new()` | `crates/hypercube/src/chip.rs` |
| MerkleTreeTcs | Poseidon2-based Merkle tree tensor commitment scheme | `slop/crates/merkle-tree/src/tcs.rs` |
| Mle | Batch of multilinear polynomials as 2D Tensor (rows=hypercube evals, cols=polynomials); generic over field and backend | `slop/crates/multilinear/src/mle.rs` |
| MleEval | Batch of scalar evaluations of MLEs at a single point | `slop/crates/multilinear/src/mle.rs` |
| Opcode | SP1 custom instruction encoding of RISC-V ops; u8 variants from ADD=0 to UNIMP=52 | `crates/core/executor/src/opcode.rs` |
| PC_INC | Default program counter increment = 4 bytes | `crates/core/executor/src/lib.rs` |
| PlonkBn254Proof | PLONK proof over BN254 with 5 public_inputs, encoded_proof, raw_proof, plonk_vkey_hash | `crates/verifier/src/proof.rs` |
| Point | n-dimensional evaluation point (vector of field elements) for multilinear evaluation | `slop/crates/multilinear/src/point.rs` |
| Precompile | Cryptographic accelerator invoked as a syscall with dedicated AIR table (SHA, Keccak, EC ops, Poseidon2) | `crates/core/executor/src/syscall_code.rs` |
| PrecompileEvent | Tagged union of all precompile-specific event types, keyed by `SyscallCode` | `crates/core/executor/src/events/precompiles/mod.rs` |
| Prover (trait) | SDK user-facing trait: `setup(elf)`, `prove(pk, stdin)`, `execute(elf, stdin)`, `verify(proof, vkey)` | `crates/sdk/src/prover.rs` |
| PublicValues | Cross-shard state: timestamps, PC, cumulative digests, memory bounds, exit code, proof nonce | `crates/hypercube/src/air/public_values.rs` |
| RecursionAir | Enum of recursion chips; factory methods: `compress_machine()`, `shrink_machine()`, `wrap_machine()` | `crates/recursion/machine/src/machine.rs` |
| SepticCurve | Elliptic curve y^2 = x^3 + 45x + 41z^3 over F_{p^7}; accumulates global interaction digests | `crates/hypercube/src/septic_curve.rs` |
| SepticDigest | Global cumulative sum digest as `SepticCurve` point; represents multiset fingerprint across all global interactions | `crates/hypercube/src/septic_digest.rs` |
| SepticExtension | Degree-7 field extension F_p[z]/(z^7 - 3z - 5); coordinates for `SepticCurve` | `crates/hypercube/src/septic_extension.rs` |
| Shard | Subdivision of full execution trace proven independently; conceptual unit backed by `ExecutionRecord` | `crates/core/executor/src/record.rs` |
| ShardProof | Proof for a single shard; generic over `IopCtx` and PCS proof type | `crates/hypercube/` |
| ShardProver | Generates per-shard STARK proofs: trace gen, PCS commit, LogUp-GKR, zerocheck sumcheck, eval proof | `crates/hypercube/src/prover/shard.rs` |
| SP1CircuitWitness | Enum of witness inputs for circuit stages: Core, Deferred, Compress, Shrink, Wrap | `crates/prover/src/types.rs` |
| SP1Controller | Task-level orchestration: artifact download, CoreExecute task, compress/shrink/wrap pipeline | `crates/prover/src/worker/controller/mod.rs` |
| SP1Field | Type alias for `KoalaBear`; the base field throughout SP1 | `crates/primitives/src/lib.rs` |
| SP1GlobalContext | `IopCtx` for RISC-V and inner recursion stages; `KoalaBearDegree4Duplex` | `crates/primitives/src/lib.rs` |
| SP1OuterGlobalContext | `IopCtx` for outer recursion (wrap and SNARK stages); `BNGC` over SP1GlobalContext fields | `crates/primitives/src/lib.rs` |
| SP1Proof | Enum of proof modes: Core (Vec<ShardProof>), Compressed, Plonk, Groth16 | `crates/verifier/src/proof.rs` |
| SP1ProofWithPublicValues | User-facing proof bundle: proof, public_values, sp1_version, tee_proof | `crates/sdk/src/proof.rs` |
| SP1ProverComponents | Trait associating CoreProver, RecursionProver, WrapProver type parameters | `crates/prover/src/components.rs` |
| SP1Stdin | Input stream for zkVM program; provides `write()` for typed values | `crates/core/machine/` (io module) |
| SP1VerificationError | Error enum: InvalidPublicValues, VersionMismatch, Core, Recursion, Plonk, Groth16, Other, UnexpectedExitCode | `crates/sdk/src/prover.rs` |
| Stacked PCS | Commits heterogeneous batches by stacking into virtual uniform vector; Ligero-style interleaving | `slop/crates/stacked/` |
| StatusCode | Exit code wrapper; SUCCESS (0) checked during verification unless overridden | `crates/core/executor/` |
| Sumcheck | Interactive proof protocol for verifying multilinear polynomial evaluations | `slop/crates/sumcheck/` |
| SupervisorMode | Zero-sized marker implementing `TrustMode`; IS_TRUSTED=true; no page protection checks | `crates/core/machine/src/lib.rs` |
| SyscallCode | System call identifier via ECALL; u32 with byte-level fields (byte0=ID, byte1=has_own_table) | `crates/core/executor/src/syscall_code.rs` |
| TensorCsProver | Trait for tensor commitment scheme provers: `commit_tensors()`, `prove_openings_at_indices()` | `slop/crates/merkle-tree/src/tcs.rs` |
| TrustMode | Trait distinguishing supervisor (trusted) vs. user (untrusted) execution modes | `crates/core/machine/src/lib.rs` |
| UserMode | Zero-sized marker; IS_TRUSTED=false; adds trap/page-protection columns | `crates/core/machine/src/lib.rs` |
| WHIR | Polynomial commitment scheme | `slop/crates/whir/` |
| Zerocheck | Sumcheck-based protocol proving constraint polynomial vanishes on Boolean hypercube | `crates/hypercube/src/prover/zerocheck/` |
