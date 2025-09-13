use crate::bindings::{DDS_BytesTypeSupport_get_type_name, DDS_BytesTypeSupport_register_type, DDS_TypeSupport};
use std::ffi::{CString, c_char};
use crate::core::DomainParticipant;

pub struct TypeSupport {
    pub raw: *mut DDS_TypeSupport,
}

pub fn type_support_get_name() -> String {
    unsafe {
        CString::from_raw(DDS_BytesTypeSupport_get_type_name() as *mut c_char)
            .into_string()
            .unwrap()
    }
}

pub fn type_support_register_type(
    participant: &DomainParticipant,
    type_name: &str,
) -> i32 {
    unsafe { DDS_BytesTypeSupport_register_type(participant.raw, type_name.as_ptr() as *const c_char) }
}
