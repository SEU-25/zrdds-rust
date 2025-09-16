use crate::bindings::DDS_DataWriterListener;
use std::pin::Pin;
use std::{mem, ptr};

pub struct WriterListener {
    pub inner: Option<Pin<Box<DDS_DataWriterListener>>>,
}

impl WriterListener {
    pub fn new() -> Self {
        // 为整个 C 结构体分配内存（Box），然后置零字段（函数指针等初始为 None）
        let inner = Box::pin(unsafe { mem::zeroed::<DDS_DataWriterListener>() });
        Self { inner: Some(inner) }
    }
    pub fn none() -> Self {
        Self { inner: None }
    }

    pub fn as_ptr(&self) -> *const DDS_DataWriterListener {
        self.inner
            .as_ref()
            .map(|p| p.as_ref().get_ref() as *const _)
            .unwrap_or(ptr::null())
    }

    pub fn as_ptr_mut(&mut self) -> *mut DDS_DataWriterListener {
        self.inner
            .as_mut()
            .map(|p| unsafe { p.as_mut().get_unchecked_mut() } as *mut _)
            .unwrap_or(ptr::null_mut())
    }
}
