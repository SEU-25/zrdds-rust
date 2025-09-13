use std::mem::MaybeUninit;

use crate::bindings::*;
use crate::core::domain::DomainParticipant;
use crate::core::domain::dp_listener::DPListener;
use crate::core::{DPQos, StatusKindMask};

pub struct DPFactory {
    pub raw: *mut DDS_DomainParticipantFactory,
}

impl DPFactory {
    /** 创建域参与者工厂单例。

       成功返回Some()，失败返回None
    */
    pub fn instance() -> Option<DPFactory> {
        let factory = unsafe { DDS_DomainParticipantFactory_get_instance() };
        if factory.is_null() {
            None
        } else {
            Some(DPFactory { raw: factory })
        }
    }

    /** 创建一个新的域参与者实体，并设置QoS以及监听器，域参与者的创建表明应用程序打算加入domainId 指定的域中进行通信。

       成功返回Some()，失败返回None
    */
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
                self_.raw, domain_id, qos_list.raw, listener.raw, mask,
            )
        };

        if participant.is_null() {
            None
        } else {
            Some(DomainParticipant { raw: participant })
        }
    }

    pub fn default_qos(&self) -> DPQos {
        unsafe {
            DPQos {
                raw: &raw mut DDS_DOMAINPARTICIPANT_QOS_DEFAULT,
            }
        }
    }

    /** 析构单例，该方法同样是线程不安全的，多个线程同时调用该函数，可能会出问题。
     */
    pub fn finalize(&self) -> i32 {
        unsafe { DDS_DomainParticipantFactory_finalize_instance() }
    }
}
