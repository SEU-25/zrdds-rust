use crate::bindings::*;
use crate::core::domain::DomainParticipant;
use crate::core::subscription::Reader;
use std::ffi::CString;
use std::marker::PhantomData;
use crate::core::{ReaderListener, ReaderQos, ReturnCode, SubscriberQos};
use crate::core::topic_description::TopicDescription;

pub struct Subscriber {
    pub raw: *mut DDS_Subscriber,
    // pub _marker: PhantomData<&'a DomainParticipant>,
}

impl Subscriber {
    /** 获取默认qos
     */
    pub fn default_qos() -> SubscriberQos {
        SubscriberQos::default_qos()
    }

    /** 该方法在订阅者下创建一个数据读者子实体，并设置关联的主题、QoS以及监听器。

    成功返回Some()，失败返回None
    */
    pub fn create_reader(
        &self,
        topic: &TopicDescription,
        qos: &ReaderQos,
        listener: &mut ReaderListener,
        mask: u32,
    ) -> Option<Reader> {
        let reader = Reader {
            raw: unsafe { DDS_Subscriber_create_datareader(self.raw, topic.raw, qos.as_ptr(), listener.as_mut_ptr(), mask) },
            // _marker: PhantomData,
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
    pub fn lookup_reader<'a>(self_: Subscriber, topic_name: &str) -> Option<Reader> {
        let topicName = CString::new(topic_name).unwrap();

        let reader = Reader {
            raw: unsafe { DDS_Subscriber_lookup_datareader(self_.raw, topicName.as_ptr()) },
        //  _marker: PhantomData,
        };

        if reader.raw.is_null() {
            None
        } else {
            Some(reader)
        }
    }

    pub fn subscriber_get_default_reader_qos(&self, reader_qos: &mut ReaderQos) -> ReturnCode {
        unsafe { ReturnCode::from(DDS_Subscriber_get_default_datareader_qos(self.raw, reader_qos.as_ptr_mut())) }
    }
    
}
