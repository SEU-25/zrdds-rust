use crate::bindings::DDS_SampleInfoSeq;

pub struct SampleInfoSeq{
    pub(crate) raw: *mut DDS_SampleInfoSeq,
}