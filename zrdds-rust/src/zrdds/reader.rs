use crate::bindings;

pub struct DataReader {
    pub(crate) raw: *mut bindings::DDS_DataReader,
}
