use crate::bindings::*;
use crate::core::publication::Publisher;
use crate::core::publisher_listener::PublisherListener;
use crate::core::return_code::{DdsResult, ReturnCode};
use crate::core::subscriber_listener::SubscriberListener;
use crate::core::subscription::Subscriber;
use crate::core::topic::Topic;
use crate::core::topic_listener::TopicListener;
use crate::core::type_support::TypeSupport;
use crate::core::{PublisherQos, SubscriberQos, TopicQos, Writer};
use std::ffi::CString;
use std::marker::PhantomData;
use std::mem::MaybeUninit;

/// 统一的DomainParticipant结构体，同时支持高级API和底层API
pub struct DomainParticipant {
    pub raw: *mut DDS_DomainParticipant,
}

impl DomainParticipant {
    /// 高级API：简化的发布方法
    pub fn publish(&self, topic_name: &str, type_support: &TypeSupport, qos_name: &str) -> Writer {
        unsafe {
            let topic_name = CString::new(topic_name).unwrap();
            let qos_name = CString::new(qos_name).unwrap();
            let type_support = type_support.raw;
            let writer = DDS_PubTopic(
                self.raw,
                topic_name.as_ptr(),
                type_support,
                qos_name.as_ptr(),
                std::ptr::null_mut(),
            );
            Writer::new(writer)
        }
    }

    /** 该方法在域参与者下创建一个主题子实体，并设置域内唯一的名称、关联的数据类型、QoS以及监听器，用于抽象域内的数据。

    成功返回Some()，失败返回None
    */
    pub fn create_topic(
        &self,
        self_: &DomainParticipant,
        topic_name: &str,
        type_name: &str,
        qoslist: &TopicQos,
        listener: &TopicListener,
        mask: u32,
    ) -> Option<Topic> {
        let topicName = CString::new(topic_name).unwrap();
        let typeName = CString::new(type_name).unwrap();

        let topic = Topic {
            raw: unsafe {
                DDS_DomainParticipant_create_topic(
                    self_.raw,
                    topicName.as_ptr(),
                    typeName.as_ptr(),
                    qoslist.raw,
                    listener.raw,
                    mask,
                )
            },
            _marker: PhantomData,
        };

        if topic.raw.is_null() {
            None
        } else {
            Some(topic)
        }
    }

    /** 该方法在域参与者下创建一个订阅者子实体，并设置QoS以及监听器，表明应用想要向该域内订阅数据。

    成功返回Some()，失败返回None
    */
    pub fn create_subscriber(
        &self,
        self_: &DomainParticipant,
        qoslist: &SubscriberQos,
        listener: &SubscriberListener,
        mask: u32,
    ) -> Option<Subscriber> {
        let subscriber = unsafe {
            DDS_DomainParticipant_create_subscriber(self_.raw, qoslist.raw, listener.raw, mask)
        };

        if subscriber.is_null() {
            None
        } else {
            Some(Subscriber {
                raw: subscriber,
                // _marker: PhantomData,
            })
        }
    }

    /** 该方法在域参与者下创建一个发布者子实体，并设置QoS以及监听器，表明应用想要向该域内发布数据。

    成功返回Some()，失败返回None
    */
    pub fn create_publisher(
        &self,
        self_: &DomainParticipant,
        qoslist: &PublisherQos,
        listener: &PublisherListener,
        mask: u32,
    ) -> Option<Publisher> {
        let publisher = unsafe {
            DDS_DomainParticipant_create_publisher(self_.raw, qoslist.raw, listener.raw, mask)
        };

        if publisher.is_null() {
            None
        } else {
            Some(Publisher {
                raw: publisher,
                // _marker: PhantomData,
            })
        }
    }

    pub fn default_subscriber_qos(&self) -> DdsResult<DDS_SubscriberQos> {
        let mut qos = MaybeUninit::<DDS_SubscriberQos>::uninit();
        let ret =
            unsafe { DDS_DomainParticipant_get_default_subscriber_qos(self.raw, qos.as_mut_ptr()) };
        let return_code = ReturnCode::from(ret);
        if return_code.is_ok() {
            Ok(unsafe { qos.assume_init() })
        } else {
            Err(return_code)
        }
    }

    /** 取用户实体发现的内置订阅者。 获取到的内置实体不应该删除，否则造成系统异常。

    返回内置订阅者的指针，ZRDDS的实现中，只要域参与者未被删除，那么返回值一定有效。
    */
    pub fn builtin_subscriber(self_: &DomainParticipant) -> Subscriber {
        Subscriber {
            raw: unsafe { DDS_DomainParticipant_get_builtin_subscriber(self_.raw) },
            // _marker: PhantomData,
        }
    }
}
