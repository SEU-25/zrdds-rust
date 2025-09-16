use crate::bindings::*;
use crate::core::domain::DomainParticipant;

use crate::core::publication::Writer;
use crate::core::topic::Topic;
use crate::core::writer_listener::WriterListener;
use crate::core::{PublisherQos, ReturnCode, WriterQos};
use std::marker::PhantomData;

pub struct Publisher {
    pub raw: *mut DDS_Publisher,
    // pub _marker: PhantomData<&'a DomainParticipant>,
}

impl Publisher {
    /** 获取默认qos
     */
    pub fn default_qos() -> PublisherQos {
        PublisherQos::default_qos()
    }

    /** 创建DataWriter。

    成功返回Some()，失败返回None
    */
    pub fn create_writer(
        &self,
        topic: &Topic,
        writer_qos: &WriterQos,
        writer_listener: &mut WriterListener,
        mask: u32,
    ) -> Option<Writer> {
        let writer = Writer {
            raw: unsafe {
                DDS_Publisher_create_datawriter(
                    self.raw,
                    topic.raw,
                    writer_qos.as_ptr(),
                    writer_listener.as_ptr_mut(),
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

    pub fn publisher_get_default_writer_qos(&self, writer: &mut WriterQos) -> ReturnCode {
        ReturnCode::from(unsafe {
            DDS_Publisher_get_default_datawriter_qos(self.raw, writer.as_mut_ptr())
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;

    #[test]
    fn test_publisher_creation() {
        let publisher = Publisher {
            raw: ptr::null_mut(),
        };
        assert!(publisher.raw.is_null());
    }

    #[test]
    fn test_publisher_with_valid_pointer() {
        let mut mock_publisher = std::mem::MaybeUninit::<DDS_Publisher>::uninit();
        let publisher = Publisher {
            raw: mock_publisher.as_mut_ptr(),
        };
        assert!(!publisher.raw.is_null());
    }

    #[test]
    fn test_publisher_null_safety() {
        let publisher = Publisher {
            raw: ptr::null_mut(),
        };
        
        // 验证空指针不会导致程序崩溃
        assert!(publisher.raw.is_null());
        
        // 测试多个实例
        let mut publishers = Vec::new();
        for _ in 0..5 {
            publishers.push(Publisher {
                raw: ptr::null_mut(),
            });
        }
        
        for publisher in &publishers {
            assert!(publisher.raw.is_null());
        }
    }

    #[test]
    fn test_publisher_memory_layout() {
        let publisher1 = Publisher {
            raw: ptr::null_mut(),
        };
        let publisher2 = Publisher {
            raw: ptr::null_mut(),
        };
        
        // 两个不同的实例应该有不同的内存地址
        assert_ne!(&publisher1 as *const _, &publisher2 as *const _);
        
        // 但它们的 raw 字段值应该相同（都是 null）
        assert_eq!(publisher1.raw, publisher2.raw);
    }

    #[test]
    fn test_publisher_with_different_pointers() {
        let mut mock_publisher1 = std::mem::MaybeUninit::<DDS_Publisher>::uninit();
        let mut mock_publisher2 = std::mem::MaybeUninit::<DDS_Publisher>::uninit();
        
        let publisher1 = Publisher {
            raw: mock_publisher1.as_mut_ptr(),
        };
        let publisher2 = Publisher {
            raw: mock_publisher2.as_mut_ptr(),
        };
        
        // 两个 Publisher 实例应该有不同的原始指针
        assert_ne!(publisher1.raw, publisher2.raw);
        assert!(!publisher1.raw.is_null());
        assert!(!publisher2.raw.is_null());
    }

    #[test]
    fn test_multiple_publishers() {
        let mut publishers = Vec::new();
        
        // 创建多个 Publisher 实例
        for i in 0..10 {
            if i % 2 == 0 {
                publishers.push(Publisher {
                    raw: ptr::null_mut(),
                });
            } else {
                let mut mock_publisher = std::mem::MaybeUninit::<DDS_Publisher>::uninit();
                publishers.push(Publisher {
                    raw: mock_publisher.as_mut_ptr(),
                });
            }
        }
        
        // 验证偶数索引的 Publisher 有空指针
        for (i, publisher) in publishers.iter().enumerate() {
            if i % 2 == 0 {
                assert!(publisher.raw.is_null());
            } else {
                assert!(!publisher.raw.is_null());
            }
        }
    }

    #[test]
    fn test_publisher_struct_size() {
        // 验证 Publisher 结构体的大小符合预期
        assert_eq!(std::mem::size_of::<Publisher>(), std::mem::size_of::<*mut DDS_Publisher>());
    }

    #[test]
    fn test_publisher_default_qos() {
        // 测试默认 QoS 方法不会崩溃
        let _qos = Publisher::default_qos();
        // 这个测试主要验证方法调用不会导致编译错误或运行时崩溃
    }
}
