use super::writer::DataWriter;
use crate::bindings::*;
use std::ffi::CString;
use std::marker::PhantomData;

pub struct DomainParticipant<'a> {
    pub(crate) raw: *mut DDS_DomainParticipant,
    pub(crate) _phantom: PhantomData<&'a ()>,
}

impl<'a> DomainParticipant<'a> {
    pub fn publish(
        &self,
        topic_name: &str,
        type_support: *mut DDS_TypeSupport,
        qos_name: &str,
    ) -> DataWriter {
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
