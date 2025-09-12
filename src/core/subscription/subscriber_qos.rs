use crate::bindings::{DDS_SubscriberQos, DDS_SUBSCRIBER_QOS_DEFAULT};

pub struct SubscriberQos{
    pub(crate) raw: *mut DDS_SubscriberQos,
}

impl SubscriberQos{
    pub fn default_qos() -> Self{
        Self {
            raw: unsafe {&raw mut DDS_SUBSCRIBER_QOS_DEFAULT},
        }
    }
}