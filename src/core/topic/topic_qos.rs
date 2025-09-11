use crate::bindings::DDS_TopicQos;

pub struct TopicQos {
    raw: *mut DDS_TopicQos,
}