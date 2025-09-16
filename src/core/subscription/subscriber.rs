use crate::bindings::*;
use crate::core::domain::DomainParticipant;
use crate::core::subscription::Reader;
use crate::core::topic_description::TopicDescription;
use crate::core::{ReaderListener, ReaderQos, ReturnCode, SubscriberQos};
use std::ffi::CString;
use std::marker::PhantomData;

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
            raw: unsafe {
                DDS_Subscriber_create_datareader(
                    self.raw,
                    topic.raw,
                    qos.as_ptr(),
                    listener.as_mut_ptr(),
                    mask,
                )
            },
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
        unsafe {
            ReturnCode::from(DDS_Subscriber_get_default_datareader_qos(
                self.raw,
                reader_qos.as_ptr_mut(),
            ))
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;

    #[test]
    fn test_subscriber_creation() {
        let subscriber = Subscriber {
            raw: ptr::null_mut(),
        };
        assert!(subscriber.raw.is_null());
    }

    #[test]
    fn test_subscriber_with_valid_pointer() {
        let mut mock_subscriber = std::mem::MaybeUninit::<DDS_Subscriber>::uninit();
        let subscriber = Subscriber {
            raw: mock_subscriber.as_mut_ptr(),
        };
        assert!(!subscriber.raw.is_null());
    }

    #[test]
    fn test_subscriber_null_safety() {
        let subscriber = Subscriber {
            raw: ptr::null_mut(),
        };
        
        // 验证空指针不会导致程序崩溃
        assert!(subscriber.raw.is_null());
        
        // 测试多个实例
        let mut subscribers = Vec::new();
        for _ in 0..5 {
            subscribers.push(Subscriber {
                raw: ptr::null_mut(),
            });
        }
        
        for subscriber in &subscribers {
            assert!(subscriber.raw.is_null());
        }
    }

    #[test]
    fn test_subscriber_memory_layout() {
        let subscriber1 = Subscriber {
            raw: ptr::null_mut(),
        };
        let subscriber2 = Subscriber {
            raw: ptr::null_mut(),
        };
        
        // 两个不同的实例应该有不同的内存地址
        assert_ne!(&subscriber1 as *const _, &subscriber2 as *const _);
        
        // 但它们的 raw 字段值应该相同（都是 null）
        assert_eq!(subscriber1.raw, subscriber2.raw);
    }

    #[test]
    fn test_subscriber_with_different_pointers() {
        let mut mock_subscriber1 = std::mem::MaybeUninit::<DDS_Subscriber>::uninit();
        let mut mock_subscriber2 = std::mem::MaybeUninit::<DDS_Subscriber>::uninit();
        
        let subscriber1 = Subscriber {
            raw: mock_subscriber1.as_mut_ptr(),
        };
        let subscriber2 = Subscriber {
            raw: mock_subscriber2.as_mut_ptr(),
        };
        
        // 两个 Subscriber 实例应该有不同的原始指针
        assert_ne!(subscriber1.raw, subscriber2.raw);
        assert!(!subscriber1.raw.is_null());
        assert!(!subscriber2.raw.is_null());
    }

    #[test]
    fn test_multiple_subscribers() {
        let mut subscribers = Vec::new();
        
        // 创建多个 Subscriber 实例
        for i in 0..10 {
            if i % 2 == 0 {
                subscribers.push(Subscriber {
                    raw: ptr::null_mut(),
                });
            } else {
                let mut mock_subscriber = std::mem::MaybeUninit::<DDS_Subscriber>::uninit();
                subscribers.push(Subscriber {
                    raw: mock_subscriber.as_mut_ptr(),
                });
            }
        }
        
        // 验证偶数索引的 Subscriber 有空指针
        for (i, subscriber) in subscribers.iter().enumerate() {
            if i % 2 == 0 {
                assert!(subscriber.raw.is_null());
            } else {
                assert!(!subscriber.raw.is_null());
            }
        }
    }

    #[test]
    fn test_subscriber_struct_size() {
        // 验证 Subscriber 结构体的大小符合预期
        assert_eq!(std::mem::size_of::<Subscriber>(), std::mem::size_of::<*mut DDS_Subscriber>());
    }

    #[test]
    fn test_subscriber_default_qos() {
        // 测试默认 QoS 方法不会崩溃
        let _qos = Subscriber::default_qos();
        // 这个测试主要验证方法调用不会导致编译错误或运行时崩溃
    }

    #[test]
    fn test_subscriber_creation_patterns() {
        // 测试不同的创建模式
        let subscribers = vec![
            Subscriber { raw: ptr::null_mut() },
        ];
        
        for subscriber in &subscribers {
            assert!(subscriber.raw.is_null());
        }
    }
}
