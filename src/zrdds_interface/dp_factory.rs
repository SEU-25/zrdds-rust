use crate::bindings::*;
use crate::zrdds_interface::dp_domain_participant::DPDomainParticipant;

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
            Some(DPFactory {
                raw: factory,
            })
        }
    }

    /** 创建一个新的域参与者实体，并设置QoS以及监听器，域参与者的创建表明应用程序打算加入domainId 指定的域中进行通信。

    成功返回Some()，失败返回None
    */
    pub fn create_dp(
        &self,
        self_: &DPFactory,
        domain_id: u32,
        qos_list: *const DDS_DomainParticipantQos,
        listener: *mut DDS_DomainParticipantListener,
        mask: DDS_StatusKindMask,
    ) -> Option<DPDomainParticipant> {
        let participant = unsafe {
            DDS_DomainParticipantFactory_create_participant(
                self_.raw,
                domain_id,
                qos_list,
                listener,
                mask,
            )
        };

        if participant.is_null() {
            None
        } else {
            Some(DPDomainParticipant{ raw: participant})
        }
    }

    /** 析构单例，该方法同样是线程不安全的，多个线程同时调用该函数，可能会出问题。
     */
    pub fn finalize(&self) -> i32 {
        unsafe { DDS_DomainParticipantFactory_finalize_instance() }
    }
}
