use crate::bindings::*;
use crate::zrdds_interface::publisher::Publisher;
use crate::zrdds_interface::subscriber::Subscriber;
use crate::zrdds_interface::topic::Topic;
use std::ffi::CString;
use std::marker::PhantomData;
pub struct DPDomainParticipant {
    pub(crate) raw: *mut DDS_DomainParticipant,
}

impl DPDomainParticipant {
    /** 该方法在域参与者下创建一个主题子实体，并设置域内唯一的名称、关联的数据类型、QoS以及监听器，用于抽象域内的数据。

    成功返回Some()，失败返回None
    */
    pub fn create_topic(
        &self,
        self_: &DPDomainParticipant,
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
                    self_.raw,
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
        self_: &DPDomainParticipant,
        qoslist: *const DDS_SubscriberQos,
        listener: *mut DDS_SubscriberListener,
        mask: u32,
    ) -> Option<Subscriber> {
        let subscriber =
            unsafe { DDS_DomainParticipant_create_subscriber(self_.raw, qoslist, listener, mask) };

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
        self_: &DPDomainParticipant,
        qoslist: *const DDS_PublisherQos,
        listener: *mut DDS_PublisherListener,
        mask: u32,
    ) -> Option<Publisher> {
        let publisher =
            unsafe { DDS_DomainParticipant_create_publisher(self_.raw, qoslist, listener, mask) };

        if publisher.is_null() {
            None
        } else {
            Some(Publisher {
                raw: publisher,
                _marker: PhantomData,
            })
        }
    }

    /** 取用户实体发现的内置订阅者。 获取到的内置实体不应该删除，否则造成系统异常。

    返回内置订阅者的指针，ZRDDS的实现中，只要域参与者未被删除，那么返回值一定有效。
    */
    pub fn builtin_subscriber(self_: &DPDomainParticipant) -> Subscriber {
        Subscriber {
            raw: unsafe { DDS_DomainParticipant_get_builtin_subscriber(self_.raw) },
            _marker: PhantomData,
        }
    }
}
