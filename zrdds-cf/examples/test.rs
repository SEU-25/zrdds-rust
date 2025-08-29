use std::any::type_name;
use std::ptr::{null, null_mut};
use zrdds::bindings::{DDS_DomainParticipant, DDS_DomainParticipantListener, DDS_DomainParticipantQos, DDS_Listener, DDS_DOMAINPARTICIPANT_QOS_DEFAULT, DDS_STATUS_MASK_NONE};
use zrdds::zrdds_interface::*;

fn main(){
    let dpf = dp_factory::DPFactory::instance().unwrap();

    let dp = dpf.create_dp(&dpf,1,null_mut(),null_mut(),DDS_STATUS_MASK_NONE).unwrap();

    let topic = dp.create_topic(&dp,"123","456",null_mut(),null_mut(),DDS_STATUS_MASK_NONE).unwrap();

    drop(dp);
    drop(topic);
}
