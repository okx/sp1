---
name: "recursion-pitfalls"
description: "Known pitfalls and warnings related to recursion and wrapping"
---
# Recursion Pitfalls

[Pitfall] **Wrap machine chip set**: `wrap_machine()` uses different chip set than `compress_machine()`/`shrink_machine()` — wrap drops `Poseidon2Wide` and uses `Poseidon2LinearLayer` + `Poseidon2SBox` + `ExtFeltConvert`. Trigger: using wrong machine for wrong stage. Correct approach: use factory methods strictly per stage. Affected module: `sp1-recursion-machine`

[Pitfall] **Outer context for wrap**: `SP1OuterGlobalContext` must be used for wrap stage, not `SP1GlobalContext`. Trigger: using inner context in wrap program. Correct approach: wrap_machine operates over `SP1OuterGlobalContext`; compress/shrink use `SP1GlobalContext`. Affected module: `sp1-recursion-circuit`

[Pitfall] **Gnark FFI mode**: Gnark FFI has two modes — native (feature=native, requires Go toolchain) and Docker (default). Trigger: expecting native Gnark without Go installed. Correct approach: use Docker mode for portability, native for performance. Affected module: `sp1-recursion-gnark-ffi`

[Pitfall] **Recursion compiler optimization**: Multiple arithmetic operations in recursion compiler could be optimized to single opcodes (4 TODOs in `ir/arithmetic.rs`). Trigger: performance-sensitive recursion circuits. Correct approach: await optimization; current multi-opcode sequences are functionally correct. Affected module: `sp1-recursion-compiler`

[Pitfall] **Compress witness mismatch**: There is a mismatch between `SP1CompressWithVKeyWitnessValues` and `SP1RecursionProof` noted in controller. Source: `prover/src/worker/controller/compress.rs`. Affected module: `sp1-prover`

[Warning] **Vkey verification cleanup**: After finalizing vkeys, the vkey verification solution needs cleanup. Source: `verifier/src/compressed/mod.rs`. Affected module: `sp1-verifier`

[Warning] **LogUp+GKR tests**: Zerocheck tests are commented out, pending re-addition. Source: `recursion/circuit/src/zerocheck.rs`. Affected module: `sp1-recursion-circuit`

[Warning] **Sumcheck builder parallel iteration**: Sumcheck module has TODO to use builder par iter collect. Source: `recursion/circuit/src/sumcheck/mod.rs`. Affected module: `sp1-recursion-circuit`
