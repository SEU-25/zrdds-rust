use crate::bindings::*;
use crate::zrdds_interface::publisher::Publisher;
use std::marker::PhantomData;

pub struct Writer<'a, 'b> {
    pub(crate) raw: *mut DDS_DataWriter,
    pub(crate) _marker: PhantomData<&'b Publisher<'a>>,
}

impl Writer<'_, '_> {
    /** 发布一个数据样本。
     */
    pub fn write(
        &self,
        self_: Writer,
        sample: *const DDS_Bytes,
        handle: *const DDS_InstanceHandle_t,
    ) -> i32 {
        let _writer: *mut DDS_BytesDataWriter = self_.raw.cast();

        unsafe { DDS_BytesDataWriter_write(_writer, sample, handle) }
    }
}
