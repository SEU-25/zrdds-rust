use crate::bindings::{DDS_BytesTypeSupport_get_type_name, DDS_BytesTypeSupport_register_type};
use std::ffi::{CString, c_char};
use crate::core::DomainParticipant;

pub fn typeSupport_get_name() -> String {
    unsafe {
        CString::from_raw(DDS_BytesTypeSupport_get_type_name() as *mut c_char)
            .into_string()
            .unwrap()
    }
}

pub fn typeSupport_register_type(
    participant: &DomainParticipant,
    typeName: &str,
) -> i32 {
    unsafe { DDS_BytesTypeSupport_register_type(participant.raw, typeName.as_ptr() as *const c_char) }
}
