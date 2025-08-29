use std::marker::PhantomData;
use crate::bindings::*;
use crate::zrdds_interface::publisher::Publisher;

pub struct Writer<'a, 'b> {
    pub(crate) raw: *mut DDS_DataWriter,
    pub(crate) _marker: PhantomData<&'b Publisher<'a>>,
}