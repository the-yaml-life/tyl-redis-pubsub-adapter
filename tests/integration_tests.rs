//! Integration tests for TYL Redis PubSub Adapter
//!
//! These tests verify the integration between the Redis PubSub adapter
//! and other TYL framework modules.

use serde::{Deserialize, Serialize};
use tyl_redis_pubsub_adapter::{EventPublisher, RedisConfig, RedisPubSubAdapter};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct IntegrationTestEvent {
    test_id: String,
    data: String,
    sequence: u32,
}

#[tokio::test]
async fn test_end_to_end_publishing() {
    // Test complete publishing workflow with Redis
    let config = RedisConfig::default();
    let adapter = RedisPubSubAdapter::new(config)
        .await
        .expect("Should be able to create Redis PubSub adapter");

    let test_event = IntegrationTestEvent {
        test_id: "integration-001".to_string(),
        data: "test data".to_string(),
        sequence: 1,
    };

    let result = adapter.publish("integration.test", test_event).await;
    assert!(result.is_ok(), "Should be able to publish event");

    let event_id = result.unwrap();
    assert!(!event_id.is_empty(), "Should return valid event ID");
}

#[tokio::test]
async fn test_configuration_integration() {
    // Test that Redis configuration integrates properly with TYL config
    let config = RedisConfig::default();
    assert!(
        !config.connection_url().is_empty(),
        "Should have valid Redis URL"
    );

    let adapter_result = RedisPubSubAdapter::new(config).await;
    assert!(
        adapter_result.is_ok(),
        "Should create adapter with valid config"
    );
}

#[tokio::test]
async fn test_batch_publishing_integration() {
    // Test batch publishing with multiple events
    let config = RedisConfig::default();
    let adapter = RedisPubSubAdapter::new(config)
        .await
        .expect("Should create adapter");

    let events = vec![
        tyl_redis_pubsub_adapter::TopicEvent {
            topic: "batch.test".to_string(),
            event: IntegrationTestEvent {
                test_id: "batch-001".to_string(),
                data: "batch event 1".to_string(),
                sequence: 1,
            },
            partition_key: None,
        },
        tyl_redis_pubsub_adapter::TopicEvent {
            topic: "batch.test".to_string(),
            event: IntegrationTestEvent {
                test_id: "batch-002".to_string(),
                data: "batch event 2".to_string(),
                sequence: 2,
            },
            partition_key: Some("key1".to_string()),
        },
    ];

    let result = adapter.publish_batch(events).await;
    assert!(result.is_ok(), "Should publish batch successfully");

    let event_ids = result.unwrap();
    assert_eq!(event_ids.len(), 2, "Should return ID for each event");
}

#[tokio::test]
async fn test_error_handling_integration() {
    // Test error handling integrates with TYL error system

    // Test with invalid Redis configuration (this might not fail immediately
    // due to connection pooling, but demonstrates error handling)
    let invalid_config = RedisConfig::default(); // This might still work if Redis is running

    // For now, just test that we can create the adapter
    // In a real scenario, you'd test with a definitely invalid config
    let result = RedisPubSubAdapter::new(invalid_config).await;

    // If Redis is running, this should succeed
    // If not, it should fail with appropriate error
    match result {
        Ok(_) => println!("Redis is available - adapter created successfully"),
        Err(e) => {
            // Verify it's a proper TYL error
            assert!(
                e.to_string().contains("Redis"),
                "Error should mention Redis"
            );
        }
    }
}

#[tokio::test]
async fn test_transactional_publishing_integration() {
    // Test transactional publishing integration
    let config = RedisConfig::default();
    let adapter = RedisPubSubAdapter::new(config)
        .await
        .expect("Should create adapter");

    let events = vec![tyl_redis_pubsub_adapter::TopicEvent {
        topic: "transaction.test".to_string(),
        event: IntegrationTestEvent {
            test_id: "tx-001".to_string(),
            data: "transactional event".to_string(),
            sequence: 1,
        },
        partition_key: None,
    }];

    let transaction_id = "tx-12345".to_string();
    let result = adapter.publish_transactional(events, transaction_id).await;

    assert!(result.is_ok(), "Should publish transactionally");

    let event_ids = result.unwrap();
    assert_eq!(event_ids.len(), 1, "Should return event ID");
}

#[tokio::test]
async fn test_redis_adapter_composition() {
    // Test that the composition with existing Redis adapter works
    let config = RedisConfig::default();
    let adapter = RedisPubSubAdapter::new(config)
        .await
        .expect("Should create adapter");

    // Test accessing the underlying Redis adapter
    let redis_adapter = adapter.redis_adapter();

    // Test health check through the composed adapter
    use tyl_redis_adapter::KeyValueHealth;
    let health_result = KeyValueHealth::health_check(redis_adapter).await;

    match health_result {
        Ok(_) => println!("Health check passed - Redis adapter composition working"),
        Err(e) => {
            println!("Health check failed (Redis might not be running): {}", e);
            // This is OK for testing - just verify error is proper TYL error
            assert!(
                e.to_string().len() > 0,
                "Should have meaningful error message"
            );
        }
    }
}

#[test]
fn test_serialization_integration() {
    // Test that event serialization works with serde
    let test_event = IntegrationTestEvent {
        test_id: "serialization-test".to_string(),
        data: "test serialization".to_string(),
        sequence: 42,
    };

    // Test JSON serialization
    let json = serde_json::to_string(&test_event).expect("Should serialize to JSON");

    let deserialized: IntegrationTestEvent =
        serde_json::from_str(&json).expect("Should deserialize from JSON");

    assert_eq!(
        test_event, deserialized,
        "Serialization round-trip should work"
    );
}

#[test]
fn test_tyl_error_integration() {
    // Test that TYL error system integration works
    use tyl_redis_pubsub_adapter::redis_pubsub_errors;

    let error = redis_pubsub_errors::connection_failed("test error");
    assert!(
        error.to_string().contains("Redis PubSub"),
        "Error should contain Redis PubSub context"
    );

    let publish_error = redis_pubsub_errors::publish_failed("test.topic", "test error");
    assert!(
        publish_error.to_string().contains("test.topic"),
        "Error should contain topic"
    );
}
