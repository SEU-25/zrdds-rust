use std::ffi::CString;
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

    // 写普通 Bytes
    let data = b"Hello DDS";
    let ret = zrdds::bytes_write(150, "MyTopic", data);
    if ret != 0 {
        panic!("写普通 Bytes 失败");
    }
}
