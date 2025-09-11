use crate::bindings::DDS_PublisherQos;

pub struct PublisherQos {
    pub(crate) raw: *mut DDS_PublisherQos,
}