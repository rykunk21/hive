//! LLM providers for pods.
//!
//! Integrates with `rig` to let pods make LLM calls. During setup, a pod can:
//! - Attach available **local tools**.
//! - Register with **MCP servers** exposed over the API.
