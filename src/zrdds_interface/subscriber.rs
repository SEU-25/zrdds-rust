use crate::bindings::*;

pub struct Subscriber{
    pub(crate) raw: *mut DDS_Subscriber,
}