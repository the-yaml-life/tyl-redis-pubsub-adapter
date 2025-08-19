//! Basic usage example for TYL Redis PubSub Adapter
//!
//! This example demonstrates how to use the Redis PubSub adapter
//! for publishing and subscribing to events.

use serde::{Deserialize, Serialize};
use tyl_redis_pubsub_adapter::{EventPublisher, RedisConfig, RedisPubSubAdapter};

#[derive(Debug, Serialize, Deserialize)]
struct UserEvent {
    user_id: String,
    action: String,
    timestamp: i64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== TYL Redis PubSub Basic Usage ===\n");

    // Basic publishing example
    basic_publishing_example().await?;

    // Batch publishing example
    batch_publishing_example().await?;

    // Publishing with keys example
    keyed_publishing_example().await?;

    println!("✅ All examples completed successfully!");
    Ok(())
}

async fn basic_publishing_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Basic Publishing ---");

    // Create Redis PubSub adapter with default configuration
    let config = RedisConfig::default();
    let adapter = RedisPubSubAdapter::new(config).await?;

    // Create a sample event
    let event = UserEvent {
        user_id: "user123".to_string(),
        action: "login".to_string(),
        timestamp: 1234567890,
    };

    // Publish the event
    let event_id = adapter.publish("user.events", event).await?;
    println!("✅ Published event with ID: {}", event_id);

    println!();
    Ok(())
}

async fn batch_publishing_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Batch Publishing ---");

    let config = RedisConfig::default();
    let adapter = RedisPubSubAdapter::new(config).await?;

    // Create multiple events
    let events = vec![
        tyl_redis_pubsub_adapter::TopicEvent {
            topic: "user.events".to_string(),
            event: UserEvent {
                user_id: "user1".to_string(),
                action: "register".to_string(),
                timestamp: 1234567891,
            },
            partition_key: None,
        },
        tyl_redis_pubsub_adapter::TopicEvent {
            topic: "user.events".to_string(),
            event: UserEvent {
                user_id: "user2".to_string(),
                action: "login".to_string(),
                timestamp: 1234567892,
            },
            partition_key: None,
        },
    ];

    // Publish events in batch
    let event_ids = adapter.publish_batch(events).await?;
    println!("✅ Published batch of {} events", event_ids.len());
    for (i, event_id) in event_ids.iter().enumerate() {
        println!("  Event {}: {}", i + 1, event_id);
    }

    println!();
    Ok(())
}

async fn keyed_publishing_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Publishing with Partition Keys ---");

    let config = RedisConfig::default();
    let adapter = RedisPubSubAdapter::new(config).await?;

    // Publish event with partition key for ordering
    let event = UserEvent {
        user_id: "user456".to_string(),
        action: "purchase".to_string(),
        timestamp: 1234567893,
    };

    let event_id = adapter
        .publish_with_key("order.events", "user456", event)
        .await?;
    println!("✅ Published keyed event with ID: {}", event_id);

    println!();
    Ok(())
}
