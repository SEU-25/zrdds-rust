use crate::bindings::*;
use crate::core::domain::DomainParticipant;

pub struct Topic<'a> {
    pub raw: *mut DDS_Topic,
    pub(crate) _marker: std::marker::PhantomData<&'a DomainParticipant>,
}
