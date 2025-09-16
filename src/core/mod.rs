//! Core DDS functionality
//!
//! This module provides the low-level, core DDS interfaces organized by functionality.
//! It serves as the foundation for higher-level APIs.

pub mod bytes;
mod chat_message;
pub mod domain;
pub mod publication;
pub mod return_code;
pub mod sample;
pub mod subscription;
pub mod topic;
pub mod type_support;

#[cfg(test)]
mod tests;

// Re-export all public items from submodules
pub use domain::*;
pub use publication::*;
pub use return_code::*;
pub use subscription::*;
pub use topic::*;
