use std::marker::PhantomData;
use crate::bindings::*;
use crate::zrdds_interface::subscriber::Subscriber;

pub struct Reader<'a, 'b>{
    pub(crate) raw: *mut DDS_DataReader,
    pub(crate) _marker: PhantomData<&'b Subscriber<'a>>,
}