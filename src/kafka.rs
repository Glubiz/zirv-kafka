use std::sync::OnceLock;
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use zirv_config::read_config; // Assuming your project provides a similar configuration macro

// Our global, one-time-initialized Kafka producer
static KAFKA_PRODUCER: OnceLock<FutureProducer> = OnceLock::new();

/// Initializes the global Kafka producer exactly once.
///
/// This function should be called early in your application's lifecycle (for example, in your `main` function).
/// It reads the configuration for the Kafka bootstrap servers and message timeout, then creates and initializes
/// the Kafka producer.
///
/// # Panics
/// - If the Kafka bootstrap servers configuration is not provided.
/// - If the producer fails to be created.
/// - If the global producer is already initialized.
pub async fn init_kafka_producer() {
    let bootstrap_servers = read_config!("kafka.bootstrap_servers", String)
        .unwrap_or_else(|| "localhost:9092".to_owned());
    let message_timeout_ms: u64 = read_config!("kafka.message_timeout_ms", u64).unwrap_or(5000);

    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", &bootstrap_servers)
        .set("message.timeout.ms", &message_timeout_ms.to_string())
        .create()
        .expect("Failed to create Kafka producer.");

    KAFKA_PRODUCER
        .set(producer)
        .expect("Kafka producer can only be initialized once!");
}

/// Retrieves a reference to the global Kafka producer.
///
/// # Panics
/// Panics if `init_kafka_producer` has not been called, as the producer will not be initialized.
///
/// # Returns
/// A reference to the initialized `FutureProducer`.
pub fn get_kafka_producer() -> &'static FutureProducer {
    KAFKA_PRODUCER
        .get()
        .expect("Kafka producer not initialized! Call init_kafka_producer first.")
}

/// Asynchronously produces a message to the specified Kafka topic.
///
/// # Arguments
/// * `topic` - The Kafka topic to send the message to.
/// * `key` - The key associated with the message (useful for partitioning).
/// * `payload` - The message payload as a string slice.
///
/// # Example
/// ```rust
/// # async fn example() {
///     use zirv_kafka::produce_message;
///     produce_message("contact-updated", "contact-123", "Contact information updated").await;
/// # }
/// ```
///
/// This function flushes the message (with a 1‑second timeout) and logs the result.
pub async fn produce_message(topic: &str, key: &str, payload: &str) {
    let producer = get_kafka_producer();
    let record: FutureRecord<'_, _, _> = FutureRecord::to(topic)
        .payload(payload)
        .key(key);

    match producer.send(record, std::time::Duration::from_secs(1)).await {
        Ok(delivery) => match delivery {
            Ok((_partition, _offset)) => println!("Message delivered successfully to topic '{}'.", topic),
            Err((e, _)) => eprintln!("Error delivering message: {:?}", e),
        },
        Err(e) => eprintln!("Failed to await message delivery: {:?}", e),
    }
}
