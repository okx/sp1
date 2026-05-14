---
name: "sp1-cuda"
description: "Module design for sp1-cuda: client-server wrapper for GPU-accelerated proving"
---
# sp1-cuda Module

## Responsibilities

- Provide `CudaProver` as client wrapper for GPU proving
- Communicate with local GPU proving server via reqwest (JSON, rustls-tls)
- Hold `CudaClient` pointing to CUDA gRPC server
- Provide `CudaProvingKey` as remote handle

## NOT Responsible For

- Implementing GPU kernels (server-side only, accessed over IPC)
- CPU-based proving
- Network proving

## Core Entities

| Entity | Key Fields | Description |
|--------|-----------|-------------|
| `CudaProver` | CudaClient | GPU prover client |
| `CudaClient` | endpoint | Connection to GPU server |
| `CudaProvingKey` | (remote handle) | Remote proving key |

## Dependencies

- Require to reference arch/dependency.md for full dependency details

## Relevant Flows

- Require to reference core-flows/ for flows involving this module

## Module-Specific Pitfalls

[Pitfall] GPU server must be running locally before `CudaProver` can be used — no automatic server startup
