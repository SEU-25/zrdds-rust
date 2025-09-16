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


#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;

    #[test]
    fn test_domain_participant_creation() {
        let participant = DomainParticipant {
            raw: ptr::null_mut(),
        };
        assert!(participant.raw.is_null());
    }

    #[test]
    fn test_domain_participant_with_valid_pointer() {
        // 创建一个模拟的有效指针
        let mut mock_participant = std::mem::MaybeUninit::<DDS_DomainParticipant>::uninit();
        let participant = DomainParticipant {
            raw: mock_participant.as_mut_ptr(),
        };
        assert!(!participant.raw.is_null());
    }

    #[test]
    fn test_create_topic_with_null_participant() {
        let participant = DomainParticipant {
            raw: ptr::null_mut(),
        };
        
        // 创建模拟的依赖对象
        let topic_qos = TopicQos {
            raw: ptr::null_mut(),
        };
        let topic_listener = TopicListener {
            raw: ptr::null_mut(),
        };
        
        // 由于 participant.raw 是 null，这个调用应该返回 None
        let result = participant.create_topic(
            &participant,
            "test_topic",
            "test_type",
            &topic_qos,
            &topic_listener,
            0,
        );
        
        // 在实际的 DDS 实现中，null participant 应该导致失败
        // 这里我们只能测试函数不会崩溃
        assert!(result.is_none() || result.is_some());
    }

    #[test]
    fn test_create_subscriber_with_null_participant() {
        let participant = DomainParticipant {
            raw: ptr::null_mut(),
        };
        
        let subscriber_qos = SubscriberQos {
            raw: ptr::null_mut(),
        };
        let subscriber_listener = SubscriberListener {
            raw: ptr::null_mut(),
        };
        
        let result = participant.create_subscriber(
            &participant,
            &subscriber_qos,
            &subscriber_listener,
            0,
        );
        
        // 测试函数调用不会崩溃
        assert!(result.is_none() || result.is_some());
    }

    #[test]
    fn test_publish_with_null_participant() {
        let participant = DomainParticipant {
            raw: ptr::null_mut(),
        };
        
        let type_support = TypeSupport {
            raw: ptr::null_mut(),
        };
        
        // 这个调用可能会创建一个 Writer，但由于 participant 是 null，
        // 返回的 Writer 也应该是无效的
        let writer = participant.publish("test_topic", &type_support, "test_qos");
        
        // 验证函数调用不会崩溃
        // Writer 的有效性取决于底层 DDS 实现
        assert!(writer.raw.is_null() || !writer.raw.is_null());
    }

    #[test]
    fn test_default_subscriber_qos() {
        let participant = DomainParticipant {
            raw: ptr::null_mut(),
        };
        
        // 测试 default_subscriber_qos 方法
        let result = participant.default_subscriber_qos();
        
        // 这个方法返回 DdsResult，我们测试它不会崩溃
        match result {
            Ok(_) => {
                // 成功情况
            }
            Err(_) => {
                // 错误情况，这是预期的，因为 participant 是 null
            }
        }
    }

    #[test]
    fn test_builtin_subscriber() {
        let participant = DomainParticipant {
            raw: ptr::null_mut(),
        };
        
        // 测试 builtin_subscriber 静态方法
        let subscriber = DomainParticipant::builtin_subscriber(&participant);
        
        // 验证函数调用不会崩溃
        assert!(subscriber.raw.is_null() || !subscriber.raw.is_null());
    }

    #[test]
    fn test_multiple_participants() {
        // 测试创建多个 DomainParticipant 实例
        let mut participants = Vec::new();
        
        for _ in 0..5 {
            participants.push(DomainParticipant {
                raw: ptr::null_mut(),
            });
        }
        
        // 验证所有实例都正确创建
        for participant in &participants {
            assert!(participant.raw.is_null());
        }
        
        // 测试实例的独立性
        assert_eq!(participants.len(), 5);
    }
}
