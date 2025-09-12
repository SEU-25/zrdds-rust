//! Domain management module
//!
//! This module contains the core domain-related functionality for DDS,
//! including domain participants and domain participant factories.

pub mod factory;
pub mod participant;
pub mod dp_qos;
pub mod dp_listener;

pub use factory::*;
pub use participant::*;
pub use dp_qos::*;
use crate::bindings::DDS_StatusKindMask;

pub type StatusKindMask = DDS_StatusKindMask;