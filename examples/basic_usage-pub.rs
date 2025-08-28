use std::{ffi::CString, thread, time::Duration};
use zrdds;
use zrdds::bindings::*;

fn main() {
    let _dp_factory = zrdds::init("ZRDDS_QOS_PROFILES.xml", "non_rio");

    let dp = _dp_factory.create_domain_participant(150, "udp_dp");

    // 创建写者
    let topic = CString::new("MyTopic").unwrap();
    let writer = dp.publish(
        "MyTopic",
        unsafe { &raw mut DDS_BytesTypeSupport_instance },
        "non_zerocopy_reliable",
    );

    loop {
        let data = b"Hello DDS";

        let ret = zrdds::bytes_write(150, "MyTopic", data);
        if ret != 0 {
            eprintln!("❌ 发送数据失败: {}", ret);
        } else {
            println!("✅ 发送数据: ");
        }

        // 每隔 1 秒发送一次
        thread::sleep(Duration::from_secs(1));
    }
}
