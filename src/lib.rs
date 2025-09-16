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

pub mod dds_handlers;

pub mod utils;

// Dioxus modules
pub mod dioxus_structs;
pub mod dioxus_app;
// pub mod zrdds;
// pub use zrdds::*;

pub mod core;
pub mod components;