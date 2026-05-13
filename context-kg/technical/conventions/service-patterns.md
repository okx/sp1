---
name: "service-patterns"
description: "Task orchestration, error handling, retry, and concurrency patterns in SP1"
---
# Service Patterns

## Task Orchestration Pattern

[Convention] **Controller-Worker model**: `SP1Controller` orchestrates proving by submitting tasks to `WorkerClient`. Tasks are typed (`TaskType::CoreExecute`, compress, shrink, wrap) and tracked via channels.
- `LocalWorkerClient` uses in-process mpsc/watch channels
- Tasks have `TaskId` and `ProofId` identifiers
- Channels are pre-created for all known `TaskType` variants at init

## Error Classification Pattern

[Convention] **Retryable vs Fatal errors**: `TaskError` classifies errors into three categories:
- `Retryable`: Network/transport errors (reqwest, tonic Internal/Unavailable/etc.) — auto-retry
- `Fatal`: Logic errors, invalid input — no retry
- `Execution`: VM execution failures — no retry

## Network Retry Pattern

[Convention] **All gRPC calls with retry**: `NetworkClient` wraps all gRPC calls in `with_retry`/`with_retry_timeout`. Direct RPC calls without retry are prohibited.

## Artifact Compression Pattern

[Convention] **Bincode + zstd**: All artifacts (ELF, stdin, proofs) are bincode-serialized and zstd-compressed (level 3) before network upload. Presigned S3-compatible URLs are used for actual data transfer.

## Concurrency Pattern

[Convention] **Async trace gen + blocking proof**: `ShardProver` generates traces asynchronously, then moves blocking proof computation to `tokio::task::spawn_blocking`. This prevents blocking the tokio runtime.

## Sequential Pipeline Gating

[Convention] **Channel-based stage gating**: Pipeline stages are gated by tokio channels. Compress completion signals shrink/wrap readiness via `compress_complete_tx/rx`. This prevents premature task submission.

## Memory Management Pattern

[Convention] **Semaphore-based permits**: `ShardProver` uses memory permits (`MemoryPermit`) to limit concurrent proving operations based on available system memory.

## Version Checking Pattern

[Convention] **Pre-validation version check**: All proof verification starts with `SP1_CIRCUIT_VERSION` comparison. Mismatched versions are rejected before any cryptographic work.
