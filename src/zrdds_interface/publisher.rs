use crate::bindings::*;

pub struct Publisher{
    pub(crate) raw: *mut DDS_Publisher,
}