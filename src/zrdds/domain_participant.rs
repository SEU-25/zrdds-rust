use std::ffi::CString;
use crate::bindings::*;
use super::writer::DataWriter;

pub struct DomainParticipant {
    pub(crate) raw: *mut DDS_DomainParticipant,
}

impl DomainParticipant {
    pub fn publish(&self, topic_name: &str, type_support: *mut DDS_TypeSupport, qos_name: &str) -> DataWriter {
        unsafe {
            let topic_name = CString::new(topic_name).unwrap();
            let qos_name = CString::new(qos_name).unwrap();
            let type_support = type_support as *mut DDS_TypeSupport;
            let writer = DDS_PubTopic(
                self.raw,
                topic_name.as_ptr(),
                type_support,
                qos_name.as_ptr(),
                std::ptr::null_mut(),
            );
            DataWriter { raw: writer }
        }
    }
}
