use crate::bindings::DDS_SubscriberQos;

pub struct SubscriberQos{
    pub(crate) raw: *mut DDS_SubscriberQos,
}