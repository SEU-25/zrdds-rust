use crate::bindings::{DDS_PUBLISHER_QOS_DEFAULT, DDS_PublisherQos};

pub struct PublisherQos {
    pub raw: *mut DDS_PublisherQos,
}

impl PublisherQos {
    pub fn default_qos() -> Self {
        PublisherQos {
            raw: unsafe { &raw mut DDS_PUBLISHER_QOS_DEFAULT },
        }
    }
}
