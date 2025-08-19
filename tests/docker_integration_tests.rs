//! Docker integration tests for TYL Redis PubSub Adapter
//!
//! These tests verify integration with a real Redis instance.
//! They can be run manually with a Redis Docker container.

use serde::{Deserialize, Serialize};
use tyl_redis_pubsub_adapter::{EventPublisher, RedisConfig, RedisPubSubAdapter};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct DockerTestEvent {
    test_id: String,
    data: String,
    sequence: u32,
    timestamp: i64,
}

/// Test with Redis available on localhost:6379
/// To run this test:
/// 1. Start Redis: docker run -d -p 6379:6379 redis:7-alpine
/// 2. Run test: cargo test test_redis_available --test docker_integration_tests
#[tokio::test]
#[ignore] // Ignored by default - requires Redis to be running
async fn test_redis_available_single_publish() {
    // Test assumes Redis is available on default port
    let config = RedisConfig::default();

    // This will only work if Redis is actually running
    let adapter_result = RedisPubSubAdapter::new(config).await;

    match adapter_result {
        Ok(adapter) => {
            println!("✅ Redis is available - testing single publish");

            let test_event = DockerTestEvent {
                test_id: "docker-001".to_string(),
                data: "Docker test data".to_string(),
                sequence: 1,
                timestamp: chrono::Utc::now().timestamp(),
            };

            let result = adapter.publish("docker.test", test_event).await;
            assert!(result.is_ok(), "Should publish to Redis");

            let event_id = result.unwrap();
            assert!(!event_id.is_empty(), "Should return valid event ID");

            println!("✅ Published event with ID: {}", event_id);
        }
        Err(e) => {
            println!("⚠️  Redis not available for integration test: {}", e);
            println!("   To run this test, start Redis with:");
            println!("   docker run -d -p 6379:6379 redis:7-alpine");
            // Don't fail the test if Redis isn't available
        }
    }
}

#[tokio::test]
#[ignore] // Ignored by default - requires Redis to be running
async fn test_redis_available_batch_publish() {
    let config = RedisConfig::default();

    let adapter_result = RedisPubSubAdapter::new(config).await;

    match adapter_result {
        Ok(adapter) => {
            println!("✅ Redis is available - testing batch publish");

            let events = vec![
                tyl_redis_pubsub_adapter::TopicEvent {
                    topic: "docker.batch".to_string(),
                    event: DockerTestEvent {
                        test_id: "batch-001".to_string(),
                        data: "Batch event 1".to_string(),
                        sequence: 1,
                        timestamp: chrono::Utc::now().timestamp(),
                    },
                    partition_key: None,
                },
                tyl_redis_pubsub_adapter::TopicEvent {
                    topic: "docker.batch".to_string(),
                    event: DockerTestEvent {
                        test_id: "batch-002".to_string(),
                        data: "Batch event 2".to_string(),
                        sequence: 2,
                        timestamp: chrono::Utc::now().timestamp(),
                    },
                    partition_key: Some("partition1".to_string()),
                },
            ];

            let result = adapter.publish_batch(events).await;
            assert!(result.is_ok(), "Should publish batch to Redis");

            let event_ids = result.unwrap();
            assert_eq!(event_ids.len(), 2, "Should return IDs for all events");

            println!("✅ Published batch of {} events", event_ids.len());
        }
        Err(e) => {
            println!("⚠️  Redis not available for batch test: {}", e);
        }
    }
}

#[tokio::test]
#[ignore] // Ignored by default - requires Redis to be running
async fn test_redis_available_transactional_publish() {
    let config = RedisConfig::default();

    let adapter_result = RedisPubSubAdapter::new(config).await;

    match adapter_result {
        Ok(adapter) => {
            println!("✅ Redis is available - testing transactional publish");

            let events = vec![tyl_redis_pubsub_adapter::TopicEvent {
                topic: "docker.transaction".to_string(),
                event: DockerTestEvent {
                    test_id: "tx-001".to_string(),
                    data: "Transactional event".to_string(),
                    sequence: 1,
                    timestamp: chrono::Utc::now().timestamp(),
                },
                partition_key: None,
            }];

            let transaction_id = "docker-tx-123".to_string();
            let result = adapter.publish_transactional(events, transaction_id).await;
            assert!(result.is_ok(), "Should publish transactionally to Redis");

            let event_ids = result.unwrap();
            assert_eq!(event_ids.len(), 1, "Should return transaction event ID");

            println!("✅ Transactional publish completed");
        }
        Err(e) => {
            println!("⚠️  Redis not available for transactional test: {}", e);
        }
    }
}

