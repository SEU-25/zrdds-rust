use std::marker::PhantomData;
use crate::bindings::*;
use crate::zrdds_interface::subscriber::Subscriber;

pub struct Reader<'a, 'b>{
    pub(crate) raw: *mut DDS_DataReader,
    pub(crate) _marker: PhantomData<&'b Subscriber<'a>>,
}

impl Reader<'_, '_>{
    /** 设置该数据读者的监听器。

    本方法将覆盖原有监听器，如果设置空对象表示清除原先设置的监听器。
    */
    pub fn set_listener(self_: Reader, listener: *mut DDS_DataReaderListener, mask: u32) -> i32 {
        unsafe { DDS_BytesDataReader_set_listener(self_.raw.cast(), listener, mask) }
    }
}