use crate::bindings::*;
use crate::core::domain::DomainParticipant;

use crate::core::publication::Writer;
use crate::core::topic::Topic;
use std::marker::PhantomData;

pub struct Publisher<'a> {
    pub(crate) raw: *mut DDS_Publisher,
    pub(crate) _marker: PhantomData<&'a DomainParticipant>,
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
        writer_qos: *const DDS_DataWriterQos,
        writer_listener: *mut DDS_DataWriterListener,
        mask: u32,
    ) -> Option<Writer> {
        let writer = Writer {
            raw: unsafe {
                DDS_Publisher_create_datawriter(
                    self.raw,
                    topic.raw,
                    writer_qos,
                    writer_listener,
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
