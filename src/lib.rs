pub mod producer;

#[macro_export]
/// Macro to asynchronously initialize the global Kafka producer.
///
/// # Example
/// ```rust
/// use zirv_kafka::init_kafka_producer;
///
/// #[tokio::main]
/// async fn main() {
///     init_kafka_producer!();
///     // Your application logic...
/// }
/// ```
macro_rules! init_kafka_producer {
    () => {
        $crate::producer::init_kafka_producer().await;
    };
}

#[macro_export]
/// Macro to retrieve a reference to the global Kafka producer.
///
/// # Example
/// ```rust
/// use zirv_kafka::get_kafka_producer;
///
/// fn produce() {
///     let producer = get_kafka_producer!();
///     // Use `producer` directly if needed...
/// }
/// ```
macro_rules! get_kafka_producer {
    () => {
        $crate::producer::get_kafka_producer()
    };
}

#[macro_export]
/// Macro to produce a Kafka message asynchronously.
///
/// This macro wraps the call to `$crate::kafka::produce_message(topic, key, payload)`. Being asynchronous,
/// it returns a future that should be awaited by the caller.
///
/// # Example
/// ```rust
/// use zirv_kafka::produce_message;
///
/// async fn send_message() {
///     produce_message!("contact-updated", "contact-123", "Update payload").await;
/// }
/// ```
macro_rules! produce_message {
    ($topic:expr, $key:expr, $payload:expr) => {
        $crate::producer::produce_message($topic, $key, $payload)
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use rdkafka::producer::Producer;

    #[tokio::test]
    async fn test_kafka_producer_initialization() {
        // Note: Ensure that your configuration (e.g., via environment variables or a config file)
        // for "kafka.bootstrap_servers" is set before running this test.
        init_kafka_producer!();
        let producer = get_kafka_producer!();
        // Basic check to see if the producer is created. You could also call produce_message if needed.
        assert!(
            producer.in_flight_count() == 0,
            "Producer should be initialized and in a valid state."
        );
    }
}
