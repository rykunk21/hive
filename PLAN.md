# Hive Implementation Plan

## Phase 1: Foundation (No self-modification yet)

### 1.1 Dependency Setup
- Add `rig` to Cargo.toml for LLM tooling
- Add `tokio` with full features for async runtime
- Add `serde` and `toml` for config serialization
- Verify all dependencies compile

### 1.2 Configuration Layer
- Define `hive.toml` schema (pod types, LLM provider, resource limits)
- Create `src/config.rs` for loading and validating hive.toml
- Hive reads config on startup, initializes components from it

### 1.3 Knowledge Base Skeleton
- Define `KnowledgeBase` struct with in-memory storage
- Define `Index` trait for pluggable retrieval backends
- Implement per-pod-type embedding space registration
- Stub retrieval methods (return placeholder results)

### 1.4 Pod Trait Definition
- Define `Pod` trait with `init`, `run`, `shutdown` methods
- Define `PodType` metadata struct (name, retrieval preferences, resource budget)
- Create `PodRegistry` for mapping type names to constructors
- Implement a minimal `EchoPod` as proof-of-concept

### 1.5 Hive Core Loop
- Replace empty `Hive` struct with real orchestration state
- Implement `Hive::new(config)` that loads config and initializes knowledge base
- Implement `Hive::spawn_pod(type_name)` that instantiates and runs a pod
- Implement basic event collection from pods
- TUI wires into real hive state instead of hardcoded data

### 1.6 Link / Messaging Layer
- Define `Message` enum for hive-pod and pod-pod communication
- Implement async message channels using tokio::sync::mpsc
- Hive routes messages between pods and aggregates results

## Phase 2: Retrieval & Routing

### 2.1 Embedding Pipeline
- Integrate rig for text embeddings
- Pod outputs are embedded and stored in per-type knowledge space
- Query routing directs retrieval to correct embedding space

### 2.2 Retrieval Optimization
- Implement similarity search over embeddings
- Per-pod-type retrieval strategies (different top-k, different filters)
- Knowledge base tracks which retrievals were useful (feedback loop for later optimization)

### 2.3 Pod Lifecycle Management
- Pods report status to hive (running, completed, failed)
- Hive reaps completed pods and recycles resources
- TUI shows real pod state transitions

## Phase 3: Self-Modification (The Core Differentiator)

### 3.1 Metrics Collection
- Track pod success/failure rates per type
- Track retrieval accuracy (did the retrieved info help?)
- Track resource utilization
- Store metrics in knowledge base

### 3.2 Delta Generation
- Use LLM (via rig) to analyze metrics and propose source changes
- Generate structured diffs for pod logic, hive routing, or retrieval config
- Deltas are proposals, not direct mutations

### 3.3 Validation Gate
- Compile check: apply diff to temp copy, run `cargo check`
- Property tests: run existing tests, verify nothing breaks
- Approval step: TUI or config flag controls whether diffs auto-apply

### 3.4 Apply & Commit
- Apply validated diff to working tree
- Commit with structured message including metrics that motivated the change
- Tag commit with hive version + lineage info

### 3.5 Rollback
- Maintain git history so every change is reversible
- Hive can detect if a self-modification degraded performance and auto-revert

## Phase 4: Fork Model

### 4.1 Lineage Tracking
- Record parent hive commit in hive metadata
- Track which commits are "structural improvements" vs "task-specific tuning"

### 4.2 Upstream Pull
- Detect when parent hive has new structural commits
- Offer to cherry-pick/merge upstream improvements
- Preserve local task-specific adaptations during merge

### 4.3 Downstream Publish
- Tag a hive as "publishable" — others can fork it
- Include lineage metadata in published repo

## Phase 5: Polish

### 5.1 Persistent Storage
- Knowledge base persists to disk (sqlite or file-based vector store)
- Hive state survives restarts
- Configuration history tracked

### 5.2 Advanced Pod Types
- Code generator pod (writes Rust code)
- Evaluator pod (runs tests, reports results)
- Planner pod (breaks tasks into sub-tasks for other pods)

### 5.3 Distributed Mode (Future)
- Multiple hive instances coordinate over network
- Pods can run on remote nodes
- Knowledge base replication across hive cluster

## Current Priority

Start with Phase 1. The goal is a running system where:
- Config loads from `hive.toml`
- Hive can spawn a typed pod
- Pod runs and reports back
- TUI shows real (not fake) state
- Everything compiles and runs end-to-end

Self-modification comes after the basic loop works.
