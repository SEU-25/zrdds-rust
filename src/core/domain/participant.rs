use crate::core::publication::Publisher;
use crate::core::return_code::{DdsResult, ReturnCode};
use crate::core::subscription::Subscriber;
use crate::core::topic::Topic;
use crate::{
    PublisherListener, PublisherQos, SubscriberListener, SubscriberQos, TopicListener, bindings::*,
};
use std::ffi::CString;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::ptr::null_mut;

/// 统一的DomainParticipant结构体，同时支持高级API和底层API
pub struct DomainParticipant {
    pub(crate) raw: *mut DDS_DomainParticipant,
}

pub type DomainParticipantListener = DDS_DomainParticipantListener;

impl DomainParticipant {
    /// 高级API：简化的发布方法
    pub fn publish(
        &self,
        topic_name: &str,
        type_support: *mut DDS_TypeSupport,
        qos_name: &str,
    ) -> crate::core::publication::Writer<'static, 'static> {
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
            crate::core::publication::Writer::new(writer)
        }
    }

    /** 该方法在域参与者下创建一个主题子实体，并设置域内唯一的名称、关联的数据类型、QoS以及监听器，用于抽象域内的数据。

    成功返回Some()，失败返回None
    */
    pub fn create_topic(
        &self,
        topic_name: &str,
        type_name: &str,
        qoslist: *const DDS_TopicQos,
        listener: *mut DDS_TopicListener,
        mask: u32,
    ) -> Option<Topic> {
        let topicName = CString::new(topic_name).unwrap();
        let typeName = CString::new(type_name).unwrap();

        let topic = Topic {
            raw: unsafe {
                DDS_DomainParticipant_create_topic(
                    self.raw,
                    topicName.as_ptr(),
                    typeName.as_ptr(),
                    qoslist,
                    listener,
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
        qoslist: *const DDS_SubscriberQos,
        listener: *mut DDS_SubscriberListener,
        mask: u32,
    ) -> Option<Subscriber> {
        let subscriber =
            unsafe { DDS_DomainParticipant_create_subscriber(self.raw, qoslist, listener, mask) };

        if subscriber.is_null() {
            None
        } else {
            Some(Subscriber {
                raw: subscriber,
                _marker: PhantomData,
            })
        }
    }

    /** 该方法在域参与者下创建一个发布者子实体，并设置QoS以及监听器，表明应用想要向该域内发布数据。

    成功返回Some()，失败返回None
    */
    pub fn create_publisher(
        &self,
        qoslist: &PublisherQos,
        listener: &PublisherListener,
        mask: u32,
    ) -> Option<Publisher> {
        let mut listener_copy = listener.clone();
        let publisher = unsafe {
            DDS_DomainParticipant_create_publisher(
                self.raw,
                qoslist.raw,
                &mut listener_copy as *mut DDS_PublisherListener,
                mask,
            )
        };

        if publisher.is_null() {
            None
        } else {
            Some(Publisher {
                raw: publisher,
                _marker: PhantomData,
            })
        }
    }

    pub fn default_publisher_qos(&self) -> DdsResult<PublisherQos> {
        let qos = PublisherQos { raw: null_mut() };
        let ret = unsafe { DDS_DomainParticipant_get_default_publisher_qos(self.raw, qos.raw) };
        let return_code = ReturnCode::from(ret);
        if return_code.is_ok() {
            Ok(qos)
        } else {
            Err(return_code)
        }
    }

    pub fn default_subscriber_qos(&self) -> DdsResult<SubscriberQos> {
        let qos = SubscriberQos { raw: null_mut() };
        let ret = unsafe { DDS_DomainParticipant_get_default_subscriber_qos(self.raw, qos.raw) };
        let return_code = ReturnCode::from(ret);
        if return_code.is_ok() {
            Ok(qos)
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
            _marker: PhantomData,
        }
    }
}

impl Default for DomainParticipantListener {
    fn default() -> Self {
        Self {
            topiclistener: TopicListener::default(),
            publisherlistener: PublisherListener::default(),
            subscriberlistener: SubscriberListener::default(),
        }
    }
}
