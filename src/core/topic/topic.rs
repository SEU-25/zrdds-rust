use crate::bindings::*;
use crate::core::domain::DomainParticipant;
use crate::core::topic_description::TopicDescription;

pub struct Topic<'a> {
    pub raw: *mut DDS_Topic,
    pub _marker: std::marker::PhantomData<&'a DomainParticipant>,
}

impl<'a> Topic<'a> {
    pub fn get_description(&self) -> TopicDescription {
        TopicDescription {
            raw: self.raw as *mut _,
        }
    }
}