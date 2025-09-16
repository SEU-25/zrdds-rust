use crate::bindings::DDS_TopicListener;

pub struct TopicListener {
    pub raw: *mut DDS_TopicListener,
}
