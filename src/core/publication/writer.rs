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
