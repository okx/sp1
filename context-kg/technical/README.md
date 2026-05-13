---
name: "README"
description: "Directory index and reading guide for the SP1 zkVM technical knowledge base"
---
# context-kg — Knowledge Base

SP1 is a high-performance zkVM that proves correct execution of RISC-V programs using STARKs with Hypercube proving, recursive proof composition, and Groth16/PLONK wrapping for on-chain verification. Built in Rust as a Cargo workspace (v6.1.0).

## Files and Directories

| Path | Description |
|------|-------------|
| knowledge-base.md | **Highest authority** — all Skills defer to this on conflicts |
| terminology.md | Domain term glossary for unified terminology |
| arch/architecture-overview.md | Layer definitions, service responsibilities |
| arch/dependency.md | Inter-module deps, storage, external services |
| modules/ | Per-module design docs (responsibilities, entities, pitfalls) |
| pitfalls/ | Known failure modes grouped by topic |
| core-flows/ | Core proving, recursion, and verification flows |
| apis/ | SDK API conventions, error codes |
| conventions/ | Feature patterns, service patterns, common tools |
| repo_brief.json | Machine-readable repository summary |

## How to Read This Knowledge Base

1. **Knowledge base is the highest authority** — defer to it over general AI knowledge
2. **Locate the specific module** — read the module doc before starting work
3. **Check pitfalls and core flows first**
4. **Produce a constraint checklist** — explicitly declare if no relevant content
5. **Cross-validate during work** — correct violations immediately
