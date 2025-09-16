use crate::bindings::*;
use crate::core::ReaderListener;
use crate::core::subscription::Subscriber;
use std::marker::PhantomData;

/// 统一的Reader结构体，同时支持高级API和底层API
pub struct Reader {
    pub raw: *mut DDS_DataReader,
    // pub _marker: PhantomData<&Subscriber>,
}

/// 简化构造函数，用于高级API
impl Reader {
    pub fn new(raw: *mut DDS_DataReader) -> Self {
        Reader {
            raw,
            // _marker: PhantomData,
        }
    }
}

impl Reader {
    /** 设置该数据读者的监听器。

    本方法将覆盖原有监听器，如果设置空对象表示清除原先设置的监听器。
    */
    pub fn set_listener(self_: Reader, listener: &mut ReaderListener, mask: u32) -> i32 {
        unsafe { DDS_BytesDataReader_set_listener(self_.raw.cast(), listener.as_mut_ptr(), mask) }
    }
}

/// 生成 `NAME_on_process_sample` 与 `NAME_on_data_available` 两个导出符号。
#[macro_export]
macro_rules! dds_simple_data_reader_listener {
    // 让调用者提供处理体形参名：|$rdr, $samp, $info| $body
    ($name:ident, $Type:ident, |$rdr:ident, $samp:ident, $in:ident| $body:block) => {
        ::paste::paste! {
            #[allow(unused_variables)]
            #[unsafe(no_mangle)]
            pub unsafe extern "C" fn [<$name _on_process_sample>](
                the_reader: *mut DDS_DataReader,
                sample: *mut $Type,
                info: *mut DDS_SampleInfo,
            ) {
                // 将真实参数绑定到调用者自定义的名字上
                let $rdr = the_reader;
                let $samp = sample;
                let $in   = info;
                $body
            }

            #[unsafe(no_mangle)]
            pub extern "C" fn [<$name _on_data_available>](the_reader: *mut DDS_DataReader) {
                unsafe {
                    let mut data_values = ::core::mem::MaybeUninit::<[<$Type Seq>]>::uninit();
                    let mut sample_infos = ::core::mem::MaybeUninit::<DDS_SampleInfoSeq>::uninit();

                    [<$Type Seq_initialize>](data_values.as_mut_ptr());
                    DDS_SampleInfoSeq_initialize(sample_infos.as_mut_ptr());

                    let data_values: &mut [<$Type Seq>] = data_values.assume_init_mut();
                    let sample_infos: &mut DDS_SampleInfoSeq = sample_infos.assume_init_mut();

                    let reader = the_reader as *mut [<$Type DataReader>];

                    let ret = [<$Type DataReader_take>](
                        reader,
                        data_values,
                        sample_infos,
                        LENGTH_UNLIMITED,
                        DDS_ANY_SAMPLE_STATE,
                        DDS_ANY_VIEW_STATE,
                        DDS_ANY_INSTANCE_STATE,
                    );
                    if ret != DDS_ReturnCode_t_DDS_RETCODE_OK {
                        return;
                    }

                    let mut i: u32 = 0;
                    let len: u32 = sample_infos._length as u32;
                    while i < len {
                        let info = DDS_SampleInfoSeq_get_reference(sample_infos, i);
                        if (*info).valid_data == 0 {
                            i += 1;
                            continue;
                        }
                        let sample = [<$Type Seq_get_reference>](data_values, i);

                        [<$name _on_process_sample>](the_reader, sample, info);
                        i += 1;
                    }

                    let _ = [<$Type DataReader_return_loan>](reader, data_values, sample_infos);
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;

    #[test]
    fn test_reader_creation() {
        let reader = Reader {
            raw: ptr::null_mut(),
        };
        assert!(reader.raw.is_null());
    }

    #[test]
    fn test_reader_new() {
        let reader = Reader::new(ptr::null_mut());
        assert!(reader.raw.is_null());
        
        // 测试非空指针
        let mut mock_reader = std::mem::MaybeUninit::<DDS_DataReader>::uninit();
        let reader = Reader::new(mock_reader.as_mut_ptr());
        assert!(!reader.raw.is_null());
    }

    #[test]
    fn test_reader_with_valid_pointer() {
        let mut mock_reader = std::mem::MaybeUninit::<DDS_DataReader>::uninit();
        let reader = Reader {
            raw: mock_reader.as_mut_ptr(),
        };
        assert!(!reader.raw.is_null());
    }

    #[test]
    fn test_reader_null_safety() {
        let reader = Reader {
            raw: ptr::null_mut(),
        };
        
        // 验证空指针不会导致程序崩溃
        assert!(reader.raw.is_null());
        
        // 测试多个实例
        let mut readers = Vec::new();
        for _ in 0..5 {
            readers.push(Reader {
                raw: ptr::null_mut(),
            });
        }
        
        for reader in &readers {
            assert!(reader.raw.is_null());
        }
    }

    #[test]
    fn test_reader_memory_layout() {
        let reader1 = Reader {
            raw: ptr::null_mut(),
        };
        let reader2 = Reader {
            raw: ptr::null_mut(),
        };
        
        // 两个不同的实例应该有不同的内存地址
        assert_ne!(&reader1 as *const _, &reader2 as *const _);
        
        // 但它们的 raw 字段值应该相同（都是 null）
        assert_eq!(reader1.raw, reader2.raw);
    }

    #[test]
    fn test_reader_with_different_pointers() {
        let mut mock_reader1 = std::mem::MaybeUninit::<DDS_DataReader>::uninit();
        let mut mock_reader2 = std::mem::MaybeUninit::<DDS_DataReader>::uninit();
        
        let reader1 = Reader {
            raw: mock_reader1.as_mut_ptr(),
        };
        let reader2 = Reader {
            raw: mock_reader2.as_mut_ptr(),
        };
        
        // 两个 Reader 实例应该有不同的原始指针
        assert_ne!(reader1.raw, reader2.raw);
        assert!(!reader1.raw.is_null());
        assert!(!reader2.raw.is_null());
    }

    #[test]
    fn test_multiple_readers() {
        let mut readers = Vec::new();
        
        // 创建多个 Reader 实例
        for i in 0..10 {
            if i % 2 == 0 {
                readers.push(Reader::new(ptr::null_mut()));
            } else {
                let mut mock_reader = std::mem::MaybeUninit::<DDS_DataReader>::uninit();
                readers.push(Reader::new(mock_reader.as_mut_ptr()));
            }
        }
        
        // 验证偶数索引的 Reader 有空指针
        for (i, reader) in readers.iter().enumerate() {
            if i % 2 == 0 {
                assert!(reader.raw.is_null());
            } else {
                assert!(!reader.raw.is_null());
            }
        }
    }

    #[test]
    fn test_reader_struct_size() {
        // 验证 Reader 结构体的大小符合预期
        assert_eq!(std::mem::size_of::<Reader>(), std::mem::size_of::<*mut DDS_DataReader>());
    }

    #[test]
    fn test_reader_creation_patterns() {
        // 测试不同的创建模式
        let readers = vec![
            Reader { raw: ptr::null_mut() },
            Reader::new(ptr::null_mut()),
        ];
        
        for reader in &readers {
            assert!(reader.raw.is_null());
        }
    }
}
