use crate::bindings::DDS_TopicDescription;

pub struct TopicDescription{
    pub(crate) raw: *mut DDS_TopicDescription,
}