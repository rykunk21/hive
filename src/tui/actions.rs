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

    // TODO: SpawnPod(String) — spawn a pod of the given type
    // TODO: KillPod(String) — kill a pod by ID
    // TODO: TriggerEvolve — run one self-modification cycle
    // TODO: SelectNext — navigate to next item in focused panel
    // TODO: SelectPrev — navigate to previous item in focused panel
    // TODO: ScrollUp — scroll activity log up
    // TODO: ScrollDown — scroll activity log down
}
