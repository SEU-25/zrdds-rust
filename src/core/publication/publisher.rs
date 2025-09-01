use crate::bindings::*;
use crate::core::domain::DomainParticipant;

use crate::core::topic::Topic;
use crate::core::publication::Writer;
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
    pub fn create_writer<'a>(
        publisher: Publisher,
        topic: Topic,
        writerQos: *const DDS_DataWriterQos,
        writerListener: *mut DDS_DataWriterListener,
        mask: u32,
    ) -> Option<Writer<'a, 'a>> {
        let writer = Writer {
            raw: unsafe {
                DDS_Publisher_create_datawriter(
                    publisher.raw,
                    topic.raw,
                    writerQos,
                    writerListener,
                    mask,
                )
            },
            _marker: PhantomData,
        };

        if writer.raw.is_null(){
            None
        }else{
            Some(writer)
        }
    }
}
