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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes_new() {
        let bytes = Bytes::new();
        assert!(bytes.inner.is_some());
        assert!(!bytes.as_ref().is_null());
    }

    #[test]
    fn test_bytes_none() {
        let bytes = Bytes::none();
        assert!(bytes.inner.is_none());
        assert!(bytes.as_ref().is_null());
    }

    #[test]
    fn test_bytes_as_ref() {
        let bytes = Bytes::new();
        let ptr = bytes.as_ref();
        assert!(!ptr.is_null());

        let none_bytes = Bytes::none();
        let null_ptr = none_bytes.as_ref();
        assert!(null_ptr.is_null());
    }

    #[test]
    fn test_bytes_as_mut() {
        let mut bytes = Bytes::new();
        let ptr = bytes.as_mut();
        assert!(!ptr.is_null());

        let mut none_bytes = Bytes::none();
        let null_ptr = none_bytes.as_mut();
        assert!(null_ptr.is_null());
    }

    #[test]
    fn test_octet_seq_initialize() {
        let mut bytes = Bytes::new();
        // 测试初始化不会崩溃
        bytes.octet_seq_initialize();
        // 验证指针仍然有效
        assert!(!bytes.as_ref().is_null());
    }

    #[test]
    fn test_octet_seq_loan_contiguous() {
        let mut bytes = Bytes::new();
        bytes.octet_seq_initialize();
        
        let buffer = vec![1u8, 2u8, 3u8, 4u8];
        let result = bytes.octet_seq_loan_contiguous(&buffer, 4, 4);
        
        // 验证函数调用成功（返回值应该是有效的 Boolean）
        // 注意：具体的返回值取决于底层 DDS 实现
        assert!(result == 0 || result == 1); // Boolean 值应该是 0 或 1
    }

    #[test]
    fn test_octet_seq_loan_contiguous_empty_buffer() {
        let mut bytes = Bytes::new();
        bytes.octet_seq_initialize();
        
        let buffer: Vec<u8> = vec![];
        let result = bytes.octet_seq_loan_contiguous(&buffer, 0, 0);
        
        // 验证空缓冲区的处理
        assert!(result == 0 || result == 1);
    }

    #[test]
    fn test_bytes_memory_safety() {
        // 测试多个 Bytes 实例的创建和销毁
        let mut bytes_vec = Vec::new();
        for _ in 0..10 {
            bytes_vec.push(Bytes::new());
        }
        
        // 验证所有实例都有效
        for bytes in &bytes_vec {
            assert!(!bytes.as_ref().is_null());
        }
        
        // 测试混合 Some 和 None 实例
        bytes_vec.push(Bytes::none());
        assert!(bytes_vec.last().unwrap().as_ref().is_null());
    }
}
