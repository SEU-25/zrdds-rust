use std::{ffi::CString, thread, time::Duration,ptr, mem};
use zrdds::bindings::*;
use serde_json::json;

use std::sync::{Arc, Mutex};
use eframe::egui;
use egui::Color32;
use serde_json::Value;

use std::collections::HashMap;

use whoami::*;

// 每个鼠标状态
#[derive(Clone)]
struct MouseState {
    username: String,
    color: egui::Color32,
    x: f32,
    y: f32,
}
static mut RECEIVED: Option<Arc<Mutex<HashMap<String, MouseState>>>> = None;

fn color_from_json(value: &Value) -> Color32 {
    if let Value::Array(arr) = value {
        let r = arr.get(0).and_then(|v| v.as_u64()).unwrap_or(0) as u8;
        let g = arr.get(1).and_then(|v| v.as_u64()).unwrap_or(0) as u8;
        let b = arr.get(2).and_then(|v| v.as_u64()).unwrap_or(0) as u8;
        let a = arr.get(3).and_then(|v| v.as_u64()).unwrap_or(0) as u8; // 不透明
        Color32::from_rgba_unmultiplied(r, g, b, a)
    } else {
        // 默认颜色
        Color32::WHITE
    }
}
fn main() {
    // 共享状态
    let received: Arc<Mutex<HashMap<String, MouseState>>> = Arc::new(Mutex::new(HashMap::new()));

unsafe {
    RECEIVED = Some(received.clone());
}

    unsafe {
        // DDS 初始化
        let factory = DDS_DomainParticipantFactory_get_instance();

        let dp_qos: *const DDS_DomainParticipantQos = unsafe {
            &raw const DDS_DOMAINPARTICIPANT_QOS_DEFAULT
        };
        let participant = DDS_DomainParticipantFactory_create_participant(
            factory,
            11,
            dp_qos,
            ptr::null_mut(),
            DDS_STATUS_MASK_NONE,
        );

        let type_name = DDS_BytesTypeSupport_get_type_name();
        DDS_BytesTypeSupport_register_type(participant, type_name);
        
        let topic_name = CString::new("mouse_topic").unwrap();
        let topic_qos: *const DDS_TopicQos = unsafe {
            &raw const DDS_TOPIC_QOS_DEFAULT
        };

        let topic = unsafe {
            DDS_DomainParticipant_create_topic(
                participant,
                topic_name.as_ptr(),
                type_name,
                topic_qos,
                ptr::null_mut(),
                DDS_STATUS_MASK_NONE,
            )
        };

        // 创建 Publisher
        let pu_qos: *const DDS_PublisherQos = unsafe {
            &raw const DDS_PUBLISHER_QOS_DEFAULT
        };
        let publisher = DDS_DomainParticipant_create_publisher(
            participant,
            pu_qos,
            ptr::null_mut(),
            DDS_STATUS_MASK_NONE,
        );

        let mut datawriter_qos: DDS_DataWriterQos = mem::zeroed();
        DDS_Publisher_get_default_datawriter_qos(publisher, &mut datawriter_qos);
        datawriter_qos.history.depth = 5;

        let writer = DDS_Publisher_create_datawriter(
            publisher,
            topic,
            &datawriter_qos,
            ptr::null_mut(),
            DDS_STATUS_MASK_NONE,
        ) as *mut DDS_DataWriter;

        // 用 Arc<Mutex<>> 包装 writer，传给 UI
        let writer = Arc::new(Mutex::new(writer));

                // 创建 Subscriber
        let su_qos: *const DDS_SubscriberQos = unsafe {
            &raw const DDS_SUBSCRIBER_QOS_DEFAULT
        };
        let subscriber = DDS_DomainParticipant_create_subscriber(
            participant,
            su_qos,
            ptr::null_mut(),
            DDS_STATUS_MASK_NONE,
        );

        //let received: Arc<Mutex<Vec<MouseState>>> = Arc::new(Mutex::new(Vec::new()));

        // 回调函数
        extern "C" fn on_data_available(reader: *mut DDS_DataReader) {
            unsafe {
                if reader.is_null() { return; }
                let reader = reader as *mut DDS_BytesDataReader;
                let mut data_values: DDS_BytesSeq = mem::zeroed();
                DDS_BytesSeq_initialize(&mut data_values);
                let mut sample_infos: DDS_SampleInfoSeq = mem::zeroed();
                DDS_SampleInfoSeq_initialize(&mut sample_infos);

                DDS_BytesDataReader_take(
                    reader,
                    &mut data_values,
                    &mut sample_infos,
                    MAX_INT32_VALUE as i32,//MAX_INT32_VALUE
                    DDS_ANY_SAMPLE_STATE,
                    DDS_ANY_VIEW_STATE,
                    DDS_ANY_INSTANCE_STATE,
                );

                for i in 0..sample_infos._length {
                    let sample_ptr = DDS_BytesSeq_get_reference(&mut data_values, i);
                    if !sample_ptr.is_null() {
                        let sample = &*sample_ptr;

                        let len = DDS_OctetSeq_get_length(&sample.value);
                        let mut vec = Vec::with_capacity(len as usize);

                        for j in 0..len {
                            let bptr = DDS_OctetSeq_get_reference(&sample.value, j);
                            if !bptr.is_null() {
                                vec.push(*bptr);
                            }
                        }

                        if let Ok(s) = String::from_utf8(vec) {

                            // 可以反序列化成 serde_json::Value 或自定义结构
                            if let Ok(mouse_state) = serde_json::from_str::<serde_json::Value>(&s) {
                                //println!("mouse username: {}", mouse_state["username"]);
                                if let Some(ref received_clone) = RECEIVED {
                                    let mut data = received_clone.lock().unwrap();
                                    data.insert(mouse_state["username"].to_string(),MouseState { username:mouse_state["username"].to_string(), color:color_from_json(&mouse_state["color"]),
                                     x: mouse_state["x"].as_f64().unwrap() as f32 , y: mouse_state["y"].as_f64().unwrap() as f32 });
                                }
                            }
                        } 
                    }
                }

                DDS_BytesDataReader_return_loan(reader, &mut data_values, &mut sample_infos);
            }
        }

        let mut listener: DDS_DataReaderListener = mem::zeroed();
        listener.on_data_available = Some(on_data_available);

        let mut datareader_qos: DDS_DataReaderQos = mem::zeroed();
        DDS_Subscriber_get_default_datareader_qos(subscriber, &mut datareader_qos);
        datareader_qos.history.depth = 5;

        let reader = DDS_Subscriber_create_datareader(
            subscriber,
            topic as *mut DDS_TopicDescription,
            &datareader_qos,
            &mut listener,
            DDS_STATUS_MASK_ALL,
        ) as *mut DDS_DataReader;


        let options = eframe::NativeOptions::default();
        eframe::run_native(
            "Shared Mouse Canvas",
            options,
            Box::new(move |cc| Box::new(MouseApp::new(received.clone(), writer.clone(), cc))),
        );
    }


    
}

