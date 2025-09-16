use crate::bindings::{DDS_TOPIC_QOS_DEFAULT, DDS_TopicQos};

pub struct TopicQos {
    pub raw: *mut DDS_TopicQos,
}

impl TopicQos {
    pub fn default_qos() -> TopicQos {
        TopicQos {
            raw: unsafe { &raw mut DDS_TOPIC_QOS_DEFAULT },
        }
    }
}
