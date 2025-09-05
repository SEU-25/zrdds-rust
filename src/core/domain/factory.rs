use std::mem::MaybeUninit;

use crate::core::domain::DomainParticipant;
use crate::{DomainParticipantListener, DomainParticipantQos, bindings::*};

pub struct DPFactory {
    pub(crate) raw: *mut DDS_DomainParticipantFactory,
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
        domain_id: u32,
        qos_list: &DomainParticipantQos,
        listener: &DomainParticipantListener,
        mask: DDS_StatusKindMask,
    ) -> Option<DomainParticipant> {
        let mut listener_copy = *listener;
        let participant = unsafe {
            DDS_DomainParticipantFactory_create_participant(
                self.raw,
                domain_id,
                qos_list.raw,
                &mut listener_copy as *mut _,
                mask,
            )
        };

        if participant.is_null() {
            None
        } else {
            Some(DomainParticipant { raw: participant })
        }
    }

    pub fn default_qos(&self) -> Result<DomainParticipantQos, i32> {
        let mut qos = DomainParticipantQos {
            raw: MaybeUninit::uninit().as_mut_ptr(),
        };

        let ret =
            unsafe { DDS_DomainParticipantFactory_get_default_participant_qos(self.raw, qos.raw) };

        if ret == 0 { Ok(qos) } else { Err(ret) }
    }

    /** 析构单例，该方法同样是线程不安全的，多个线程同时调用该函数，可能会出问题。
     */
    pub fn finalize(&self) -> i32 {
        unsafe { DDS_DomainParticipantFactory_finalize_instance() }
    }
}
