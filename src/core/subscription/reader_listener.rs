use std::mem;
use crate::bindings::{DDS_DataReader, DDS_DataReaderListener};
use crate::dds_handlers::on_chat_data_available;

pub struct ReaderListener {
    pub(crate) raw: *mut DDS_DataReaderListener,
}

impl ReaderListener {
    pub fn set_on_data_available(&mut self, on_data_available: extern "C" fn(reader: *mut DDS_DataReader)) {
        unsafe { (*self.raw).on_data_available = Some(on_data_available); }
    }
}