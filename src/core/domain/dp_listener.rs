use crate::bindings::DDS_DomainParticipantListener;

pub struct DPListener{
    pub raw: *mut DDS_DomainParticipantListener,
}