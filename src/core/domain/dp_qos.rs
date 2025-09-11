use crate::bindings::DDS_DomainParticipantQos;

pub struct DPQos{
    pub(crate) raw: *mut DDS_DomainParticipantQos,
}
