# CLAUDE.md - tyl-redis-pubsub-adapter

## 📋 **Module Context**

**tyl-redis-pubsub-adapter** is the Redis PubSub adapter implementation for the TYL framework. This module provides a production-ready Redis PubSub adapter that implements the TYL PubSub port using Redis as the underlying message broker.

## 🏗️ **Architecture**

### **Port (Interface)**
This module implements the TYL PubSub port traits:
```rust
trait EventPublisher {
    async fn publish<T>(&self, topic: &str, event: T) -> PubSubResult<EventId>;
    async fn publish_with_key<T>(&self, topic: &str, key: &str, event: T) -> PubSubResult<EventId>;
    async fn publish_batch<T>(&self, events: Vec<TopicEvent<T>>) -> PubSubResult<Vec<EventId>>;
    async fn publish_transactional<T>(&self, events: Vec<TopicEvent<T>>, transaction_id: TransactionId) -> PubSubResult<Vec<EventId>>;
    async fn publish_event<T>(&self, event: Event<T>) -> PubSubResult<EventId>;
}

trait EventSubscriber {
    async fn subscribe<T>(&self, topic: &str, handler: Box<dyn EventHandler<T>>) -> PubSubResult<SubscriptionId>;
    async fn unsubscribe(&self, subscription_id: SubscriptionId) -> PubSubResult<()>;
    // ... more methods
}
```

### **Adapters (Implementations)**
- `RedisPubSubAdapter` - Redis implementation using native Redis PUBLISH/SUBSCRIBE commands
- Composition with `tyl-redis-adapter` for connection management

### **Core Types**
- `RedisPubSubAdapter` - Main adapter implementing PubSub traits
- `PubSubResult<T>` - Result type alias from tyl-pubsub-port
- `TylError` - Error types from tyl-errors via tyl-pubsub-port
- `RedisConfig` - Configuration from tyl-config

## 🧪 **Testing**

```bash
cargo test -p tyl-redis-pubsub-adapter
cargo test --doc -p tyl-redis-pubsub-adapter
cargo run --example basic_usage -p tyl-redis-pubsub-adapter
```

## 📂 **File Structure**

```
tyl-redis-pubsub-adapter/
├── src/lib.rs                 # Core implementation with Redis PubSub adapter
├── examples/
│   └── basic_usage.rs         # Complete publishing examples
├── tests/
│   └── integration_tests.rs   # Integration tests with TYL modules
├── README.md                  # Main documentation
├── CLAUDE.md                  # This file
└── Cargo.toml                 # Package metadata
```

## 🔧 **How to Use**

### **Basic Publishing**
```rust
use tyl_redis_pubsub_adapter::{RedisPubSubAdapter, EventPublisher, RedisConfig};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct UserEvent {
    user_id: String,
    action: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = RedisConfig::default();
    let adapter = RedisPubSubAdapter::new(config).await?;
    
    let event = UserEvent {
        user_id: "123".to_string(),
        action: "login".to_string(),
    };
    
    let event_id = adapter.publish("user.events", event).await?;
    println!("Published event: {}", event_id);
    
    Ok(())
}
```

### **Batch Publishing**
```rust
use tyl_redis_pubsub_adapter::TopicEvent;

let events = vec![
    TopicEvent {
        topic: "user.events".to_string(),
        event: UserEvent { /* ... */ },
        partition_key: None,
    },
    // ... more events
];

let event_ids = adapter.publish_batch(events).await?;
```

### **Transactional Publishing**
```rust
let transaction_id = "tx-123".to_string();
let event_ids = adapter.publish_transactional(events, transaction_id).await?;
```

## 🛠️ **Useful Commands**

```bash
cargo clippy -p tyl-redis-pubsub-adapter
cargo fmt -p tyl-redis-pubsub-adapter  
cargo doc --no-deps -p tyl-redis-pubsub-adapter --open
cargo test -p tyl-redis-pubsub-adapter --verbose
```

## 📦 **Dependencies**

### **Runtime**
- `tyl-errors` - Error handling (via tyl-pubsub-port)
- `tyl-config` - Redis configuration
- `tyl-logging` - Structured logging
- `tyl-tracing` - Distributed tracing
- `tyl-pubsub-port` - PubSub port traits and types
- `tyl-redis-adapter` - Redis connection management
- `redis` - Redis client library
- `tokio` - Async runtime
- `futures-util` - Stream utilities
- `serde` - Serialization support
- `serde_json` - JSON handling
- `async-trait` - Async trait support

### **Development**
- `tokio-test` - Async testing utilities
- `testcontainers` - Integration testing with Redis containers

## 🎯 **Design Principles**

1. **Hexagonal Architecture** - Clean separation of concerns with ports and adapters
2. **Composition over Inheritance** - Reuses existing Redis adapter without modification
3. **TDD Approach** - Tests written first to define expected behavior
4. **Error Handling** - Comprehensive error types with context using TYL error system
5. **Performance** - Uses Redis pipelines and transactions for batch operations
6. **Observability** - Integration with TYL logging and tracing

## ⚠️ **Known Limitations**

- Redis PubSub doesn't have native consumer groups (unlike Redis Streams)
- Subscription management is basic - full implementation would need more sophisticated state tracking
- Some EventSubscriber methods are partially implemented (pause/resume)
- Redis connection pooling handled by underlying tyl-redis-adapter

## 📝 **Notes for Contributors**

- Follow TDD approach - write tests first
- Maintain hexagonal architecture
- Use composition with existing TYL modules
- Don't duplicate functionality - extend existing adapters
- Document all public APIs with examples
- Add integration tests for new features
- Use Redis-specific optimizations (pipelines, transactions)

## 🔗 **Related TYL Modules**

- [`tyl-pubsub-port`](https://github.com/the-yaml-life/tyl-pubsub-port) - PubSub port definitions
- [`tyl-redis-adapter`](https://github.com/the-yaml-life/tyl-redis-adapter) - Redis connection management
- [`tyl-errors`](https://github.com/the-yaml-life/tyl-errors) - Error handling
- [`tyl-config`](https://github.com/the-yaml-life/tyl-config) - Configuration management
- [`tyl-logging`](https://github.com/the-yaml-life/tyl-logging) - Structured logging
- [`tyl-tracing`](https://github.com/the-yaml-life/tyl-tracing) - Distributed tracing

## 🏗️ **Implementation Details**

### **Composition Pattern**
The adapter uses composition to leverage existing TYL infrastructure:
```rust
pub struct RedisPubSubAdapter {
    // Composition: reuse existing Redis adapter for basic operations
    redis_adapter: RedisAdapter,
    // Redis client for additional PubSub connections
    redis_client: Client,
    // Track active subscriptions
    active_subscriptions: Arc<Mutex<HashMap<SubscriptionId, ActiveSubscription>>>,
}
```

### **Redis Commands Used**
- `PUBLISH` - For event publishing
- `SUBSCRIBE` / `PSUBSCRIBE` - For event subscription
- Pipelines - For batch operations
- `MULTI/EXEC` - For transactional publishing

### **Error Handling Integration**
All errors are properly converted to TYL error types:
```rust
pub mod redis_pubsub_errors {
    pub fn connection_failed(msg: impl Into<String>) -> TylError {
        TylError::network(format!("Redis PubSub: {}", msg.into()))
    }
    
    pub fn publish_failed(topic: &str, msg: impl Into<String>) -> TylError {
        TylError::database(format!("Redis PubSub publish to '{}' failed: {}", topic, msg.into()))
    }
    
    // ... more error helpers
}
```