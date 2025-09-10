use crate::bindings::DDS_DomainParticipantQos;

struct DPQos{
    pub(crate) raw: *const DDS_DomainParticipantQos,
}
