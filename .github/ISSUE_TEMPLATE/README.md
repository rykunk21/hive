# Using Hive Issue Templates

When you open a new issue on the `rykunk21/hive` repo, GitHub will show you a template chooser. Here's how each one works.

---

## 1. Bug Report

**When to use:** Something crashes, hangs, produces wrong output, or behaves unexpectedly.

**What to fill in:**
- **Description** — one sentence of what's broken
- **Steps to Reproduce** — exact sequence to trigger it
- **Expected vs Actual** — what you wanted vs what happened
- **Environment** — Hive version, Rust version, OS
- **Logs** — stack traces, error messages, screenshots

**Example title:** `BUG: Hive panics when spawning second planner pod`

---

## 2. Feature Request

**When to use:** You want something new that doesn't exist yet.

**What to fill in:**
- **Summary** — one-line pitch
- **Motivation** — why the current system can't do this
- **Proposed Design** — high-level approach (not implementation detail)
- **Acceptance Criteria** — checklist of what "done" looks like
- **Scope** — what's in and what's deliberately out
- **Risks** — what could go wrong or simpler alternatives

**Example title:** `FEAT: Add persistent SQLite backend for knowledge base`

---

## 3. Pod Type Request

**When to use:** You want a new kind of agent in the hive.

**What to fill in:**
- **Pod Type Name** — short, descriptive (e.g., `code-reviewer`)
- **Domain** — what problem it solves
- **Interface** — what it expects as input, what it produces
- **Retrieval Preferences** — which knowledge it needs access to
- **Resource Budget** — CPU/memory/token estimates
- **Example Use Case** — concrete scenario
- **Similar Types** — how it's different from existing pods

**Example title:** `POD: Add dependency-analyzer pod for cargo tree analysis`

---

## 4. Refactor / Architecture Change

**When to use:** The code works but the structure is wrong — tech debt, performance bottleneck, or API cleanup.

**What to fill in:**
- **Current State** — describe the existing pattern
- **Problem** — why it's problematic
- **Proposed Change** — what it should look like
- **Impact** — affected modules, breaking changes, migration path
- **Verification** — how you prove the refactor succeeds (tests, benchmarks)

**Example title:** `REFACTOR: Replace thread-per-pod with tokio task pool`

---

## Quick Reference

| If you want to... | Use template | Labels |
|---|---|---|
| Report a crash or bug | Bug Report | `bug`, `triage` |
| Add new functionality | Feature Request | `enhancement`, `triage` |
| Add a new agent type | Pod Type Request | `pod-type`, `triage` |
| Restructure existing code | Refactor | `refactor`, `triage` |
| Ask a question or discuss | Discussions (not issues) | — |
| Report security vulnerability | Security Advisories | — |

---

## After Filing

All templates start with the `triage` label. A maintainer will:
1. Read and clarify if needed
2. Assign proper labels (`backlog`, `red`, `green`, etc. for kanban)
3. Link to a milestone if it fits a phase
4. Either start work or close as duplicate / out-of-scope

---

## Tips

- **Be specific.** "Hive is slow" is useless. "Spawning 10 pods takes 30s" is actionable.
- **Include context.** Paste the relevant design doc or prior issue that motivates this.
- **Keep scope tight.** One issue = one problem or one feature. Don't bundle.
- **Check existing issues first.** Search before filing to avoid duplicates.
