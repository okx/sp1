---
name: "proving-pitfalls"
description: "Known pitfalls and warnings related to the proving pipeline"
---
# Proving Pitfalls

[Pitfall] **Empty preprocessed traces**: Preprocessed traces must not be empty when committing — runtime assertion in `ShardProver::prove_shard_with_data`. Trigger: incorrect shape selection excluding required chips. Correct approach: ensure `MachineShape` includes all chips needed for the shard's events. Affected module: `sp1-hypercube`

[Pitfall] **Constraint degree overflow**: `MAX_CONSTRAINT_DEGREE` is 3; adding constraints of degree > 3 causes assertion failure in `Chip::new()`. Trigger: creating a new AIR with high-degree polynomial constraints. Correct approach: decompose high-degree constraints using intermediate variables. Affected module: `sp1-hypercube`

[Pitfall] **Proof construction order**: Shard proof construction order is fixed — commit_traces -> prove_logup_gkr -> zerocheck -> prove_trusted_evaluations. Trigger: attempting to reorder steps. Correct approach: follow the exact sequence; Fiat-Shamir transcript depends on ordering. Affected module: `sp1-hypercube`

[Pitfall] **spawn_blocking for proof work**: `ShardProver::prove_shard_with_pk` uses `tokio::task::spawn_blocking` for CPU-intensive proof generation. Trigger: calling from sync context without tokio runtime. Correct approach: ensure tokio runtime is available. Affected module: `sp1-hypercube`

[Pitfall] **Compress/shrink/wrap ordering**: Shrink/wrap tasks must not be submitted until compress completes — gated by `compress_complete_rx` channel. Trigger: submitting tasks out of order. Correct approach: await `compress_complete_rx` before submitting shrink. Affected module: `sp1-prover`

[Pitfall] **LogUp-GKR interaction count**: Number of LogUp-GKR interactions should be calculated from actual interaction data, not hardcoded. TODO noted in `logup_gkr/prover.rs`. Affected module: `sp1-hypercube`

[Pitfall] **sum_as_poly degree limitation**: Currently only supports degree-3 constraint polynomials. TODO noted in `prover/zerocheck/sum_as_poly.rs`. Correct approach: wait for flexibility to be added for degree-2. Affected module: `sp1-hypercube`

[Warning] **Rayon parallelization disabled**: Variable memory chip trace generation has rayon parallelization commented out with TODO to re-enable. Source: `recursion/machine/src/chips/mem/variable.rs`. Affected module: `sp1-recursion-machine`

[Warning] **Shape module deprecation**: Parts of the shape module are marked for deprecation. Source: `hypercube/src/shape/mod.rs`. Affected module: `sp1-hypercube`
