use crate::bindings::DDS_SampleInfo;

pub struct SampleInfo {
    pub raw: *mut DDS_SampleInfo,
}