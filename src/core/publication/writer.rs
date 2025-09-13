use crate::bindings::*;
use crate::core::publication::Publisher;
use std::marker::PhantomData;

/// 统一的Writer结构体，同时支持高级API和底层API
pub struct Writer {
    pub raw: *mut DDS_DataWriter,
}

/// 简化构造函数，用于高级API
impl Writer {
    pub fn new(raw: *mut DDS_DataWriter) -> Self {
        Writer {
            raw,
        }
    }
}

impl Writer {
    /** 发布一个数据样本。
     */
    pub fn write(&self, sample: *const DDS_Bytes, handle: *const DDS_InstanceHandle_t) -> i32 {
        let _writer: *mut DDS_BytesDataWriter = self.raw.cast();

        unsafe { DDS_BytesDataWriter_write(_writer, sample, handle) }
    }
}
