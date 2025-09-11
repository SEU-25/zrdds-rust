use crate::bindings::DDS_BytesSeq;

pub struct BytesSeq{
    pub(crate) raw: *mut DDS_BytesSeq,
}