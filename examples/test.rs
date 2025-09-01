use std::ptr::{null_mut};
use zrdds::bindings::{DDS_BytesDataReader, DDS_BytesDataReader_read, DDS_STATUS_MASK_NONE};
use zrdds::zrdds_interface::*;

fn main(){
    let dpf = dp_factory::DPFactory::instance().unwrap();

    let dp = dpf.create_dp(&dpf,1,null_mut(),null_mut(),DDS_STATUS_MASK_NONE).unwrap();

    let topic = dp.create_topic(&dp,"123","456",null_mut(),null_mut(),DDS_STATUS_MASK_NONE).unwrap();

    drop(dp); // 编译器报错
    drop(topic);
}
