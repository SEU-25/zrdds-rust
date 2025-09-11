use crate::bindings::DDS_DataWriterQos;

pub struct WriterQos{
    pub(crate) raw: *mut DDS_DataWriterQos,
}