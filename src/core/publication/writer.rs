use crate::bindings::*;
use crate::core::bytes::bytes::Bytes;
use crate::core::bytes::instancehandle_t::InstanceHandleT;

/// 统一的Writer结构体，同时支持高级API和底层API
pub struct Writer {
    pub raw: *mut DDS_DataWriter,
}

/// 简化构造函数，用于高级API
impl Writer {
    pub fn new(raw: *mut DDS_DataWriter) -> Self {
        Writer { raw }
    }
}

impl Writer {
    /** 发布一个数据样本。
     */
    pub fn write(&self, sample: &Bytes, handle: &InstanceHandleT) -> i32 {
        let _writer: *mut DDS_BytesDataWriter = self.raw.cast();

        unsafe { DDS_BytesDataWriter_write(_writer, sample.as_ref(), handle.raw) }
    }

    pub fn writer_register_instance(&mut self, bytes: &mut Bytes) -> InstanceHandleT {
        InstanceHandleT {
            raw: &mut unsafe {
                DDS_BytesDataWriter_register_instance(
                    self.raw as *mut DDS_BytesDataWriter,
                    bytes.as_mut(),
                )
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;

    #[test]
    fn test_writer_creation() {
        let writer = Writer {
            raw: ptr::null_mut(),
        };
        assert!(writer.raw.is_null());
    }

    #[test]
    fn test_writer_new() {
        let writer = Writer::new(ptr::null_mut());
        assert!(writer.raw.is_null());
        
        // 测试非空指针
        let mut mock_writer = std::mem::MaybeUninit::<DDS_DataWriter>::uninit();
        let writer = Writer::new(mock_writer.as_mut_ptr());
        assert!(!writer.raw.is_null());
    }

    #[test]
    fn test_writer_with_valid_pointer() {
        let mut mock_writer = std::mem::MaybeUninit::<DDS_DataWriter>::uninit();
        let writer = Writer {
            raw: mock_writer.as_mut_ptr(),
        };
        assert!(!writer.raw.is_null());
    }

    #[test]
    fn test_writer_null_safety() {
        let writer = Writer {
            raw: ptr::null_mut(),
        };
        
        // 验证空指针不会导致程序崩溃
        assert!(writer.raw.is_null());
        
        // 测试多个实例
        let mut writers = Vec::new();
        for _ in 0..5 {
            writers.push(Writer {
                raw: ptr::null_mut(),
            });
        }
        
        for writer in &writers {
            assert!(writer.raw.is_null());
        }
    }

    #[test]
    fn test_writer_memory_layout() {
        let writer1 = Writer {
            raw: ptr::null_mut(),
        };
        let writer2 = Writer {
            raw: ptr::null_mut(),
        };
        
        // 两个不同的实例应该有不同的内存地址
        assert_ne!(&writer1 as *const _, &writer2 as *const _);
        
        // 但它们的 raw 字段值应该相同（都是 null）
        assert_eq!(writer1.raw, writer2.raw);
    }

    #[test]
    fn test_writer_with_different_pointers() {
        let mut mock_writer1 = std::mem::MaybeUninit::<DDS_DataWriter>::uninit();
        let mut mock_writer2 = std::mem::MaybeUninit::<DDS_DataWriter>::uninit();
        
        let writer1 = Writer {
            raw: mock_writer1.as_mut_ptr(),
        };
        let writer2 = Writer {
            raw: mock_writer2.as_mut_ptr(),
        };
        
        // 两个 Writer 实例应该有不同的原始指针
        assert_ne!(writer1.raw, writer2.raw);
        assert!(!writer1.raw.is_null());
        assert!(!writer2.raw.is_null());
    }

    #[test]
    fn test_multiple_writers() {
        let mut writers = Vec::new();
        
        // 创建多个 Writer 实例
        for i in 0..10 {
            if i % 2 == 0 {
                writers.push(Writer::new(ptr::null_mut()));
            } else {
                let mut mock_writer = std::mem::MaybeUninit::<DDS_DataWriter>::uninit();
                writers.push(Writer::new(mock_writer.as_mut_ptr()));
            }
        }
        
        // 验证偶数索引的 Writer 有空指针
        for (i, writer) in writers.iter().enumerate() {
            if i % 2 == 0 {
                assert!(writer.raw.is_null());
            } else {
                assert!(!writer.raw.is_null());
            }
        }
    }

    #[test]
    fn test_writer_struct_size() {
        // 验证 Writer 结构体的大小符合预期
        assert_eq!(std::mem::size_of::<Writer>(), std::mem::size_of::<*mut DDS_DataWriter>());
    }
}
