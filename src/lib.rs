pub mod consumer;
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
///     // Use producer directly...
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
/// Wraps a call to `$crate::producer::produce_message(topic, key, payload)`.
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

/// Macro to initialize a BaseConsumer with default settings and spawn its polling thread.
/// Returns the thread JoinHandle.
///
/// # Example
///
/// ```rust
/// use zirv_kafka::start_base_consumer;
///
/// // This spawns a thread that polls the consumer and prints each message payload.
/// let _handle = start_base_consumer!("my-group", &["test-topic"], |msg| {
///     match msg {
///         Ok(m) => println!("Received: {:?}", m),
///         Err(e) => eprintln!("Error: {:?}", e),
///     }
/// });
/// // The thread will run in the background.
/// ```
#[macro_export]
macro_rules! start_base_consumer {
    ($group:expr, $topics:expr, $handler:expr) => {{
        let consumer = $crate::consumer::init_base_consumer($group, $topics)
            .expect("Consumer initialization failed");
        let arc_consumer = std::sync::Arc::new(consumer);
        $crate::consumer::start_consumer_thread(arc_consumer, $handler)
    }};
}
