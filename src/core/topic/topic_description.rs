use crate::bindings::DDS_TopicDescription;

pub struct TopicDescription{
    pub raw: *mut DDS_TopicDescription,
}