use crate::bindings::*;
use crate::zrdds_interface::dp_domain_participant::DPDomainParticipant;

pub struct Topic<'a>{
    pub(crate) raw: *mut DDS_Topic,
    pub(crate) _marker: std::marker::PhantomData<&'a DPDomainParticipant>,
}