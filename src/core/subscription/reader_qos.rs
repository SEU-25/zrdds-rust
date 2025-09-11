use crate::bindings::DDS_DataReaderQos;

pub struct ReaderQos{
    pub(crate) raw: *mut DDS_DataReaderQos,
}