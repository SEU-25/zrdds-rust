use zrdds::{self, DomainParticipantListener, PublisherListener, bindings::DDS_STATUS_MASK_NONE};

fn main() {
    let dpf = zrdds::DPFactory::instance().unwrap();
    let dp = dpf
        .create_dp(
            150,
            &dpf.default_qos().unwrap(),
            &mut DomainParticipantListener::default(),
            DDS_STATUS_MASK_NONE,
        )
        .unwrap();

    dp.create_publisher(
        dp.default_subscriber_qos(),
        PublisherListener::default(),
        DDS_STATUS_MASK_NONE,
    );
}
