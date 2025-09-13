use crate::bindings::DDS_DataReaderQos;

pub struct ReaderQos{
    pub raw: *mut DDS_DataReaderQos,
}