use crate::bindings::*;
use crate::core::domain::DomainParticipant;

use crate::core::publication::Writer;
use crate::core::topic::Topic;
use crate::core::writer_listener::WriterListener;
use crate::core::{PublisherQos, ReturnCode, WriterQos};
use std::marker::PhantomData;

pub struct Publisher<'a> {
    pub raw: *mut DDS_Publisher,
    pub _marker: PhantomData<&'a DomainParticipant>,
}

impl Publisher<'_> {
    /** 获取默认qos
     */
    pub fn default_qos() -> PublisherQos {
        PublisherQos::default_qos()
    }

    /** 创建DataWriter。

    成功返回Some()，失败返回None
    */
    pub fn create_writer(
        &self,
        topic: &Topic,
        writer_qos: &WriterQos,
        writer_listener: &mut WriterListener,
        mask: u32,
    ) -> Option<Writer> {
        let writer = Writer {
            raw: unsafe {
                DDS_Publisher_create_datawriter(
                    self.raw,
                    topic.raw,
                    writer_qos.as_ptr(),
                    writer_listener.as_ptr_mut(),
                    mask,
                )
            },
        };

        if writer.raw.is_null() {
            None
        } else {
            Some(writer)
        }
    }

    pub fn publisher_get_default_writer_qos(&self, writer: &mut WriterQos) -> ReturnCode {
        ReturnCode::from(unsafe {
            DDS_Publisher_get_default_datawriter_qos(self.raw, writer.as_mut_ptr())
        })
    }
}
