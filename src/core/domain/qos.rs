use crate::bindings::DDS_DomainParticipantQos;

pub struct DPQos {
    pub(crate) raw: *const DDS_DomainParticipantQos,
}
impl DPQos {}
