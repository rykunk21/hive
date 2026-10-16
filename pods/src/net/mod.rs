//! Protocol router for MCP and JSON-RPC requests.
//!
//! This module routes external messages to the pod queue:
//! - **MCP**: Manages pod subscriptions to MCP servers.
//! - **JSON-RPC**: Handles external messages sent outside the direct UI.
//!
//! The router inspects each request and forwards it to the appropriate handler.
//!
//!
