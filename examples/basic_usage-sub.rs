use std::ffi::CString;
use zrdds;
use zrdds::bindings::*;

extern "C" fn on_data_available(reader: *mut DDS_DataReader) {
    unsafe {
        let topic_desc = DataReaderImplGetTopicDescription(reader);
        let topic_name = DDS_TopicDescription_get_name(topic_desc);
        let topic = std::ffi::CStr::from_ptr(topic_name).to_string_lossy();

        println!("📩 有新数据到达: {}", topic);

        // TODO: 这里可以继续调用 DDS 的 API 取数据
    }
}

fn make_listener() -> DDS_DataReaderListener {
    DDS_DataReaderListener {
        listener: DDS_Listener { 
            user_data: std::ptr::null_mut(),  // 👈 这里补上
        },
        on_requested_deadline_missed: None,
        on_requested_incompatible_qos: None,
        on_sample_rejected: None,
        on_sample_lost: None,
        on_liveliness_changed: None,
        on_data_available: Some(on_data_available), // 👈 注册回调
        on_data_arrived: None,
        on_subscription_matched: None,
    }
}
fn main() {
    // 初始化工厂
    let dp_factory = zrdds::init("ZRDDS_QOS_PROFILES.xml", "non_rio");

    // 创建参与者
    let dp = dp_factory.create_domain_participant(150, "udp_dp");


    let mut listener = make_listener();

    // 订阅 MyTopic
    let reader = dp.subscribe_with_listener(
        "MyTopic",
        unsafe { &raw mut DDS_BytesTypeSupport_instance },
        "non_zerocopy_reliable",
        &mut listener,
    );

    println!("✅ 等待数据中... 按 Enter 退出");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}