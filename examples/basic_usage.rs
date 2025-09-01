use std::{ffi::CString, thread, ptr, mem};
use std::sync::{Arc, Mutex, mpsc::{channel, Sender, Receiver}};
use std::collections::HashMap;
use std::time::Duration;

use eframe::egui;
use egui::Color32;
use scrap::{Capturer, Display};
use image::*;
use base64::*;
use once_cell::sync::OnceCell;
use serde_json::json;
use serde_json::Value;
use whoami::*;
use zrdds::bindings::*;

#[derive(Clone)]
struct MouseState {
    username: String,
    color: Color32,
    x: f32,
    y: f32,
}

static mut RECEIVED: Option<Arc<Mutex<HashMap<String, MouseState>>>> = None;
static RECEIVED_SCREEN: OnceCell<Arc<Mutex<Option<Vec<u8>>>>> = OnceCell::new();

fn color_from_json(value: &Value) -> Color32 {
    if let Value::Array(arr) = value {
        let r = arr.get(0).and_then(|v| v.as_u64()).unwrap_or(0) as u8;
        let g = arr.get(1).and_then(|v| v.as_u64()).unwrap_or(0) as u8;
        let b = arr.get(2).and_then(|v| v.as_u64()).unwrap_or(0) as u8;
        let a = arr.get(3).and_then(|v| v.as_u64()).unwrap_or(255) as u8;
        Color32::from_rgba_unmultiplied(r, g, b, a)
    } else {
        Color32::WHITE
    }
}

fn main() {
    let received: Arc<Mutex<HashMap<String, MouseState>>> = Arc::new(Mutex::new(HashMap::new()));
    let received_screen: Arc<Mutex<Option<Vec<u8>>>> = Arc::new(Mutex::new(None));
    unsafe { RECEIVED = Some(received.clone()); }
    RECEIVED_SCREEN.set(received_screen.clone()).unwrap();

    unsafe {
        // DDS 初始化略（与原逻辑一样）
        let factory = DDS_DomainParticipantFactory_get_instance();
        let dp_qos: *const DDS_DomainParticipantQos = &raw const DDS_DOMAINPARTICIPANT_QOS_DEFAULT;
        let participant = DDS_DomainParticipantFactory_create_participant(
            factory, 11, dp_qos, ptr::null_mut(), DDS_STATUS_MASK_NONE
        );

        let type_name = DDS_BytesTypeSupport_get_type_name();
        DDS_BytesTypeSupport_register_type(participant, type_name);

        let topic_name = CString::new("mouse_topic").unwrap();
        let topic_qos: *const DDS_TopicQos = &raw const DDS_TOPIC_QOS_DEFAULT;
        let topic = DDS_DomainParticipant_create_topic(
            participant,
            topic_name.as_ptr(),
            type_name,
            topic_qos,
            ptr::null_mut(),
            DDS_STATUS_MASK_NONE,
        );

        let pu_qos: *const DDS_PublisherQos = &raw const DDS_PUBLISHER_QOS_DEFAULT;
        let publisher = DDS_DomainParticipant_create_publisher(participant, pu_qos, ptr::null_mut(), DDS_STATUS_MASK_NONE);

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

        let writer = Arc::new(Mutex::new(writer));

        // Subscriber 逻辑略，与原来一致

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
    my_color: Color32,
    screen_img: Option<egui::ColorImage>,
    screen_tx: Sender<Vec<u8>>,
    screen_rx: Receiver<Vec<u8>>,
}

impl MouseApp {
    fn new(
        received: Arc<Mutex<HashMap<String, MouseState>>>,
        writer: Arc<Mutex<*mut DDS_DataWriter>>,
        _cc: &eframe::CreationContext,
    ) -> Self {
        let (tx, rx) = channel::<Vec<u8>>();

        // 用 start_screen_capture 启动后台线程
        start_screen_capture(tx.clone());

        Self {
            received,
            writer,
            my_color: Color32::from_rgba_unmultiplied(255, 0, 0, 255),
            screen_img: None,
            screen_tx: tx,
            screen_rx: rx,
        }
    }
}

impl eframe::App for MouseApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // UI 逻辑
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.color_edit_button_srgba(&mut self.my_color);
            });

            if ui.button("share").clicked() {
                // 不需要在按钮里做阻塞捕获，后台线程已经在采集
            }

            // 接收屏幕帧
            while let Ok(jpeg_bytes) = self.screen_rx.try_recv() {
                if let Ok(img) = image::load_from_memory(&jpeg_bytes) {
                    let size = [img.width() as usize, img.height() as usize];
                    self.screen_img = Some(egui::ColorImage::from_rgba_unmultiplied(size, &img.to_rgba8()));

                    // DDS 发送放在这里，确保 img 还在作用域
                    let username = whoami::username();
                    let msg = json!({
                        "type": "screen",
                        "username": username,
                        "width": img.width(),     // 这里 img 还有效
                        "height": img.height(),
                        "data_bytes": jpeg_bytes   // 直接字节流
                    });
                    let json_str = msg.to_string();
                    let buffer = json_str.as_bytes();

                    unsafe {
                        let mut data: DDS_Bytes = std::mem::zeroed();
                        DDS_OctetSeq_initialize(&mut data.value as *mut DDS_OctetSeq);
                        DDS_OctetSeq_loan_contiguous(
                            &mut data.value as *mut DDS_OctetSeq,
                            buffer.as_ptr() as *mut DDS_Octet,
                            buffer.len() as DDS_ULong,
                            buffer.len() as DDS_ULong,
                        );
                        let writer = *self.writer.lock().unwrap();
                        let handle = DDS_BytesDataWriter_register_instance(writer as *mut DDS_BytesDataWriter, &mut data);
                        DDS_BytesDataWriter_write(writer as *mut DDS_BytesDataWriter, &mut data, &handle);
                    }
                }
            }
            // 显示画面
            if let Some(ref img) = self.screen_img { 
                let texture = ui.ctx().load_texture( "screen_share", img.clone(), egui::TextureOptions::LINEAR, ); // 指定显示尺寸，比如按原图大小：
                let size = egui::Vec2::new(img.width() as f32, img.height() as f32); ui.image(&texture);
            }

            // 绘制鼠标
            let painter = ui.painter();
            let data = self.received.lock().unwrap();
            for mouse in data.values() {
                painter.circle_filled(egui::pos2(mouse.x, mouse.y), 6.0, mouse.color);
                painter.text(egui::pos2(mouse.x + 10.0, mouse.y - 10.0), egui::Align2::LEFT_TOP, &mouse.username, egui::FontId::proportional(14.0), Color32::WHITE);
            }
        });

        // 采集本地鼠标并发送
        if let Some(pos) = ctx.input(|i| i.pointer.hover_pos()) {
            let username = whoami::username();
            let c = self.my_color;
            let color_arr = [c.r(), c.g(), c.b(), c.a()];
            let mouse = json!({
                "type": "mouse",
                "username": username,
                "color": color_arr,
                "x": pos.x,
                "y": pos.y
            });
            let json_str = mouse.to_string();
            let buffer = json_str.as_bytes();
            unsafe {
                let mut data: DDS_Bytes = mem::zeroed();
                DDS_OctetSeq_initialize(&mut data.value as *mut DDS_OctetSeq);
                DDS_OctetSeq_loan_contiguous(&mut data.value as *mut DDS_OctetSeq, buffer.as_ptr() as *mut DDS_Octet, buffer.len() as DDS_ULong, buffer.len() as DDS_ULong);

                let writer = *self.writer.lock().unwrap();
                let handle = DDS_BytesDataWriter_register_instance(writer as *mut DDS_BytesDataWriter, &mut data);
                DDS_BytesDataWriter_write(writer as *mut DDS_BytesDataWriter, &mut data, &handle);
            }
        }

        ctx.request_repaint();
    }
}

