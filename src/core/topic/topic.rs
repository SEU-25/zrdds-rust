use crate::bindings::*;
use crate::core::domain::DomainParticipant;
use crate::core::topic_description::TopicDescription;

pub struct Topic<'a> {
    pub raw: *mut DDS_Topic,
    pub _marker: std::marker::PhantomData<&'a DomainParticipant>,
}

impl<'a> Topic<'a> {
    pub fn get_description(&self) -> TopicDescription {
        TopicDescription {
            raw: self.raw as *mut _,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;

    #[test]
    fn test_topic_creation() {
        let topic = Topic {
            raw: ptr::null_mut(),
            _marker: std::marker::PhantomData,
        };
        assert!(topic.raw.is_null());
    }

    #[test]
    fn test_topic_with_valid_pointer() {
        let dummy_value = 42u32;
        let topic = Topic {
            raw: &dummy_value as *const u32 as *mut DDS_Topic,
            _marker: std::marker::PhantomData,
        };
        assert!(!topic.raw.is_null());
    }

    #[test]
    fn test_topic_null_safety() {
        let topic = Topic {
            raw: ptr::null_mut(),
            _marker: std::marker::PhantomData,
        };
        // 测试空指针的安全性
        assert!(topic.raw.is_null());
        assert_eq!(topic.raw as usize, 0);
    }

    #[test]
    fn test_topic_memory_layout() {
        let topic = Topic {
            raw: ptr::null_mut(),
            _marker: std::marker::PhantomData,
        };
        // 验证内存布局
        assert_eq!(std::mem::size_of_val(&topic.raw), std::mem::size_of::<*mut DDS_Topic>());
        assert_eq!(std::mem::size_of_val(&topic._marker), 0); // PhantomData 不占用空间
    }

    #[test]
    fn test_topic_with_different_pointers() {
        let dummy1 = 1u32;
        let dummy2 = 2u32;
        
        let topic1 = Topic {
            raw: &dummy1 as *const u32 as *mut DDS_Topic,
            _marker: std::marker::PhantomData,
        };
        
        let topic2 = Topic {
            raw: &dummy2 as *const u32 as *mut DDS_Topic,
            _marker: std::marker::PhantomData,
        };
        
        assert_ne!(topic1.raw, topic2.raw);
    }

    #[test]
    fn test_multiple_topics() {
        let topics: Vec<Topic> = (0..5).map(|i| {
            let dummy = Box::leak(Box::new(i as u32));
            Topic {
                raw: dummy as *mut u32 as *mut DDS_Topic,
                _marker: std::marker::PhantomData,
            }
        }).collect();
        
        assert_eq!(topics.len(), 5);
        for topic in &topics {
            assert!(!topic.raw.is_null());
        }
    }

    #[test]
    fn test_topic_struct_size() {
        let size = std::mem::size_of::<Topic>();
        // Topic 应该只包含一个指针和 PhantomData（不占空间）
        assert_eq!(size, std::mem::size_of::<*mut DDS_Topic>());
    }

    #[test]
    fn test_topic_get_description() {
        let dummy_value = 42u32;
        let topic = Topic {
            raw: &dummy_value as *const u32 as *mut DDS_Topic,
            _marker: std::marker::PhantomData,
        };
        
        let description = topic.get_description();
        assert_eq!(description.raw as *mut DDS_Topic, topic.raw);
    }

    #[test]
    fn test_topic_creation_patterns() {
        // 测试不同的创建模式
        let topic1 = Topic {
            raw: ptr::null_mut(),
            _marker: std::marker::PhantomData,
        };
        
        let dummy = 123u32;
        let topic2 = Topic {
            raw: &dummy as *const u32 as *mut DDS_Topic,
            _marker: std::marker::PhantomData,
        };
        
        assert!(topic1.raw.is_null());
        assert!(!topic2.raw.is_null());
    }
}
