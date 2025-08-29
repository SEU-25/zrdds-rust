use crate::bindings::*;
use crate::zrdds_interface::dp_domain_participant::DPDomainParticipant;

pub struct Subscriber<'a>{
    pub(crate) raw: *mut DDS_Subscriber,
    pub(crate) _marker: std::marker::PhantomData<&'a DPDomainParticipant>,
}