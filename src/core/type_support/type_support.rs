use crate::bindings::{
    DDS_BytesTypeSupport_get_type_name, DDS_BytesTypeSupport_register_type, DDS_TypeSupport,
};
use crate::core::DomainParticipant;
use std::ffi::{CString, c_char};

pub struct TypeSupport {
    pub raw: *mut DDS_TypeSupport,
}

pub fn type_support_get_name() -> String {
    unsafe {
        CString::from_raw(DDS_BytesTypeSupport_get_type_name() as *mut c_char)
            .into_string()
            .unwrap()
    }
}

pub fn type_support_register_type(participant: &DomainParticipant, type_name: &str) -> i32 {
    unsafe {
        DDS_BytesTypeSupport_register_type(participant.raw, type_name.as_ptr() as *const c_char)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;

    #[test]
    fn test_type_support_creation() {
        let type_support = TypeSupport {
            raw: ptr::null_mut(),
        };
        assert!(type_support.raw.is_null());
    }

    #[test]
    fn test_type_support_with_valid_pointer() {
        let dummy_value = 42u32;
        let type_support = TypeSupport {
            raw: &dummy_value as *const u32 as *mut DDS_TypeSupport,
        };
        assert!(!type_support.raw.is_null());
    }

    #[test]
    fn test_type_support_null_safety() {
        let type_support = TypeSupport {
            raw: ptr::null_mut(),
        };
        // 测试空指针的安全性
        assert!(type_support.raw.is_null());
        assert_eq!(type_support.raw as usize, 0);
    }

    #[test]
    fn test_type_support_memory_layout() {
        let type_support = TypeSupport {
            raw: ptr::null_mut(),
        };
        // 验证内存布局
        assert_eq!(std::mem::size_of_val(&type_support.raw), std::mem::size_of::<*mut DDS_TypeSupport>());
    }

    #[test]
    fn test_type_support_with_different_pointers() {
        let dummy1 = 1u32;
        let dummy2 = 2u32;
        
        let type_support1 = TypeSupport {
            raw: &dummy1 as *const u32 as *mut DDS_TypeSupport,
        };
        
        let type_support2 = TypeSupport {
            raw: &dummy2 as *const u32 as *mut DDS_TypeSupport,
        };
        
        assert_ne!(type_support1.raw, type_support2.raw);
    }

    #[test]
    fn test_multiple_type_supports() {
        let type_supports: Vec<TypeSupport> = (0..5).map(|i| {
            let dummy = Box::leak(Box::new(i as u32));
            TypeSupport {
                raw: dummy as *mut u32 as *mut DDS_TypeSupport,
            }
        }).collect();
        
        assert_eq!(type_supports.len(), 5);
        for type_support in &type_supports {
            assert!(!type_support.raw.is_null());
        }
    }

    #[test]
    fn test_type_support_struct_size() {
        let size = std::mem::size_of::<TypeSupport>();
        // TypeSupport 应该只包含一个指针
        assert_eq!(size, std::mem::size_of::<*mut DDS_TypeSupport>());
    }

    #[test]
    fn test_type_support_creation_patterns() {
        // 测试不同的创建模式
        let type_support1 = TypeSupport {
            raw: ptr::null_mut(),
        };
        
        let dummy = 123u32;
        let type_support2 = TypeSupport {
            raw: &dummy as *const u32 as *mut DDS_TypeSupport,
        };
        
        assert!(type_support1.raw.is_null());
        assert!(!type_support2.raw.is_null());
    }

    #[test]
    fn test_type_support_register_type_function_exists() {
        // 测试函数是否存在（不实际调用，因为需要有效的 DDS 运行时）
        let dummy_participant = DomainParticipant {
            raw: ptr::null_mut(),
        };
        
        // 这里只测试函数签名是否正确，不实际调用
        let _result = type_support_register_type(&dummy_participant, "test_type");
        // 由于使用空指针，这可能会导致错误，但我们只是测试编译时的正确性
    }
}    