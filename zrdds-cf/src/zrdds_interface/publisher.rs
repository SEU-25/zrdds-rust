use crate::bindings::*;
use crate::zrdds_interface::dp_domain_participant::DPDomainParticipant;

pub struct Publisher<'a>{
    pub(crate) raw: *mut DDS_Publisher,
    pub(crate) _marker: std::marker::PhantomData<&'a DPDomainParticipant>,
}