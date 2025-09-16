use std::ptr::null_mut;
use zrdds::bindings::{DDS_DomainParticipantQos, DDS_STATUS_MASK_NONE};
use zrdds::core::domain::DPFactory;

fn main() {
    let dpf = DPFactory::instance().unwrap();

    let dp = dpf
        .create_dp(
            &dpf,
            1,
            &dpf.default_qos().unwrap(),
            null_mut(),
            DDS_STATUS_MASK_NONE,
        )
        .unwrap();

    let _topic = dp
        .create_topic(
            &dp,
            "123",
            "456",
            null_mut(),
            null_mut(),
            DDS_STATUS_MASK_NONE,
        )
        .unwrap();

    // drop(dp); // 编译器报错
    // drop(_topic);
}
