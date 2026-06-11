# Hive Design Document

## Overview

Hive is a self-evolving agent orchestration loop built in Rust. It models a software system as a living colony: a central hive manages shared state, knowledge, and retrieval infrastructure. Typed pods (agents) run concurrently within the hive, contributing to and drawing from shared resources. Over time, the system rewrites itself based on runtime learning.

## Core Concepts

### The Hive

The hive is the core runtime. It owns:
- A shared knowledge base with per-pod-type retrieval optimization
- Resource allocation and inter-pod communication channels
- The self-modification loop that rewrites source artifacts based on runtime learning
- Upstream lineage tracking so downstream forks can pull structured updates

### Pods

Pods are typed agent units that run within the hive. Each pod type signals to the hive how to index, retrieve, and route information relevant to that agent's domain. Pod types include planners, summarizers, code generators, retrievers, evaluators, or any domain-specific worker.

Pods are not static workers. Like the hive itself, pod source is self-modifying. A pod accumulates task-specific learning and can rewrite its own behavior over time.

### The Fork Model

The most distinctive feature of hive is its fork-propagation model:

1. Fork the hive or a pod repo. Your fork starts as an exact copy.
2. Run it. The hive and its pods begin optimizing themselves for your workload.
3. Your fork diverges into a task-optimized configuration.
4. Others can fork yours. They inherit your optimizations as a starting point, then build further.
5. Upstream updates propagate. If the source hive evolves, you can pull structural improvements without losing local adaptations.

This creates a tree of progressively specialized hives.

```
upstream/hive ──→ your/hive ──→ colleague/hive
   (base)        (task-tuned)   (further specialized)
     ↑               ↑
  pulls updates   pulls updates
```

## Architecture

### Hive Core (`src/hive/`)

The hive core is responsible for:
- **Orchestration loop**: Spawns pods, monitors health, collects results, triggers self-modification cycles
- **Knowledge base management**: Maintains embeddings, indexes, and retrieval strategies per pod type
- **Resource allocation**: CPU, memory, and LLM API budget distribution across active pods
- **Fork lineage tracking**: Records parent-child relationships between hives for structured update propagation
- **Self-modification pipeline**: Bounded, auditable rewrite of source artifacts based on runtime metrics

### Pod Runtime (`src/pod/`)

The pod runtime defines:
- **Pod trait**: Lifecycle methods (`init`, `run`, `shutdown`) plus metadata for type registration
- **Type registry**: Maps pod type names to their constructors and retrieval preferences
- **Communication protocol**: How pods send/receive messages with the hive and other pods
- **Learning accumulator**: Per-pod storage of task patterns that inform self-modification

### Knowledge Base (`src/knowledge/`)

The knowledge layer provides:
- **Per-type retrieval optimization**: Separate embedding spaces and retrieval strategies indexed by pod type
- **Storage backend**: Pluggable vector store (initially in-memory, later persistent)
- **Indexing pipeline**: Converts pod outputs into searchable knowledge entries
- **Query router**: Directs retrieval requests to the appropriate index based on pod type context

### Link / Communication (`src/link/`)

The link layer handles:
- **Inter-pod messaging**: Async message passing between pods and the hive
- **Result aggregation**: Collects pod outputs and routes them to the next phase of the orchestration loop
- **Backpressure**: Throttles pods when the hive or knowledge base is under load

### TUI (`src/tui/`)

The terminal UI provides:
- **Pod registry view**: Available pod types and their status
- **Active pod monitor**: Running pods, resource usage, current task
- **Activity log**: Timeline of hive events, pod spawns/completions, self-modification cycles
- **Control surface**: Manual commands (spawn, kill, trigger self-modification, view fork lineage)

### Self-Modification Pipeline

Self-modification is bounded and auditable:
1. **Metrics collection**: Runtime data on pod performance, retrieval accuracy, task success rates
2. **Delta generation**: Proposed changes to source artifacts (pod logic, hive routing, retrieval config)
3. **Validation gate**: Compile check, property tests, and approval before applying changes
4. **Apply & commit**: Structured rewrite with rollback capability
5. **Lineage record**: Changes are tagged and pushed as commits for fork tracking

### External Dependencies

- **rig**: LLM completions, embeddings, and agent tooling layer
- **tokio**: Async runtime (lightweight compared to full actix for local loop)
- **ratatui**: Terminal UI framework (already integrated)
- **serde + toml**: Configuration and artifact serialization

## Data Flow

```
┌─────────┐     ┌─────────┐     ┌─────────┐
│  User   │────→│  Hive   │────→│  Pods   │
│ (TUI)   │←────│ (core)  │←────│ (typed) │
└─────────┘     └────┬────┘     └─────────┘
                     │
              ┌──────┴──────┐
              │ Knowledge   │
              │   Base      │
              └──────┬──────┘
                     │
              ┌──────┴──────┐
              │   Self-     │
              │ Modification│
              │  Pipeline   │
              └─────────────┘
```

## Design Goals

1. Minimal coupling between hive core and individual pods
2. Pod types are declarative — the hive infers retrieval and routing from type metadata
3. Self-modification is bounded and auditable
4. Fork lineage is a first-class data structure, not a convention
5. The system becomes progressively more optimized for its workload over time

## Status

Early design phase. The fork model and pod-type abstraction are conceptually defined. Implementation begins with the hive core loop and pod runtime.
