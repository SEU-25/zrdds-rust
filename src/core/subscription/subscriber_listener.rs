use crate::bindings::DDS_SubscriberListener;

pub struct SubscriberListener {
    pub raw: *mut DDS_SubscriberListener,
}
