use crate::bindings::DDS_DomainParticipantQos;

pub struct DPQos {
    pub raw: *mut DDS_DomainParticipantQos,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;

    #[test]
    fn test_dp_qos_creation() {
        let qos = DPQos {
            raw: ptr::null_mut(),
        };
        assert!(qos.raw.is_null());
    }

    #[test]
    fn test_dp_qos_memory_layout() {
        // 验证结构体的内存布局符合预期
        let qos1 = DPQos {
            raw: ptr::null_mut(),
        };
        let qos2 = DPQos {
            raw: ptr::null_mut(),
        };
        
        // 两个不同的实例应该有不同的内存地址
        assert_ne!(&qos1 as *const _, &qos2 as *const _);
        
        // 但它们的 raw 字段值应该相同（都是 null）
        assert_eq!(qos1.raw, qos2.raw);
    }

    #[test]
    fn test_dp_qos_with_different_pointers() {
        // 测试不同的指针值
        let mut mock_qos1 = std::mem::MaybeUninit::<DDS_DomainParticipantQos>::uninit();
        let mut mock_qos2 = std::mem::MaybeUninit::<DDS_DomainParticipantQos>::uninit();
        
        let qos1 = DPQos {
            raw: mock_qos1.as_mut_ptr(),
        };
        let qos2 = DPQos {
            raw: mock_qos2.as_mut_ptr(),
        };
        
        // 两个 QoS 实例应该有不同的原始指针
        assert_ne!(qos1.raw, qos2.raw);
        assert!(!qos1.raw.is_null());
        assert!(!qos2.raw.is_null());
    }
}
