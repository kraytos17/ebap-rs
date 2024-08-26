# Async Event Handling System in Rust

This project implements an asynchronous event handling system in Rust using Tokio. The system supports different event types and allows listeners to register for and react to specific events, such as file reads and network requests.

## Key Features

- **Asynchronous Processing**: Uses Tokio for non-blocking event handling.
- **Custom Events**: Implement the `Event` trait to define new event types.
- **Dynamic Listeners**: Register listeners that respond to specific events in real-time.
- **Concurrency**: Designed to handle multiple events concurrently.

## Overview

### Event Trait

The core `Event` trait defines the interface for all events:

```rust
pub trait Event: std::fmt::Debug + Send + Sync + 'static {
    fn event_type(&self) -> &'static str;
    fn as_any(&self) -> &dyn std::any::Any;
}

```
