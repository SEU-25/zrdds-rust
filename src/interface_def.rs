use crate::bindings::*;

pub struct DDS_DomainParticipantFactory;
pub struct DDS_DomainParticipant;
pub struct DDS_Subscriber;
pub struct DDS_Publisher;
pub struct DDS_DataReader<T>(std::marker::PhantomData<T>);
pub struct DDS_DataWriter<T>(std::marker::PhantomData<T>);

impl DDS_DomainParticipantFactory {
    pub fn get_instance() -> Option<DDS_DomainParticipantFactory>;
    pub fn finalize_instance() -> DDS_ReturnCode_t;
    pub fn create_participant(
        domain_id: DDS_DomainId_t,
        qos: Option<&DDS_DomainParticipantQos>,
        listener: Option<&DDS_DomainParticipantListener>,
        mask: DDS_StatusMask,
    ) -> Option<DDS_DomainParticipant>;
}

impl DDS_DomainParticipant {
    pub fn create_topic(
        &self,
        topic_name: &str,
        type_name: &str,
        qos: Option<&DDS_TopicQos>,
        listener: Option<&DDS_TopicListener>,
        mask: DDS_StatusMask,
    ) -> Option<DDS_Topic>;

    pub fn create_topic_with_type_support(
        &self,
        topic_name: &str,
        type_support: &DDS_TypeSupport,
        qos: Option<&DDS_TopicQos>,
        listener: Option<&DDS_TopicListener>,
        mask: DDS_StatusMask,
    ) -> Option<DDS_Topic>;

    pub fn create_subscriber(
        &self,
        qos: Option<&DDS_SubscriberQos>,
        listener: Option<&DDS_SubscriberListener>,
        mask: DDS_StatusMask,
    ) -> Option<DDS_Subscriber>;

    pub fn create_publisher(
        &self,
        qos: Option<&DDS_PublisherQos>,
        listener: Option<&DDS_PublisherListener>,
        mask: DDS_StatusMask,
    ) -> Option<DDS_Publisher>;

    pub fn get_builtin_subscriber(&self) -> &DDS_Subscriber;
}

impl DDS_Subscriber {
    pub fn create_datareader<T>(
        &self,
        topic: &DDS_TopicDescription,
        qos: Option<&DDS_DataReaderQos>,
        listener: Option<&DDS_DataReaderListener>,
        mask: DDS_StatusMask,
    ) -> Option<DDS_DataReader<T>>;

    pub fn lookup_datareader(&self, topic_name: &str) -> Option<DDS_DataReader>;
}

impl DDS_Publisher {
    pub fn create_datawriter<T>(
        &self,
        topic: &DDS_Topic,
        qos: Option<&DDS_DataWriterQos>,
        listener: Option<&DDS_DataWriterListener>,
        mask: DDS_StatusMask,
    ) -> Option<DDS_DataWriter<T>>;
}

impl<T> DDS_DataReader<T> {
    pub fn set_listener(
        &mut self,
        listener: Option<&DDS_DataReaderListener>,
        mask: DDS_StatusMask,
    ) -> DDS_ReturnCode_t;
}

impl<T> DDS_DataWriter<T> {
    pub fn write(
        &self,
        sample: &T,
        handle: Option<&DDS_InstanceHandle_t>,
    ) -> DDS_ReturnCode_t;

    pub fn write_with_timestamp(
        &self,
        sample: &T,
        handle: Option<&DDS_InstanceHandle_t>,
        timestamp: &DDS_Time_t,
    ) -> DDS_ReturnCode_t;

    pub fn write_with_destination(
        &self,
        sample: &T,
        handle: Option<&DDS_InstanceHandle_t>,
        timestamp: Option<&DDS_Time_t>,
        dst_handle: &DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t;
}

/// 数据读取器监听器特征
pub trait DDS_DataReaderListener {
    fn on_requested_deadline_missed(
        &self,
        reader: &DDS_DataReader,
        status: &DDS_RequestedDeadlineMissedStatus,
    );

    fn on_requested_incompatible_qos(
        &self,
        reader: &DDS_DataReader,
        status: &DDS_RequestedIncompatibleQosStatus,
    );

    fn on_sample_rejected(
        &self,
        reader: &DDS_DataReader,
        status: &DDS_SampleRejectedStatus,
    );

    fn on_sample_lost(
        &self,
        reader: &DDS_DataReader,
        status: &DDS_SampleLostStatus,
    );

    fn on_liveliness_changed(
        &self,
        reader: &DDS_DataReader,
        status: &DDS_LivelinessChangedStatus,
    );

    fn on_data_available(&self, reader: &DDS_DataReader);

    fn on_data_arrived(&self, reader: &DDS_DataReader);

    fn on_subscription_matched(
        &self,
        reader: &DDS_DataReader,
        status: &DDS_SubscriptionMatchedStatus,
    );
}
