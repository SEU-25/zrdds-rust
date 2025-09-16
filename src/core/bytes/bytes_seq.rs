use crate::bindings::DDS_BytesSeq;

pub struct BytesSeq {
    pub raw: *mut DDS_BytesSeq,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;

    #[test]
    fn test_bytes_seq_creation() {
        let bytes_seq = BytesSeq {
            raw: ptr::null_mut(),
        };
        assert!(bytes_seq.raw.is_null());
    }

    #[test]
    fn test_bytes_seq_with_valid_pointer() {
        // 创建一个模拟的有效指针（在实际使用中应该来自 DDS）
        let mut mock_seq = std::mem::MaybeUninit::<crate::bindings::DDS_BytesSeq>::uninit();
        let bytes_seq = BytesSeq {
            raw: mock_seq.as_mut_ptr(),
        };
        assert!(!bytes_seq.raw.is_null());
    }

    #[test]
    fn test_bytes_seq_null_safety() {
        let bytes_seq = BytesSeq {
            raw: ptr::null_mut(),
        };
        // 验证空指针不会导致程序崩溃
        assert!(bytes_seq.raw.is_null());
        
        // 测试多个实例
        let mut seq_vec = Vec::new();
        for _ in 0..5 {
            seq_vec.push(BytesSeq {
                raw: ptr::null_mut(),
            });
        }
        
        for seq in &seq_vec {
            assert!(seq.raw.is_null());
        }
    }
}
