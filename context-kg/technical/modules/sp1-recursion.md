---
name: "sp1-recursion"
description: "Module design for sp1-recursion: recursive proof composition with circuit DSL and Gnark wrapping"
---
# sp1-recursion Module

Spans multiple crates: `recursion/circuit`, `recursion/compiler`, `recursion/executor`, `recursion/machine`, `recursion/gnark-ffi`.

## Responsibilities

- Define `CircuitConfig` and `WrapConfig` (inner/outer DSL configurations)
- Implement recursive verifiers: `SP1RecursiveVerifier`, `SP1CompressWithVKeyVerifier`, `SP1DeferredVerifier`, `SP1CompressRootVerifierWithVKey`
- Provide recursion machine chips via `RecursionAir` enum
- Factory methods: `compress_machine()`, `shrink_machine()`, `wrap_machine()` with distinct chip sets
- Compile recursion DSL programs via `AsmCompiler`
- Bridge to Gnark (Go) for Groth16/PLONK BN254 proof generation (native FFI or Docker)

## NOT Responsible For

- Core RISC-V AIR constraints (that is sp1-core-machine)
- Shard-level STARK proving (that is sp1-hypercube)
- User-facing API (that is sp1-sdk)

## Core Entities

| Entity | Key Fields | Description |
|--------|-----------|-------------|
| `RecursionAir` | chip variants (MemoryConst, MemoryVar, BaseAlu, ExtAlu, Poseidon2Wide, etc.) | Enum of recursion chips |
| `CircuitConfig` | Bit type, circuit primitives | Inner circuit configuration |
| `WrapConfig` | (extends CircuitConfig for outer) | Outer circuit configuration |
| `SP1FieldConfigVariable` | impl for SP1GlobalContext and SP1OuterGlobalContext | Field-specific circuit operations |
| `Groth16Bn254Proof` | public_inputs, encoded_proof, groth16_vkey_hash | Terminal Groth16 proof |
| `PlonkBn254Proof` | public_inputs, encoded_proof, plonk_vkey_hash | Terminal PLONK proof |

## Dependencies

- Require to reference arch/dependency.md for full dependency details

## Relevant Flows

- Require to reference core-flows/ for flows involving this module

## Module-Specific Pitfalls

[Pitfall] `wrap_machine()` uses different chip set than `compress_machine()`/`shrink_machine()` — wrap drops `Poseidon2Wide` and uses `Poseidon2LinearLayer` + `Poseidon2SBox` + `ExtFeltConvert` instead

[Pitfall] Gnark FFI requires either native Go toolchain (feature=native) or Docker — default is Docker which is slower but requires no Go setup

[Pitfall] `SP1OuterGlobalContext` must be used for wrap stage, not `SP1GlobalContext` — mixing contexts produces invalid circuits

[Pitfall] Recursion programs must be compiled using `AsmCompiler` after `Builder<C>` — `DslIrProgram` must be created with `new_unchecked`