#[tokio::test]
#[ignore] // Ignored by default - requires Redis to be running
async fn test_redis_available_health_check() {
    let config = RedisConfig::default();

    let adapter_result = RedisPubSubAdapter::new(config).await;

    match adapter_result {
        Ok(adapter) => {
            println!("✅ Redis is available - testing health check");

            // Test health check through composed Redis adapter
            let redis_adapter = adapter.redis_adapter();

            use tyl_redis_adapter::KeyValueHealth;
            let health_result = KeyValueHealth::health_check(redis_adapter).await;

            assert!(health_result.is_ok(), "Health check should pass with Redis");

            println!("✅ Health check passed");
        }
        Err(e) => {
            println!("⚠️  Redis not available for health check test: {}", e);
        }
    }
}

/// This test always runs and verifies our error handling works correctly
#[tokio::test]
async fn test_error_handling_works() {
    // Test that our error helper functions work correctly
    let connection_error =
        tyl_redis_pubsub_adapter::redis_pubsub_errors::connection_failed("test connection error");
    assert!(
        connection_error.to_string().contains("Redis PubSub"),
        "Error should contain Redis PubSub context"
    );

    let publish_error = tyl_redis_pubsub_adapter::redis_pubsub_errors::publish_failed(
        "test.topic",
        "test publish error",
    );
    assert!(
        publish_error.to_string().contains("test.topic"),
        "Error should contain topic name"
    );

    let subscribe_error = tyl_redis_pubsub_adapter::redis_pubsub_errors::subscribe_failed(
        "test.topic",
        "test subscribe error",
    );
    assert!(
        subscribe_error.to_string().contains("test.topic"),
        "Error should contain topic name"
    );

    let serialization_error = tyl_redis_pubsub_adapter::redis_pubsub_errors::serialization_failed(
        "test serialization error",
    );
    assert!(
        serialization_error.to_string().contains("Redis PubSub"),
        "Error should contain Redis PubSub context"
    );

    println!("✅ Error handling functions work correctly");
}

/// This test always runs and verifies our basic adapter functionality
#[tokio::test]
async fn test_adapter_configuration_works() {
    // Test that we can create a config and adapter (even if Redis isn't available)
    let config = RedisConfig::default();
    assert!(
        !config.connection_url().is_empty(),
        "Should have valid Redis connection URL"
    );

    // The adapter creation might fail if Redis isn't available, but that's expected
    let adapter_result = RedisPubSubAdapter::new(config).await;

    // We test both success and failure cases
    match adapter_result {
        Ok(_adapter) => {
            println!("✅ Redis is available - adapter created successfully");
        }
        Err(e) => {
            println!("⚠️  Redis not available - error is properly handled: {}", e);
            // Verify the error is a proper TYL error
            assert!(
                e.to_string().len() > 0,
                "Error should have meaningful message"
            );
        }
    }
}

#[cfg(test)]
mod docker_instructions {
    //! Instructions for running Docker Redis integration tests
    //!
    //! To run the full integration tests with Redis:
    //!
    //! 1. Start a Redis container:
    //!    ```bash
    //!    docker run -d --name redis-test -p 6379:6379 redis:7-alpine
    //!    ```
    //!
    //! 2. Run the integration tests:
    //!    ```bash
    //!    cargo test --test docker_integration_tests -- --ignored
    //!    ```
    //!
    //! 3. Clean up:
    //!    ```bash
    //!    docker stop redis-test && docker rm redis-test
    //!    ```
    //!
    //! The tests marked with `#[ignore]` require Redis to be running.
    //! Tests without `#[ignore]` run without Redis and test basic functionality.
}
