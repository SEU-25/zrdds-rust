use crate::bindings::*;

pub struct DomainParticipantQos {
    pub(crate) raw: *mut DDS_DomainParticipantQos,
}

pub struct PublisherQos {
    pub(crate) raw: *mut DDS_PublisherQos,
}

pub struct SubscriberQos {
    pub(crate) raw: *mut DDS_SubscriberQos,
}
