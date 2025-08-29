use crate::bindings;

pub struct DataWriter {
    pub(crate) raw: *mut bindings::DDS_DataWriter,
}
