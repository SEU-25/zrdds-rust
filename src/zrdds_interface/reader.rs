use crate::bindings::*;
use crate::zrdds_interface::subscriber::Subscriber;
use std::marker::PhantomData;

pub struct Reader<'a, 'b> {
    pub(crate) raw: *mut DDS_DataReader,
    pub(crate) _marker: PhantomData<&'b Subscriber<'a>>,
}

impl Reader<'_, '_> {
    /** 设置该数据读者的监听器。

    本方法将覆盖原有监听器，如果设置空对象表示清除原先设置的监听器。
    */
    pub fn set_listener(self_: Reader, listener: *mut DDS_DataReaderListener, mask: u32) -> i32 {
        unsafe { DDS_BytesDataReader_set_listener(self_.raw.cast(), listener, mask) }
    }
}

#[macro_export]
macro_rules! dds_simple_data_reader_listener {
    // A) 生成 on_data_available + on_process_sample（在宏中内联提供处理逻辑闭包）
    // 用法：
    // dds_simple_data_reader_listener!(Track, DDS_ZeroCopyBytes, take = DDS_ZeroCopyBytes_take_next_sample, |r,s,i| { /* ... */ });
    ($name:ident, $Type:ty, take = $take_fn:path, $handler:expr) => {
        paste! {
            #[no_mangle]
            pub unsafe extern "C" fn [<$name _on_data_available>](reader: *mut $crate::DDS_DataReader) {
                let mut sample: core::mem::MaybeUninit<$Type> = core::mem::MaybeUninit::uninit();
                let mut info: $crate::DDS_SampleInfo = core::mem::zeroed();

                loop {
                    let rc: $crate::DDS_ReturnCode_t =
                        $take_fn(reader, sample.as_mut_ptr(), &mut info as *mut _);

                    if rc == $crate::DDS_RETCODE_NO_DATA { break; }
                    if rc != $crate::DDS_RETCODE_OK { break; } // 可按需扩展错误处理

                    // 调用同名的样本处理函数（我们也会在下方生成它）
                    [<$name _on_process_sample>](reader, sample.as_mut_ptr(), &mut info as *mut _);
                }
            }

            // 直接把用户提供的闭包/函数体变成一个导出 C ABI 的函数
            #[no_mangle]
            pub unsafe extern "C" fn [<$name _on_process_sample>](
                reader: *mut $crate::DDS_DataReader,
                sample: *mut $Type,
                info:   *mut $crate::DDS_SampleInfo,
            ) {
                // $handler 要形如 |r, s, i| { ... }
                ($handler)(reader, sample, info)
            }
        }
    };
}
