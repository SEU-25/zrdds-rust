use crate::bindings::DDS_DataWriterListener;

pub struct WriterListener {
    pub raw: *mut DDS_DataWriterListener,
}