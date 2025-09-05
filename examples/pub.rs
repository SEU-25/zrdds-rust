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

    let publisher = dp.create_publisher(
        &dp.default_publisher_qos().unwrap(),
        &PublisherListener::default(),
        DDS_STATUS_MASK_NONE,
    ).unwrap();

    let topic = dp.create_topic(
        "HelloWorld",
        "HelloWorld",
        &dp.default_topic_qos().unwrap(),
        &mut TopicListener::default(),
        DDS_STATUS_MASK_NONE,
    ).unwrap();

    let writer = publisher.create_writer(
        topic,
        &dp.default_datawriter_qos().unwrap(),
        &mut DataWriterListener::default(),
        DDS_STATUS_MASK_NONE,
    ).unwrap();
}