// 捕获屏幕
fn capture_screen() -> Option<Vec<u8>> {
    let display = Display::primary().unwrap();
    let mut capturer = Capturer::new(display).unwrap();
    loop {
        match capturer.frame() {
            Ok(frame) => return Some(frame.to_vec()),
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_millis(10));
            }
            Err(e) => panic!("Error: {}", e),
        }
    }
}

// PNG 编码
fn encode_png(width: u32, height: u32, data: &[u8]) -> Vec<u8> {
    let buffer: ImageBuffer<Rgba<u8>, _> = ImageBuffer::from_raw(width, height, data.to_vec()).unwrap();
    let mut png_bytes: Vec<u8> = Vec::new();
    buffer.write_to(&mut std::io::Cursor::new(&mut png_bytes), image::ImageOutputFormat::Png).unwrap();
    png_bytes
}

fn start_screen_capture(screen_tx: Sender<Vec<u8>>) {
    std::thread::spawn(move || {
        let display = Display::primary().unwrap();
        let width = display.width();
        let height = display.height();
        let mut capturer = Capturer::new(display).unwrap();

        loop {
            match capturer.frame() {
                Ok(frame) => {
                    let mut buf = Vec::new();
                    let _ = image::codecs::png::PngEncoder::new(&mut buf)
                        .encode(&frame, width as u32, height as u32, image::ColorType::Rgba8);
                    let _ = screen_tx.send(buf);
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(10));
                }
                Err(e) => panic!("Error capturing screen: {:?}", e),
            }
        }
    });
}