//! Domain management module
//!
//! This module contains the core domain-related functionality for DDS,
//! including domain participants and domain participant factories.

pub mod dp_listener;
pub mod dp_qos;
pub mod factory;
pub mod participant;

pub use crate::bindings::DDS_StatusKindMask;
pub use dp_listener::*;
pub use dp_qos::*;
pub use factory::*;
pub use participant::*;

pub type StatusKindMask = DDS_StatusKindMask;
