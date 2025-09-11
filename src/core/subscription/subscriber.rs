use crate::bindings::*;
use crate::core::domain::DomainParticipant;
use crate::core::subscription::Reader;
use std::ffi::CString;
use std::marker::PhantomData;
use crate::core::ReaderListener;

pub struct Subscriber<'a> {
    pub raw: *mut DDS_Subscriber,
    pub(crate) _marker: PhantomData<&'a DomainParticipant>,
}

impl Subscriber<'_> {
    /** 获取默认qos
     */
    pub fn default_qos() -> DDS_SubscriberQos {
        unsafe { DDS_SUBSCRIBER_QOS_DEFAULT }
    }

    /** 该方法在订阅者下创建一个数据读者子实体，并设置关联的主题、QoS以及监听器。

    成功返回Some()，失败返回None
    */
    pub fn create_reader(
        &self,
        topic: *mut DDS_TopicDescription,
        qos: *const DDS_DataReaderQos,
        listener: &mut ReaderListener,
        mask: u32,
    ) -> Option<Reader> {
        let reader = Reader {
            raw: unsafe { DDS_Subscriber_create_datareader(self.raw, topic, qos, &mut listener.raw, mask) },
            _marker: PhantomData,
        };

        if reader.raw.is_null() {
            None
        } else {
            Some(reader)
        }
    }

    /** 根据主题名查找数据读者。

    如果存在多个满足条件的数据读者，则返回数据读者地址最小的那个。

    返回None表示没有满足条件的数据读者，否则返回相应的数据读者。
    */
    pub fn lookup_reader<'a>(self_: Subscriber, topic_name: &str) -> Option<Reader<'a, 'a>> {
        let topicName = CString::new(topic_name).unwrap();

        let reader = Reader {
            raw: unsafe { DDS_Subscriber_lookup_datareader(self_.raw, topicName.as_ptr()) },
            _marker: PhantomData,
        };

        if reader.raw.is_null() {
            None
        } else {
            Some(reader)
        }
    }
}
