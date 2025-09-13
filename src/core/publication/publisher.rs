use crate::bindings::*;
use crate::core::domain::DomainParticipant;

use crate::core::publication::Writer;
use crate::core::topic::Topic;
use std::marker::PhantomData;
use crate::core::writer_listener::WriterListener;
use crate::core::WriterQos;

pub struct Publisher<'a> {
    pub raw: *mut DDS_Publisher,
    pub _marker: PhantomData<&'a DomainParticipant>,
}

impl Publisher<'_> {
    /** 获取默认qos
     */
    pub fn default_qos() -> DDS_PublisherQos {
        unsafe { DDS_PUBLISHER_QOS_DEFAULT }
    }

    /** 创建DataWriter。

    成功返回Some()，失败返回None
    */
    pub fn create_writer(
        &self,
        topic: &Topic,
        writer_qos: &WriterQos,
        writer_listener: &WriterListener,
        mask: u32,
    ) -> Option<Writer> {
        let writer = Writer {
            raw: unsafe {
                DDS_Publisher_create_datawriter(
                    self.raw,
                    topic.raw,
                    writer_qos.raw,
                    writer_listener.raw,
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
}
