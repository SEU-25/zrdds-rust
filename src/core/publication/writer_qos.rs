use crate::bindings::DDS_DataWriterQos;

pub struct WriterQos{
    pub raw: *mut DDS_DataWriterQos,
}