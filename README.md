# Hive

A self-evolving agent swarm built in Rust. Hive is an experimental multi-crate workspace exploring computational agent loops, Cartesian Genetic Programming (CGP) for pod behavior, and terminal-based orchestration UIs.

## Architecture

Hive is organized as a Cargo workspace with four crates:

| Crate | Purpose | Status |
|-------|---------|--------|
| `pods` | Core hive runtime, TUI dashboard, actor-based pod system, CGP genome engine | Active development |
| `scout` | Notebook-style TUI for data exploration (Polars, Rhai scripting, plotting) | Active development |
| `nest` | Placeholder binary (reserved for future hive node / worker process) | Skeleton |
| `swarm` | Placeholder library for learning, logic, modeling, optimization, and search modules | Skeleton |


```mermaid
flowchart LR
    pods[Pods]
    scout[Scout]
    nest[Nest]
    api[API]
    dock[Dock]

    %% Pods
    pods --> p_queue[Queue]
    p_queue --> p_podmessage[PodMessage]
    pods --> p_pod[Pod]
    p_pod --> p_policy[Policy]
    p_pod --> p_genome[Genome]
    p_genome --> p_genomelist[GenomeList]
    p_pod --> p_taskspec[Task Spec]
    p_pod --> p_llm[LLM Provider]
    p_llm --> p_tools[Tools]
    p_tools --> p_shell[Shell]
    p_shell --> p_ssh[SSH]
    p_tools --> p_discover[Discover other Pods]
    p_tools --> p_send[Send a message]
    pods --> p_spawner[Spawner]
    pods --> p_net[Net]
    p_net --> p_router[Protocol Router]
    p_router --> p_jsonrpc[JSON RPC]
    p_router --> p_mcp[MCP]
    pods --> p_interface[Interface]
    p_interface --> p_tui[TUI]
    p_interface --> p_imcp[MCP]

    %% Scout
    scout --> s_engine[Engine]
    s_engine --> s_plotters[Plotters]
    s_engine --> s_dataframes[DataFrames]
    scout --> s_ui[UI]
    s_ui --> s_effects[Effects]
    scout --> s_net[Net]
    s_net --> s_tailnet[Tailnet]
    scout --> s_userinterface[User Interface]
    s_userinterface --> s_tuicli["TUI / CLI"]

    %% Nest
    nest --> n_conn[Conn]
    nest --> n_kg[Knowledge Graph]
    nest --> n_ke[Knowledge Extractor]
    nest --> n_interface[Interface]
    n_interface --> n_router[Protocol Router]
    n_router --> n_http[HTTP]
    n_router --> n_mcp[MCP]

    %% API
    api --> a_mcprouter[MCP Router]
    api --> a_auth[Authenticator]
    api --> a_tailnet[Tailnet]
    api --> a_server["pods.app Http server"]

    %% Dock
    dock --> d_auth[Authenticator]
    dock --> d_tailnet[Tailnet]

    %% Style definitions
    classDef podsRoot fill:#1d4ed8,stroke:#1e3a8a,stroke-width:3px,color:#ffffff
    classDef podsNode fill:#dbeafe,stroke:#1d4ed8,color:#1e3a8a
    classDef scoutRoot fill:#15803d,stroke:#14532d,stroke-width:3px,color:#ffffff
    classDef scoutNode fill:#dcfce7,stroke:#15803d,color:#14532d
    classDef nestRoot fill:#c2410c,stroke:#7c2d12,stroke-width:3px,color:#ffffff
    classDef nestNode fill:#ffedd5,stroke:#c2410c,color:#7c2d12
    classDef apiRoot fill:#7e22ce,stroke:#581c87,stroke-width:3px,color:#ffffff
    classDef apiNode fill:#f3e8ff,stroke:#7e22ce,color:#581c87
    classDef dockRoot fill:#b91c1c,stroke:#7f1d1d,stroke-width:3px,color:#ffffff
    classDef dockNode fill:#fee2e2,stroke:#b91c1c,color:#7f1d1d

    %% Class assignments
    class pods podsRoot
    class p_queue,p_podmessage,p_pod,p_policy,p_genome,p_genomelist,p_taskspec,p_llm,p_tools,p_shell,p_ssh,p_discover,p_send,p_spawner,p_net,p_router,p_jsonrpc,p_mcp,p_interface,p_tui,p_imcp podsNode

    class scout scoutRoot
    class s_engine,s_plotters,s_dataframes,s_ui,s_effects,s_net,s_tailnet,s_userinterface,s_tuicli scoutNode

    class nest nestRoot
    class n_conn,n_kg,n_ke,n_interface,n_router,n_http,n_mcp nestNode

    class api apiRoot
    class a_mcprouter,a_auth,a_tailnet,a_server apiNode

    class dock dockRoot
    class d_auth,d_tailnet dockNode
```


