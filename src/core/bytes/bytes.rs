use crate::bindings::{
    DDS_Bytes, DDS_Octet, DDS_OctetSeq_initialize, DDS_OctetSeq_loan_contiguous,
};
use crate::core::bytes::Boolean;
use std::pin::Pin;
use std::{mem, ptr};

pub struct Bytes {
    pub inner: Option<Pin<Box<DDS_Bytes>>>,
}

impl Bytes {
    pub fn new() -> Self {
        let inner = Box::pin(unsafe { mem::zeroed::<DDS_Bytes>() });
        Self { inner: Some(inner) }
    }

    pub fn none() -> Self {
        Bytes { inner: None }
    }

    pub fn as_ref(&self) -> *const DDS_Bytes {
        self.inner
            .as_ref()
            .map(|p| p.as_ref().get_ref() as *const _)
            .unwrap_or(ptr::null())
    }

    pub fn as_mut(&mut self) -> *mut DDS_Bytes {
        self.inner
            .as_mut()
            .map(|p| unsafe { p.as_mut().get_unchecked_mut() } as *mut _)
            .unwrap_or(ptr::null_mut())
    }

    pub fn octet_seq_initialize(&mut self) {
        unsafe {
            DDS_OctetSeq_initialize(&mut (*self.as_mut()).value);
        }
    }

    pub fn octet_seq_loan_contiguous(
        &mut self,
        buffer: &[u8],
        new_length: u32,
        new_max: u32,
    ) -> Boolean {
        unsafe {
            DDS_OctetSeq_loan_contiguous(
                &mut (*self.as_mut()).value,
                buffer.as_ptr() as *mut DDS_Octet,
                new_length,
                new_max,
            )
        }
    }
}
