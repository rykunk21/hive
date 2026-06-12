//! Integration test: verify actix pod lifecycle end-to-end.
//!
//! Spawns a SpeakerPod via PodActor, sends a Task, and verifies the
//! TaskResult comes back through the actix mailbox.

use hive::pod::{PodActor, Task};
use hive::pods::speaker::SpeakerPod;
use actix::Actor;

#[actix::test]
async fn test_speaker_pod_calls_llm() {
    // 1. Start the pod actor
    let addr = PodActor::start_for(SpeakerPod::new());

    // 2. Send a text task with a short prompt
    let result = addr.send(Task::text("hi")).await;

    // 3. Verify the message was delivered and processed
    assert!(result.is_ok(), "message should be delivered to actor");
    let inner = result.unwrap();
    assert!(inner.is_ok(), "LLM call should succeed: {:?}", inner.err());

    let task_result = inner.unwrap();

    // 4. Verify we got a non-empty response from the LLM
    assert!(
        !task_result.response.is_empty(),
        "LLM should return a response, got: '{}'",
        task_result.response
    );
    println!("LLM response: {}", task_result.response);
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
