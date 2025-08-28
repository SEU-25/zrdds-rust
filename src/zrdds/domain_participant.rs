use super::writer::DataWriter;
use super::reader::DataReader;
use crate::bindings::*;
use std::ffi::CString;

pub struct DomainParticipant {
    pub(crate) raw: *mut DDS_DomainParticipant,
}

impl DomainParticipant {
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

    pub fn subscribe(
        &self,
        topic_name: &str,
        type_support: *mut DDS_TypeSupport,
        qos_name: &str,
    ) -> DataReader {
        unsafe {
            let topic_name = std::ffi::CString::new(topic_name).unwrap();
            let qos_name = std::ffi::CString::new(qos_name).unwrap();

            let type_support = type_support as *mut DDS_TypeSupport;
            let reader = DDS_SubTopic(
                self.raw,
                topic_name.as_ptr(),
                type_support,
                qos_name.as_ptr(),
                std::ptr::null_mut(), // drListener 传 NULL
            );

            if reader.is_null() {
                panic!("订阅主题失败: {}", topic_name.to_string_lossy());
            }

            DataReader { raw: reader }
        }
    }

    pub fn subscribe_with_listener(
        &self,
        topic_name: &str,
        type_support: *mut DDS_TypeSupport,
        qos_name: &str,
        listener: &mut DDS_DataReaderListener,
    ) -> DataReader {
        unsafe {
            let topic_name = std::ffi::CString::new(topic_name).unwrap();
            let qos_name = std::ffi::CString::new(qos_name).unwrap();

            let reader = DDS_SubTopic(
                self.raw,
                topic_name.as_ptr(),
                type_support,
                qos_name.as_ptr(),
                listener as *mut DDS_DataReaderListener,
            );

            if reader.is_null() {
                panic!("订阅主题失败: {}", topic_name.to_string_lossy());
            }

            DataReader { raw: reader }
        }
    }
}
