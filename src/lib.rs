pub mod bindings;

#[cfg(feature = "egui")]
pub mod structs;
#[cfg(feature = "egui")]
pub mod dds_handlers;
#[cfg(feature = "egui")]
pub mod utils;
#[cfg(feature = "egui")]
pub mod app;

// Dioxus modules
pub mod dioxus_structs;
pub mod dioxus_app;

// pub mod zrdds;
// pub use zrdds::*;
