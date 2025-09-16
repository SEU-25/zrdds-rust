//! Rust bindings for DDS Library provided by ZRTech
//!
//! This library provides both high-level and low-level APIs for DDS (Data Distribution Service).
//!
//! # Module Organization
//!
//! - `api`: High-level, user-friendly API for common DDS operations
//! - `core`: Low-level, comprehensive DDS interface for advanced usage
//! - `bindings`: Raw FFI bindings to the underlying DDS implementation

pub mod bindings;

pub mod utils;
pub mod core;
