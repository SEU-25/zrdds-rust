use std::ptr::null_mut;
use zrdds::bindings::*;
use zrdds::dds_simple_data_reader_listener;
use zrdds::zrdds_interface::*;

dds_simple_data_reader_listener!(Track, DDS_ZeroCopyBytes, {
    // 这里写样本处理逻辑（可直接用 these FFI 指针）
    println!("got a sample");
});

fn main() {
    let dpf = dp_factory::DPFactory::instance().unwrap();

    let dp = dpf
        .create_dp(&dpf, 1, null_mut(), null_mut(), DDS_STATUS_MASK_NONE)
        .unwrap();

    let topic = dp
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
    drop(topic);

    unsafe {
        self::Track_on_data_available(std::ptr::null_mut()); //虽然编译器报错，但编译可以通过
    }
}
