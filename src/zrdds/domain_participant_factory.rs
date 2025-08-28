use std::ffi::CString;

use crate::{bindings::*, DomainParticipant};

pub struct DomainParticipantFactory {
    pub(crate) raw: *mut DDS_DomainParticipantFactory,
}

impl DomainParticipantFactory {
    pub fn create_domain_participant(
        &self,
        domain_id: u32,
        participant_name: &str,
    ) -> DomainParticipant {
        let participant_name = CString::new(participant_name).unwrap();
        let dp = unsafe { DDS_CreateDP(domain_id, participant_name.as_ptr()) };
        if dp.is_null() {
            panic!("创建域参与者失败");
        }
        DomainParticipant { raw: dp }
    }
}

impl Drop for DomainParticipantFactory {
    fn drop(&mut self) {
        unsafe {
            DDS_Finalize();
        }
    }
}