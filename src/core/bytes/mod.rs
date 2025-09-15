pub mod bytes;
pub mod bytes_seq;
pub mod instancehandle_t;

use crate::bindings::DDS_Boolean;
pub use bytes_seq::*;

type Boolean = DDS_Boolean;
