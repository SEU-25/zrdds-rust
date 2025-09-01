//! Domain management module
//!
//! This module contains the core domain-related functionality for DDS,
//! including domain participants and domain participant factories.

pub mod factory;
pub mod participant;

pub use factory::*;
pub use participant::*;