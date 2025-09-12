use crate::bindings::{DDS_DataReader, DDS_DataReaderListener};
use std::mem;

pub struct ReaderListener {
    pub raw: DDS_DataReaderListener,
}

impl ReaderListener {
    pub fn new() -> Self {
        Self {
            raw: unsafe { mem::zeroed() },
        }
    }

    pub fn set_on_data_available(
        &mut self,
        on_data_available: extern "C" fn(reader: *mut DDS_DataReader),
    ) {
        unsafe { self.raw.on_data_available = Some(on_data_available); }
    }
}
