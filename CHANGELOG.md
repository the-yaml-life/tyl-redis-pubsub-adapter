# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial Redis PubSub adapter implementation
- Complete EventPublisher trait implementation
- Basic EventSubscriber trait implementation
- Integration with existing TYL Redis adapter via composition
- Comprehensive test suite with TDD approach
- Integration tests with other TYL modules
- Complete documentation and examples

## [0.1.0] - 2024-12-XX

### Added
- Initial release of TYL Redis PubSub adapter
- **EventPublisher** implementation with:
  - `publish()` - Single event publishing
  - `publish_with_key()` - Publishing with partition keys
  - `publish_batch()` - Batch publishing using Redis pipelines
  - `publish_transactional()` - Transactional publishing with MULTI/EXEC
  - `publish_event()` - Publishing with complete metadata control
- **EventSubscriber** basic implementation with:
  - `subscribe()` - Basic subscription with event handlers
  - `unsubscribe()` - Subscription cancellation
  - Subscription lifecycle management
- **RedisPubSubAdapter** - Main adapter using composition pattern
- Redis-specific error handling integration with TYL error system
- Performance optimizations:
  - Redis pipeline support for batch operations
  - Redis transactions for atomic publishing
  - Connection pooling via existing Redis adapter
- **Composition Pattern** - Reuses `tyl-redis-adapter` without modification
- **Type Safety** - Full Rust type system integration
- **Observability** - Integration with TYL logging and tracing
- Comprehensive examples:
  - Basic publishing patterns
  - Batch and transactional operations
  - Error handling scenarios
- **Testing Infrastructure**:
  - Unit tests for all public methods
  - Integration tests with TYL framework modules
  - Doc tests for documentation examples
  - CI/CD with Redis service containers
- **Documentation**:
  - Complete README with usage examples
  - API documentation with doc comments
  - CLAUDE.md for Claude Code context
  - Architecture documentation

### Technical Details
- **Dependencies**:
  - `tyl-pubsub-port` - PubSub port definitions
  - `tyl-redis-adapter` - Redis connection management (composition)
  - `tyl-errors` - Error handling (via pubsub port)
  - `tyl-config` - Redis configuration
  - `redis` - Redis client library with async support
  - `tokio` - Async runtime
  - `serde` - Serialization support
- **Architecture**:
  - Hexagonal architecture with ports and adapters
  - Composition over inheritance design pattern
  - Clean separation of concerns
  - Extensible without core modifications

### Security
- Security audit integrated in CI pipeline
- No known vulnerabilities
- AGPL-3.0 license compliance
- Secure error handling without information leakage

[Unreleased]: https://github.com/the-yaml-life/tyl-redis-pubsub-adapter/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/the-yaml-life/tyl-redis-pubsub-adapter/releases/tag/v0.1.0