struct MouseApp {
    received: Arc<Mutex<HashMap<String, MouseState>>>,
    writer: Arc<Mutex<*mut DDS_DataWriter>>,
    my_color: egui::Color32,
}

impl MouseApp {
    fn new(
        received: Arc<Mutex<HashMap<String, MouseState>>>,
        writer: Arc<Mutex<*mut DDS_DataWriter>>,
        _cc: &eframe::CreationContext,
    ) -> Self {
        Self {
            received,
            writer,
            my_color: egui::Color32::from_rgba_unmultiplied(255, 0, 0, 255), // 默认红色
        }
    }
}

impl eframe::App for MouseApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::CentralPanel::default().show(ctx, |ui| 
        {
            //颜色选择器
            ui.horizontal(|ui| {
                ui.color_edit_button_srgba(&mut self.my_color);
            });

            let painter = ui.painter();
            let data = self.received.lock().unwrap();
            for mouse in data.values() {
                let pos = egui::pos2(mouse.x, mouse.y);
                let color = egui::Color32::from_rgba_unmultiplied(
                    mouse.color[0], mouse.color[1], mouse.color[2], mouse.color[3],
                );

                // 1. 画一个小圆点当作“鼠标”
                painter.circle_filled(pos, 6.0, color);

                // 2. 在圆点旁边显示坐标 (文字)
                let text = format!("{} ({:.0}, {:.0})", mouse.username, mouse.x, mouse.y);
                painter.text(
                    pos + egui::vec2(10.0, -10.0),         // 偏移一点，不挡住圆点
                    egui::Align2::LEFT_TOP,
                    text,
                    egui::FontId::proportional(14.0),
                    egui::Color32::WHITE,
                );
            }
        });

        //获取系统用户名
        let username = whoami::username();
        // 使用界面选择的颜色
        let c = self.my_color;
        let color_arr = [c.r(), c.g(), c.b(), c.a()];

        // 采集本地鼠标位置并发送
        if let Some(pos) = ctx.input(|i| i.pointer.hover_pos()) {
            let mouse = json!({
                "username": username,              // 你可以改成本机用户名
                "color": color_arr,        // 红色
                "x": pos.x,
                "y": pos.y
            });

            let json_str = mouse.to_string();
            let buffer = json_str.as_bytes();

            let mut data: DDS_Bytes = unsafe { mem::zeroed() };
            unsafe { DDS_OctetSeq_initialize(&mut data.value as *mut DDS_OctetSeq) };

            unsafe {
                DDS_OctetSeq_loan_contiguous(
                    &mut data.value as *mut DDS_OctetSeq,
                    buffer.as_ptr() as *mut DDS_Octet,
                    buffer.len() as DDS_ULong,
                    buffer.len() as DDS_ULong,
                );

                let writer = *self.writer.lock().unwrap(); // 解锁拿到 writer
                let handle = DDS_BytesDataWriter_register_instance(writer as *mut DDS_BytesDataWriter, &mut data);
                DDS_BytesDataWriter_write(writer as *mut DDS_BytesDataWriter, &mut data, &handle);
            }
        }

        ctx.request_repaint();
    
    }
}
