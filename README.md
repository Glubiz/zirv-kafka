# zirv-kafka

A convenient wrapper around the [rdkafka](https://github.com/fede1024/rust-rdkafka) crate that simplifies working with Apache Kafka in Rust applications.

[![crates.io](https://img.shields.io/crates/v/zirv-kafka.svg)](https://crates.io/crates/zirv-kafka)
[![Documentation](https://docs.rs/zirv-kafka/badge.svg)](https://docs.rs/zirv-kafka)
[![CI Pipeline](https://github.com/Glubiz/zirv-kafka/actions/workflows/ci.yaml/badge.svg)](https://github.com/Glubiz/zirv-kafka/actions/workflows/ci.yaml)
[![License](https://img.shields.io/crates/l/zirv-kafka)](https://github.com/Glubiz/zirv-kafka)

## Features

- Easy Kafka producer initialization and management through global instance
- Simplified message production with convenient macros
- Streamlined consumer setup and message handling
- Background thread management for continuous message polling

## Installation

Add `zirv-kafka` to your `Cargo.toml`:

```toml
[dependencies]
zirv-kafka = "0.2.0"
```

## Producer Usage

### Initialize the producer

Initialize the Kafka producer early in your application:

```rust
use zirv_kafka::init_kafka_producer;

#[tokio::main]
async fn main() {
    init_kafka_producer!().await;
    
    // Your application logic...
}
```

### Send messages

Send messages to Kafka topics using the `produce_message!` macro:

```rust
use zirv_kafka::produce_message;

async fn notify_user_updated(user_id: &str, data: &str) {
    produce_message!("user-updated", user_id, data).await;
}
```

For more direct control, you can access the producer directly:

```rust
use zirv_kafka::get_kafka_producer;
use rdkafka::producer::FutureRecord;

async fn send_custom_message() {
    let producer = get_kafka_producer!();
    let record = FutureRecord::to("my-topic")
        .payload("message content")
        .key("message-key");
    
    let result = producer.send(record, std::time::Duration::from_secs(1)).await;
    // Handle the result...
}
```

## Consumer Usage

### Create a consumer and start processing messages

Use the `start_base_consumer!` macro to initialize a consumer and process messages:

```rust
use zirv_kafka::start_base_consumer;

fn main() {
    // Start a consumer that processes messages from the "user-events" topic
    let handle = start_base_consumer!("user-service", &["user-events"], |msg_result| {
        match msg_result {
            Ok(msg) => {
                if let Some(payload) = msg.payload_view::<str>() {
                    if let Ok(content) = payload {
                        println!("Received message: {}", content);
                        // Process the message...
                    }
                }
            },
            Err(e) => eprintln!("Error while consuming message: {:?}", e),
        }
    });
    
    // The consumer runs in the background
    
    // When you're done with the consumer (optional)
    // handle.join().unwrap();
}
```

For more control over the consumer, you can use the lower-level functions:

```rust
use rdkafka::message::BorrowedMessage;
use std::sync::Arc;
use zirv_kafka::consumer::{init_base_consumer, start_consumer_thread};

fn main() {
    let consumer = init_base_consumer("my-app", &["events-topic"])
        .expect("Failed to create consumer");
    
    let arc_consumer = Arc::new(consumer);
    
    let handle = start_consumer_thread(arc_consumer, |msg| {
        // Custom message handling logic
    });
    
    // Later, when shutting down:
    // handle.join().unwrap();
}
```

## Configuration

By default, the library uses the following configuration:

- `KAFKA_BROKERS` environment variable with a default of "localhost:9092" for consumers
- For producers, it uses [zirv-config](https://crates.io/crates/zirv-config) to read:
  - `kafka.bootstrap_servers` (default: "localhost:9092")
  - `kafka.message_timeout_ms` (default: 5000)

## Requirements

- Rust (2021 edition or later)
- librdkafka development libraries if using dynamic linking

## License

This project is licensed under either:

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)