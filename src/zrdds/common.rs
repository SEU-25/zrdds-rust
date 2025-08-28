use super::domain_participant_factory::DomainParticipantFactory;
use crate::bindings::*;
use std::ffi::CString;

pub fn init(xml: &str, profile: &str) -> DomainParticipantFactory {
    let xml = CString::new(xml).unwrap();
    let profile = CString::new(profile).unwrap();
    let factory = unsafe { DDS_Init(xml.as_ptr(), profile.as_ptr()) };
    if factory.is_null() {
        panic!("DDS 初始化失败，请检查 XML 配置文件和 profile 名称");
    }

    DomainParticipantFactory { raw: factory }
}

pub fn bytes_write(domain_id: u32, topic_name: &str, data: &[u8]) -> u32 {
    unsafe {
        let topic_name = CString::new(topic_name).unwrap();
        DDS_BytesWrite(
            domain_id,
            topic_name.as_ptr() as *mut DDS_Char,
            data.as_ptr() as *const DDS_Char,
            data.len() as DDS_Long,
        )
    }
}
