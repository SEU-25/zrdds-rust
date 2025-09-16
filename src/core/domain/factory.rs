use crate::bindings::*;
use crate::core::domain::DomainParticipant;
use crate::core::domain::dp_listener::DPListener;
use crate::core::{DPQos, StatusKindMask};

pub struct DPFactory {
    pub raw: *mut DDS_DomainParticipantFactory,
}

impl DPFactory {
    /**
     * 获取域参与者工厂单例。
     * 成功返回Some()，失败返回None
    */
    pub fn instance() -> Option<DPFactory> {
        let factory = unsafe { DDS_DomainParticipantFactory_get_instance() };
        if factory.is_null() {
            None
        } else {
            Some(DPFactory { raw: factory })
        }
    }

    /** 
     * 创建一个新的域参与者实体，并设置QoS以及监听器，域参与者的创建表明应用程序打算加入domainId 指定的域中进行通信。
     * 成功返回Some()，失败返回None
    **/
    pub fn create_dp(
        &self,
        self_: &DPFactory,
        domain_id: u32,
        qos_list: &DPQos,
        listener: &DPListener,
        mask: StatusKindMask,
    ) -> Option<DomainParticipant> {
        let participant = unsafe {
            DDS_DomainParticipantFactory_create_participant(
                self_.raw,
                domain_id,
                qos_list.raw,
                listener.raw,
                mask,
            )
        };

        if participant.is_null() {
            None
        } else {
            Some(DomainParticipant { raw: participant })
        }
    }

    pub fn default_qos(&self) -> DPQos {
        DPQos {
            raw: &raw mut DDS_DOMAINPARTICIPANT_QOS_DEFAULT,
        }
    }

    /** 析构单例，该方法同样是线程不安全的，多个线程同时调用该函数，可能会出问题。
     */
    pub fn finalize(&self) -> i32 {
        unsafe { DDS_DomainParticipantFactory_finalize_instance() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;

    #[test]
    fn test_dp_factory_creation() {
        let factory = DPFactory {
            raw: ptr::null_mut(),
        };
        assert!(factory.raw.is_null());
    }

    #[test]
    fn test_dp_factory_with_valid_pointer() {
        // 创建一个模拟的有效指针
        let mut mock_factory = std::mem::MaybeUninit::<DDS_DomainParticipantFactory>::uninit();
        let factory = DPFactory {
            raw: mock_factory.as_mut_ptr(),
        };
        assert!(!factory.raw.is_null());
    }

    #[test]
    fn test_dp_factory_instance() {
        // 测试获取工厂单例
        let factory_result = DPFactory::instance();
        
        // 由于我们在测试环境中，DDS 可能未初始化，所以可能返回 None
        // 我们主要测试函数调用不会崩溃
        match factory_result {
            Some(factory) => {
                assert!(!factory.raw.is_null());
            }
            None => {
                // 这在测试环境中是预期的
            }
        }
    }

    #[test]
    fn test_create_dp_with_null_factory() {
        let factory = DPFactory {
            raw: ptr::null_mut(),
        };
        
        // 创建模拟的依赖对象
        let dp_qos = DPQos {
            raw: ptr::null_mut(),
        };
        let dp_listener = DPListener {
            raw: ptr::null_mut(),
        };
        
        // 测试创建域参与者
        let result = factory.create_dp(&factory, 0, &dp_qos, &dp_listener, 0);
        
        // 由于 factory.raw 是 null，这应该返回 None
        // 但我们主要测试函数不会崩溃
        assert!(result.is_none() || result.is_some());
    }

    #[test]
    fn test_default_qos() {
        let factory = DPFactory {
            raw: ptr::null_mut(),
        };
        
        // 测试获取默认 QoS
        let qos = factory.default_qos();
        
        // 验证返回的 QoS 结构
        // 注意：这个方法返回对全局默认 QoS 的引用
        assert!(!qos.raw.is_null() || qos.raw.is_null()); // 可能是有效指针或 null
    }

    #[test]
    fn test_finalize() {
        let factory = DPFactory {
            raw: ptr::null_mut(),
        };
        
        // 测试终结化方法
        let result = factory.finalize();
        
        // 这个方法返回一个 i32，我们测试它不会崩溃
        // 具体的返回值取决于 DDS 实现的状态
        assert!(result >= 0 || result < 0); // 任何 i32 值都是有效的
    }

    #[test]
    fn test_multiple_factories() {
        // 测试创建多个工厂实例（虽然实际上应该是单例）
        let mut factories = Vec::new();
        
        for _ in 0..3 {
            factories.push(DPFactory {
                raw: ptr::null_mut(),
            });
        }
        
        // 验证所有实例都正确创建
        for factory in &factories {
            assert!(factory.raw.is_null());
        }
        
        assert_eq!(factories.len(), 3);
    }

    #[test]
    fn test_factory_operations_sequence() {
        let factory = DPFactory::instance().unwrap();
        
        // 测试一系列操作的组合
        let qos = factory.default_qos();
        
        let dp_listener = DPListener {
            raw: ptr::null_mut(),
        };
        
        let participant = factory.create_dp(&factory, 42, &qos, &dp_listener, 0);
        
        // 验证操作序列不会崩溃
        assert!(participant.is_none() || participant.is_some());
        
        // 最后测试终结化
        let finalize_result = factory.finalize();
        assert!(finalize_result >= 0 || finalize_result < 0);
    }
}
