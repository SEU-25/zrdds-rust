use crate::bindings::*;
use crate::core::domain::DomainParticipant;
use crate::core::TopicQos;

pub struct Topic<'a> {
    pub raw: *mut DDS_Topic,
    pub _marker: std::marker::PhantomData<&'a DomainParticipant>,
}
