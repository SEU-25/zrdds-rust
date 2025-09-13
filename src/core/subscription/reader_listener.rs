use crate::bindings::{DDS_DataReader, DDS_DataReaderListener};
use std::{mem, ptr};
use std::pin::Pin;

pub struct ReaderListener {
    inner: Option<Pin<Box<DDS_DataReaderListener>>>,
}

impl ReaderListener {
    /// 有监听器：分配并置零
    pub fn new() -> Self {
        let inner = Box::pin(unsafe { mem::zeroed::<DDS_DataReaderListener>() });
        Self { inner: Some(inner) }
    }

    /// 无监听器：表示“不要监听器”
    pub fn none() -> Self {
        Self { inner: None }
    }

    /// 需要把它交给 C 时用：Some -> 有效指针；None -> NULL
    pub fn as_ptr(&self) -> *const DDS_DataReaderListener {
        self.inner
            .as_ref()
            .map(|p| p.as_ref().get_ref() as *const _)
            .unwrap_or(ptr::null())
    }

    pub fn as_mut_ptr(&mut self) -> *mut DDS_DataReaderListener {
        self.inner
            .as_mut()
            .map(|p| unsafe { p.as_mut().get_unchecked_mut() } as *mut _)
            .unwrap_or(ptr::null_mut())
    }

    /// 若当前是 None，先分配；然后设置回调
    pub fn set_on_data_available(
        &mut self,
        cb: extern "C" fn(reader: *mut DDS_DataReader),
    ) {
        if self.inner.is_none() {
            self.inner = Some(Box::pin(unsafe { mem::zeroed() }));
        }
        
        unsafe {
            // 假设字段名如下，按你的真实定义替换
            self.inner
                .as_mut()
                .unwrap()
                .as_mut()
                .get_unchecked_mut()
                .on_data_available = Some(cb);
        }
    }
}
