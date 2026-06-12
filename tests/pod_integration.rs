//! Integration test: verify actix pod lifecycle end-to-end.
//!
//! Spawns a SpeakerPod via PodActor, sends a Task, and verifies the
//! TaskResult comes back through the actix mailbox.

use hive::pod::{PodActor, Task, TaskResult};
use hive::pods::speaker::SpeakerPod;
use actix::Actor;

#[actix::test]
async fn test_speaker_pod_echos_task() {
    // 1. Start the pod actor
    let addr = PodActor::start_for(SpeakerPod::new());

    // 2. Send a text task (no channel routing)
    let result = addr.send(Task::text("hello from test")).await.unwrap();

    // 3. Verify we got a result back
    assert!(result.is_ok(), "handle_task should succeed");
    let task_result = result.unwrap();

    // 4. Verify the response contains our input
    assert!(
        task_result.response.contains("hello from test"),
        "response should echo the input text"
    );
}

#[actix::test]
async fn test_speaker_pod_routes_reply() {
    // 1. Start the pod actor
    let addr = PodActor::start_for(SpeakerPod::new());

    // 2. Send a chat task tied to a channel
    let result = addr
        .send(Task::chat("hello", "#test-channel"))
        .await
        .unwrap();

    // 3. Verify routing is set
    let task_result = result.unwrap();
    assert_eq!(
        task_result.reply_to,
        Some("#test-channel".to_string()),
        "reply_to should match the input channel"
    );
}
