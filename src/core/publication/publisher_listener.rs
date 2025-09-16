use crate::bindings::DDS_PublisherListener;

pub struct PublisherListener {
    pub raw: *mut DDS_PublisherListener,
}
