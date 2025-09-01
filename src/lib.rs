//! ZRDDS - Zero-copy Rust DDS Library
//!
//! This library provides both high-level and low-level APIs for DDS (Data Distribution Service).
//! 
//! # Module Organization
//! 
//! - `api`: High-level, user-friendly API for common DDS operations
//! - `core`: Low-level, comprehensive DDS interface for advanced usage
//! - `bindings`: Raw FFI bindings to the underlying DDS implementation

pub mod bindings;
pub mod api;
pub mod core;

// Re-export the high-level API by default for ease of use
pub use api::*;

// Provide access to core functionality when needed
pub use core as dds_core;
