---
name: "sp1-primitives"
description: "Module design for sp1-primitives: core type aliases, Poseidon2 hash, and field definitions"
---
# sp1-primitives Module

## Responsibilities

- Define `SP1Field` (KoalaBear), `SP1ExtensionField` (BinomialExtensionField<KoalaBear, 4>)
- Define `SP1GlobalContext` (KoalaBearDegree4Duplex) and `SP1OuterGlobalContext` (BNGC)
- Provide Poseidon2 hash function (`poseidon2_hash`, `SP1Perm`, `POSEIDON2_HASHER`)
- Provide `hash_deferred_proof` for building deferred proof digest chains
- Define `SP1PublicValues` with `hash()` (SHA256) and `blake3_hash()` methods
- Provide `Elf` type for program binaries

## NOT Responsible For

- Any proving or verification logic
- VM execution
- Field arithmetic implementation (that is slop-koala-bear, slop-bn254)

## Core Entities

| Entity | Key Fields | Description |
|--------|-----------|-------------|
| `SP1Field` | KoalaBear (p=2130706433) | Base field type alias |
| `SP1ExtensionField` | degree-4 over KoalaBear | Challenge field |
| `SP1Perm` | Poseidon2, width=16, 3 external rounds | Poseidon2 permutation |
| `SP1PublicValues` | buffer | Public values with dual-hash support |

## Dependencies

- Require to reference arch/dependency.md for full dependency details

## Relevant Flows

- Require to reference core-flows/ for flows involving this module

## Module-Specific Pitfalls

None identified
