use crate::bindings::DDS_InstanceHandle_t;

pub struct InstanceHandleT{
    pub raw: *mut DDS_InstanceHandle_t,
}