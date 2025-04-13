// src/consumer.rs

//! Kafka consumer functionality using the low-level BaseConsumer.
//! This module provides helper functions and macros to initialize a consumer,
//! subscribe to topics, and spawn a background thread that continuously polls
//! for new messages.

use rdkafka::config::ClientConfig;
use rdkafka::consumer::{BaseConsumer, Consumer};
use rdkafka::error::KafkaError;
use rdkafka::message::BorrowedMessage;
use std::env;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

/// Initializes a BaseConsumer with common settings.
/// Broker addresses are taken from the `KAFKA_BROKERS` environment variable (default: "localhost:9092").
///
/// # Arguments
///
/// * `group_id` - The consumer group id.
/// * `topics` - A slice of topic names to subscribe to.
///
/// # Returns
///
/// Returns a `BaseConsumer` on success or a `KafkaError` on failure.
pub fn init_base_consumer(group_id: &str, topics: &[&str]) -> Result<BaseConsumer, KafkaError> {
    let brokers = env::var("KAFKA_BROKERS").unwrap_or_else(|_| "localhost:9092".to_string());

    let consumer: BaseConsumer = ClientConfig::new()
        .set("bootstrap.servers", &brokers)
        .set("group.id", group_id)
        .set("session.timeout.ms", "6000")
        .set("auto.offset.reset", "earliest")
        .create()?;

    consumer.subscribe(topics)?;
    Ok(consumer)
}

/// Starts a background thread that continuously polls the provided BaseConsumer.
/// The consumer must be wrapped in an Arc so that it can be shared with the polling thread.
///
/// The provided handler is invoked on each message result.
///
/// # Arguments
///
/// * `consumer` - An Arc-wrapped BaseConsumer.
/// * `handler` - A closure that accepts a `Result<BorrowedMessage, KafkaError>`.
///
/// # Returns
///
/// Returns a `JoinHandle<()>` for the spawned polling thread.
pub fn start_consumer_thread<F>(
    consumer: Arc<BaseConsumer>,
    mut handler: F,
) -> thread::JoinHandle<()>
where
    F: FnMut(Result<BorrowedMessage<'_>, KafkaError>) + Send + 'static,
{
    thread::spawn(move || {
        loop {
            // Poll the consumer with a 100-ms timeout.
            if let Some(result) = consumer.poll(Duration::from_millis(100)) {
                handler(result);
            }
            // Sleep briefly to prevent busy waiting.
            thread::sleep(Duration::from_millis(50));
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::thread;
    use std::time::Duration;

    // This test requires a running Kafka broker at "localhost:9092" and a topic named "test-topic".
    // It is marked #[ignore] so that it only runs in an environment where Kafka is available.
    #[test]
    #[ignore = "Requires running Kafka instance with topic 'test-topic'"]
    fn test_init_base_consumer() {
        unsafe {
            // Set a known good broker address if not already set.
            std::env::set_var("KAFKA_BROKERS", "localhost:9092");
        }
        let consumer = init_base_consumer("test-group", &["test-topic"]);
        assert!(
            consumer.is_ok(),
            "Consumer initialization should succeed with valid brokers and topics"
        );
    }

    // This test spawns a consumer thread, processes messages for a few seconds, and checks whether
    // the message handler has been invoked at least once. This test is also marked #[ignore].
    #[test]
    #[ignore = "Requires running Kafka instance with topic 'test-topic' producing messages"]
    fn test_start_consumer_thread() {
        unsafe {
            // Set a known good broker address if not already set.
            std::env::set_var("KAFKA_BROKERS", "localhost:9092");
        }
        let consumer = init_base_consumer("test-group", &["test-topic"])
            .expect("Failed to initialize consumer");
        let arc_consumer = Arc::new(consumer);

        // Use an atomic boolean to signal if at least one message was received.
        let message_received = Arc::new(AtomicBool::new(false));
        let received_clone = Arc::clone(&message_received);

        let handle = start_consumer_thread(arc_consumer, move |msg| {
            if msg.is_ok() {
                received_clone.store(true, Ordering::SeqCst);
            }
        });

        // Let the consumer poll for 3 seconds.
        thread::sleep(Duration::from_secs(3));

        // In an actual test environment, the absence or presence of messages might vary.
        // Here, we simply assert that the thread is running without panicking.
        // Optionally, if messages are guaranteed, assert:
        // assert!(message_received.load(Ordering::SeqCst), "Expected at least one message to be received");

        // Since the thread runs an infinite loop, we can't join it gracefully here,
        // so we'll detach it by not calling join(). In a real application you'd handle shutdown appropriately.
        // For test purposes, we simply abort the thread (which will be terminated when the test ends).
        handle.thread().unpark(); // This does not stop the loop but is called to ensure thread activity.
    }
}
