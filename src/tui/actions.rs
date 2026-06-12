//! TUI actions — user-triggered commands from keyboard input.
//!
//! Actions are high-level user intents. The TUI event loop maps keystrokes
/// to actions, then dispatches them to the hive or UI state.

/// User action triggered by keyboard input.
///
/// TODO: Add spawn/kill/evolve actions once hive supports them.
#[derive(Debug, Clone)]
pub enum Action {
    /// Quit the TUI and shut down the hive.
    Quit,
    // Send the prompt that is in the bottom bar
    Submit, // TODO:
    // spawn a pod of the given type
    SpawnPod, // TODO: KillPod(String) — kill a pod by ID
}
