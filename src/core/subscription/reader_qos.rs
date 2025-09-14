use crate::bindings::DDS_DataReaderQos;
use std::pin::Pin;
use std::{mem, ptr};

pub struct ReaderQos {
    pub inner: Option<Pin<Box<DDS_DataReaderQos>>>,
}

impl ReaderQos {
    pub fn new() -> Self {
        let inner = Box::pin(unsafe { mem::zeroed::<DDS_DataReaderQos>() });
        Self { inner: Some(inner) }
    }

    pub fn none() -> Self {
        Self { inner: None }
    }

    pub fn as_ptr(&self) -> *const DDS_DataReaderQos {
        self.inner
            .as_ref()
            .map(|p| p.as_ref().get_ref() as *const _)
            .unwrap_or(ptr::null())
    }

    pub fn as_ptr_mut(&mut self) -> *mut DDS_DataReaderQos {
        self.inner
            .as_mut()
            .map(|p| unsafe { p.as_mut().get_unchecked_mut() } as *mut _)
            .unwrap_or(ptr::null_mut())
    }
    pub fn get_for_now(self, f: Box<fn(p: Self) -> Pin<Box<DDS_DataReaderQos>>>) -> Self {
        Self {
            inner: Some(f(self)),
        }
    }
}