## Workspace Structure

```
hive/
├── Cargo.toml          # Workspace manifest (members: nest, pods, scout, swarm)
├── pods/
│   ├── Cargo.toml      # Package name: "pods", lib name: "hive", bin name: "hive"
│   └── src/
│       ├── main.rs          # TUI entrypoint — boots terminal and runs hive
│       ├── lib.rs             # Library exports: core, knowledge, link, pods, tui
│       ├── core/mod.rs        # Hive runtime: command/event loop, actix actor system
│       ├── knowledge/mod.rs   # Stubbed knowledge base with embedding/retrieval traits
│       ├── link/mod.rs        # Inter-pod messaging: PodMessage, PodResponse, PodConfig
│       ├── pods/
│       │   ├── mod.rs         # Built-in pod re-exports
│       │   ├── genome/mod.rs  # Cartesian Genetic Programming genome (Node, Genome, mutation)
│       │   ├── ping/mod.rs    # Simple Ping actor for testing actix message passing
│       │   ├── pod/mod.rs     # Pod actor: wraps a CGP Genome, handles queries
│       │   └── speaker/mod.rs # LLM speaker actor via rig + Ollama (currently unused)
│       └── tui/
│           ├── mod.rs         # Event loop (~60 FPS), keyboard input dispatch
│           ├── input.rs       # Keymap: Ctrl+P (ping), Enter (submit), Ctrl+S (spawn mode)
│           └── ui/mod.rs      # ratatui rendering: agent panel, hive graph, activity log
├── scout/
│   └── src/
│       ├── main.rs            # Notebook TUI entrypoint with vim keybindings
│       ├── app/               # Cell management, viewport, notebook state
│       ├── engine/            # Script execution, plotting, Polars integration
│       ├── ui/                # ratatui rendering and visual effects
│       └── util/              # Vim-mode input handling
├── nest/
│   └── src/main.rs            # "Hello, world!" placeholder
└── swarm/
    └── src/
        ├── lib.rs             # Placeholder add function
        ├── learning.rs        # Reserved
        ├── logic.rs           # Reserved
        ├── modeling.rs        # Reserved
        ├── optimization.rs    # Reserved
        └── search.rs          # Reserved
```

## Running

### Pods (Hive Runtime + TUI)

```bash
cd pods
cargo run --bin hive
```

**TUI Controls:**
- `Ctrl+P` — Send ping to the hive actor system
- `Enter` — Submit text from the input bar
- `Ctrl+S` — Toggle spawn overlay (prepares to spawn a new pod)
- `q` / `Ctrl+C` — Quit

### Scout (Notebook TUI)

```bash
cd scout
cargo run --bin spool [notebook.md]
```

**Vim-style controls:**
- `j/k` or `↓/↑` — Navigate cells
- `Enter` — Focus/unfocus cell editor
- `Tab` — Run cell and show output
- `a` — Add new cell
- `d` — Delete cell
- `o` — Toggle output panel
- `q` — Save and quit

## Key Concepts

### Pods
Pods are actix actors that participate in the hive runtime. Each pod carries a **Cartesian Genetic Programming (CGP) genome** that defines its behavior graph. Genomes are composed of:
- **Nodes** (`func` + backward-only input connections)
- **Output pointers** (value-table indices)
- **Mutation** (random function / connection flips)

### Hive Loop
The hive runs an async event loop inside an actix system on a background thread. It receives commands (`Ping`, `Submit`, `SpawnPod`) via channels and broadcasts responses to TUI subscribers.

### Knowledge Base (Stubbed)
A trait-based design for per-type embedding spaces, retrieval, and feedback loops. Currently unimplemented but provides the structural surface for future RAG-style memory.

## Dependencies

- **actix** — Actor framework for pod lifecycle
- **tokio** — Async runtime
- **ratatui** + **crossterm** — Terminal UI
- **rig-core** — LLM client abstraction (Ollama support)
- **Polars** — Data frames (scout)
- **Rhai** — Embedded scripting (scout)
- **petgraph** — Graph structures (scout)

## Status

This is an active research project. Many components are skeletal or stubbed (`todo!()`). The pods crate has the most mature runtime, while scout provides a separate notebook interface for data workflows.

