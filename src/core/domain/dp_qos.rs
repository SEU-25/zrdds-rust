use crate::bindings::DDS_DomainParticipantQos;

pub struct DPQos{
    pub raw: *mut DDS_DomainParticipantQos,
}
