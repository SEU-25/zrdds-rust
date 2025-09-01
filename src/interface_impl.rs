use crate::bindings::*;

impl DDS_DomainParticipantFactory {
    pub fn get_instance() -> Option<DDS_DomainParticipantFactory> {
        todo!("获取域参与者工厂单例对象")
    }

    pub fn finalize_instance() -> DDS_ReturnCode_t {
        todo!("析构单例")
    }

    pub fn create_participant(
        domain_id: DDS_DomainId_t,
        qos: Option<&DDS_DomainParticipantQos>,
        listener: Option<&DDS_DomainParticipantListener>,
        mask: DDS_StatusMask,
    ) -> Option<DDS_DomainParticipant> {
        todo!("创建域参与者")
    }
}

impl DDS_DomainParticipant {
    pub fn create_topic(
        &self,
        topic_name: &str,
        type_name: &str,
        qos: Option<&DDS_TopicQos>,
        listener: Option<&DDS_TopicListener>,
        mask: DDS_StatusMask,
    ) -> Option<DDS_Topic> {
        todo!("创建主题")
    }

    pub fn create_topic_with_type_support(
        &self,
        topic_name: &str,
        type_support: &DDS_TypeSupport,
        qos: Option<&DDS_TopicQos>,
        listener: Option<&DDS_TopicListener>,
        mask: DDS_StatusMask,
    ) -> Option<DDS_Topic> {
        todo!("创建带类型支持的主题")
    }

    pub fn create_subscriber(
        &self,
        qos: Option<&DDS_SubscriberQos>,
        listener: Option<&DDS_SubscriberListener>,
        mask: DDS_StatusMask,
    ) -> Option<DDS_Subscriber> {
        todo!("创建订阅者")
    }

    pub fn create_publisher(
        &self,
        qos: Option<&DDS_PublisherQos>,
        listener: Option<&DDS_PublisherListener>,
        mask: DDS_StatusMask,
    ) -> Option<DDS_Publisher> {
        todo!("创建发布者")
    }

    pub fn get_builtin_subscriber(&self) -> &DDS_Subscriber {
        todo!("获取内置订阅者")
    }
}

impl DDS_Subscriber {
    pub fn create_datareader<T>(
        &self,
        topic: &DDS_TopicDescription,
        qos: Option<&DDS_DataReaderQos>,
        listener: Option<&DDS_DataReaderListener>,
        mask: DDS_StatusMask,
    ) -> Option<DDS_DataReader<T>> {
        todo!("创建数据读取器")
    }

    pub fn lookup_datareader(&self, topic_name: &str) -> Option<DDS_DataReader> {
        todo!("查找数据读取器")
    }
}

impl DDS_Publisher {
    pub fn create_datawriter<T>(
        &self,
        topic: &DDS_Topic,
        qos: Option<&DDS_DataWriterQos>,
        listener: Option<&DDS_DataWriterListener>,
        mask: DDS_StatusMask,
    ) -> Option<DDS_DataWriter<T>> {
        todo!("创建数据写入器")
    }
}

impl<T> DDS_DataReader<T> {
    pub fn set_listener(
        &mut self,
        listener: Option<&DDS_DataReaderListener>,
        mask: DDS_StatusMask,
    ) -> DDS_ReturnCode_t {
        todo!("设置数据读取器监听器")
    }
}

impl<T> DDS_DataWriter<T> {
    pub fn write(
        &self,
        sample: &T,
        handle: Option<&DDS_InstanceHandle_t>,
    ) -> DDS_ReturnCode_t {
        todo!("写入数据")
    }

    pub fn write_with_timestamp(
        &self,
        sample: &T,
        handle: Option<&DDS_InstanceHandle_t>,
        timestamp: &DDS_Time_t,
    ) -> DDS_ReturnCode_t {
        todo!("带时间戳写入数据")
    }

    pub fn write_with_destination(
        &self,
        sample: &T,
        handle: Option<&DDS_InstanceHandle_t>,
        timestamp: Option<&DDS_Time_t>,
        dst_handle: &DDS_InstanceHandle_t,
    ) -> DDS_ReturnCode_t {
        todo!("带目标写入数据")
    }
}
