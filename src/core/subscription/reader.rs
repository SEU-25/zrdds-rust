use crate::bindings::*;
use crate::core::subscription::Subscriber;
use std::marker::PhantomData;

/// 统一的Reader结构体，同时支持高级API和底层API
pub struct Reader<'a, 'b> {
    pub(crate) raw: *mut DDS_DataReader,
    pub(crate) _marker: PhantomData<&'b Subscriber<'a>>,
}

/// 简化构造函数，用于高级API
impl Reader<'static, 'static> {
    pub fn new(raw: *mut DDS_DataReader) -> Self {
        Reader {
            raw,
            _marker: PhantomData,
        }
    }
}

impl Reader<'_, '_> {
    /** 设置该数据读者的监听器。

    本方法将覆盖原有监听器，如果设置空对象表示清除原先设置的监听器。
    */
    pub fn set_listener(self_: Reader, listener: *mut DDS_DataReaderListener, mask: u32) -> i32 {
        unsafe { DDS_BytesDataReader_set_listener(self_.raw.cast(), listener, mask) }
    }
}

/// 生成 `NAME_on_process_sample` 与 `NAME_on_data_available` 两个导出符号。
#[macro_export]
macro_rules! dds_simple_data_reader_listener {
    ($name:ident, $Type:ident, $body:block) => {
        ::paste::paste! {
            // 允许用户在处理体里不使用参数时不报警告
            #[allow(unused_variables)]
            #[unsafe(no_mangle)]
            pub unsafe extern "C" fn [<$name _on_process_sample>](
                the_reader: *mut DDS_DataReader,
                sample: *mut $Type,
                info: *mut DDS_SampleInfo,
            ) {
                $body
            }

            #[unsafe(no_mangle)]
            pub unsafe extern "C" fn [<$name _on_data_available>](the_reader: *mut DDS_DataReader) {
                // Rust 2024：即使本函数是 unsafe，也要显式包一层 unsafe 块
                unsafe {
                    // 1) 未初始化 -> C 的 *_initialize 负责初始化
                    let mut data_values = ::core::mem::MaybeUninit::<[<$Type Seq>]>::uninit();
                    let mut sample_infos = ::core::mem::MaybeUninit::<DDS_SampleInfoSeq>::uninit();

                    [<$Type Seq_initialize>](data_values.as_mut_ptr());
                    DDS_SampleInfoSeq_initialize(sample_infos.as_mut_ptr());

                    // 初始化后再取可变引用
                    let data_values: &mut [<$Type Seq>] = data_values.assume_init_mut();
                    let sample_infos: &mut DDS_SampleInfoSeq = sample_infos.assume_init_mut();

                    // 2) 转 reader
                    let reader = the_reader as *mut [<$Type DataReader>];

                    // 3) 抓取所有样本
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

                    // 4) 遍历
                    let mut i: u32 = 0;
                    let len: u32 = sample_infos._length as u32; // 若字段名不同，请改这里
                    while i < len {
                        let info = DDS_SampleInfoSeq_get_reference(sample_infos, i);
                        if (*info).valid_data == 0 { // 若为 bool，请改为 `if !(*info).valid_data {`
                            i += 1;
                            continue;
                        }
                        let sample = [<$Type Seq_get_reference>](data_values, i);

                        // 调用导出的处理函数本身也属不安全调用
                        [<$name _on_process_sample>](the_reader, sample, info);
                        i += 1;
                    }

                    // 5) 归还 loan
                    let _ = [<$Type DataReader_return_loan>](reader, data_values, sample_infos);
                }
            }
        }
    };
}
