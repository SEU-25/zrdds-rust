use std::{ffi::CString, thread, time::Duration,ptr, mem};
use zrdds::bindings::*;
use serde_json::json;

use std::sync::{Arc, Mutex};
use eframe::egui;
use egui::Color32;
use serde_json::Value;

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use image::{ImageFormat, DynamicImage, GenericImageView};
use base64::{Engine as _, engine::general_purpose};
use chrono;

use whoami::*;

// 消息类型枚举
#[derive(Clone, Debug)]
enum MessageType {
    Mouse,
    Image,
    Draw,
    Erase,
    ImageDelete,
}

// 每个鼠标状态
#[derive(Clone)]
struct MouseState {
    username: String,
    color: egui::Color32,
    x: f32,
    y: f32,
}

// 图片数据结构
#[derive(Clone)]
struct ImageData {
    username: String,
    image_data: Vec<u8>, // PNG图片的原始字节数据
    width: u32,
    height: u32,
}

// 画笔笔迹数据结构
#[derive(Clone)]
struct DrawStroke {
    username: String,
    color: egui::Color32,
    start_x: f32,
    start_y: f32,
    end_x: f32,
    end_y: f32,
    stroke_width: f32,
}

// 擦除操作数据结构
#[derive(Clone)]
struct EraseOperation {
    username: String,
    x: f32,
    y: f32,
    radius: f32, // 擦除半径
}

// 图片删除操作数据结构
#[derive(Clone)]
struct ImageDeleteOperation {
    username: String,
    image_id: String, // 图片的唯一标识符（使用用户名作为ID）
}

// 聊天消息数据结构
#[derive(Clone)]
struct ChatMessage {
    username: String,
    message: String,
    timestamp: String, // 时间戳
}

static mut RECEIVED: Option<Arc<Mutex<HashMap<String, MouseState>>>> = None;
static mut RECEIVED_IMAGES: Option<Arc<Mutex<HashMap<String, ImageData>>>> = None;
static mut RECEIVED_STROKES: Option<Arc<Mutex<Vec<DrawStroke>>>> = None;
static mut RECEIVED_ERASES: Option<Arc<Mutex<Vec<EraseOperation>>>> = None;
static mut RECEIVED_IMAGE_DELETES: Option<Arc<Mutex<Vec<ImageDeleteOperation>>>> = None;
static mut RECEIVED_CHAT_MESSAGES: Option<Arc<Mutex<Vec<ChatMessage>>>> = None;

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

// 检测线段与圆形是否相交的函数
fn line_intersects_circle(x1: f32, y1: f32, x2: f32, y2: f32, cx: f32, cy: f32, radius: f32) -> bool {
    // 计算线段向量
    let dx = x2 - x1;
    let dy = y2 - y1;
    
    // 计算从线段起点到圆心的向量
    let fx = x1 - cx;
    let fy = y1 - cy;
    
    // 计算二次方程的系数
    let a = dx * dx + dy * dy;
    let b = 2.0 * (fx * dx + fy * dy);
    let c = (fx * fx + fy * fy) - radius * radius;
    
    // 计算判别式
    let discriminant = b * b - 4.0 * a * c;
    
    if discriminant < 0.0 {
        return false; // 没有交点
    }
    
    // 计算交点参数
    let discriminant = discriminant.sqrt();
    let t1 = (-b - discriminant) / (2.0 * a);
    let t2 = (-b + discriminant) / (2.0 * a);
    
    // 检查交点是否在线段上（t在0到1之间）
    (t1 >= 0.0 && t1 <= 1.0) || (t2 >= 0.0 && t2 <= 1.0) || (t1 < 0.0 && t2 > 1.0)
}

// 读取PNG图片文件并转换为base64
fn load_image_as_base64(file_path: &str) -> Result<(String, u32, u32), Box<dyn std::error::Error>> {
    let image_bytes = fs::read(file_path)?;
    let img = image::load_from_memory(&image_bytes)?;
    let (width, height) = img.dimensions();
    
    // 将图片转换为PNG格式的字节数组
    let mut png_bytes = Vec::new();
    {
        let mut cursor = std::io::Cursor::new(&mut png_bytes);
        img.write_to(&mut cursor, ImageFormat::Png)?;
    }
    
    let base64_string = general_purpose::STANDARD.encode(&png_bytes);
    Ok((base64_string, width, height))
}
fn main() {
    // 共享状态
    let received: Arc<Mutex<HashMap<String, MouseState>>> = Arc::new(Mutex::new(HashMap::new()));
    let received_images: Arc<Mutex<HashMap<String, ImageData>>> = Arc::new(Mutex::new(HashMap::new()));
    let received_strokes: Arc<Mutex<Vec<DrawStroke>>> = Arc::new(Mutex::new(Vec::new()));
    let received_erases: Arc<Mutex<Vec<EraseOperation>>> = Arc::new(Mutex::new(Vec::new()));
    let received_image_deletes: Arc<Mutex<Vec<ImageDeleteOperation>>> = Arc::new(Mutex::new(Vec::new()));
    let received_chat_messages: Arc<Mutex<Vec<ChatMessage>>> = Arc::new(Mutex::new(Vec::new()));

unsafe {
    RECEIVED = Some(received.clone());
    RECEIVED_IMAGES = Some(received_images.clone());
    RECEIVED_STROKES = Some(received_strokes.clone());
    RECEIVED_ERASES = Some(received_erases.clone());
    RECEIVED_IMAGE_DELETES = Some(received_image_deletes.clone());
    RECEIVED_CHAT_MESSAGES = Some(received_chat_messages.clone());
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

        // 创建图片topic
        let image_topic_name = CString::new("image_topic").unwrap();
        let image_topic = unsafe {
            DDS_DomainParticipant_create_topic(
                participant,
                image_topic_name.as_ptr(),
                type_name,
                topic_qos,
                ptr::null_mut(),
                DDS_STATUS_MASK_NONE,
            )
        };
        
        // 创建画笔笔迹topic
        let draw_topic_name = CString::new("draw_topic").unwrap();
        let draw_topic = unsafe {
            DDS_DomainParticipant_create_topic(
                participant,
                draw_topic_name.as_ptr(),
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

        // 创建图片数据writer
        let image_writer = DDS_Publisher_create_datawriter(
            publisher,
            image_topic,
            &datawriter_qos,
            ptr::null_mut(),
            DDS_STATUS_MASK_NONE,
        ) as *mut DDS_DataWriter;
        
        // 创建画笔笔迹数据writer
        let draw_writer = DDS_Publisher_create_datawriter(
            publisher,
            draw_topic,
            &datawriter_qos,
            ptr::null_mut(),
            DDS_STATUS_MASK_NONE,
        ) as *mut DDS_DataWriter;
        
        // 创建擦除操作topic
        let erase_topic_name = CString::new("erase_topic").unwrap();
        let erase_topic = unsafe {
            DDS_DomainParticipant_create_topic(
                participant,
                erase_topic_name.as_ptr(),
                type_name,
                topic_qos,
                ptr::null_mut(),
                DDS_STATUS_MASK_NONE,
            )
        };
        
        // 创建擦除操作数据writer
        let erase_writer = DDS_Publisher_create_datawriter(
            publisher,
            erase_topic,
            &datawriter_qos,
            ptr::null_mut(),
            DDS_STATUS_MASK_NONE,
        ) as *mut DDS_DataWriter;
        
        // 创建图片删除操作topic
        let image_delete_topic_name = CString::new("image_delete_topic").unwrap();
        let image_delete_topic = unsafe {
            DDS_DomainParticipant_create_topic(
                participant,
                image_delete_topic_name.as_ptr(),
                type_name,
                topic_qos,
                ptr::null_mut(),
                DDS_STATUS_MASK_NONE,
            )
        };
        
        // 创建图片删除操作数据writer
        let image_delete_writer = DDS_Publisher_create_datawriter(
            publisher,
            image_delete_topic,
            &datawriter_qos,
            ptr::null_mut(),
            DDS_STATUS_MASK_NONE,
        ) as *mut DDS_DataWriter;
        
        // 创建聊天消息topic
        let chat_topic_name = CString::new("chat_topic").unwrap();
        let chat_topic = unsafe {
            DDS_DomainParticipant_create_topic(
                participant,
                chat_topic_name.as_ptr(),
                type_name,
                topic_qos,
                ptr::null_mut(),
                DDS_STATUS_MASK_NONE,
            )
        };
        
        // 创建聊天消息数据writer
        let chat_writer = DDS_Publisher_create_datawriter(
            publisher,
            chat_topic,
            &datawriter_qos,
            ptr::null_mut(),
            DDS_STATUS_MASK_NONE,
        ) as *mut DDS_DataWriter;

        // 用 Arc<Mutex<>> 包装 writer，传给 UI
        let writer = Arc::new(Mutex::new(writer));
        let image_writer = Arc::new(Mutex::new(image_writer));
        let draw_writer = Arc::new(Mutex::new(draw_writer));
        let erase_writer = Arc::new(Mutex::new(erase_writer));
        let image_delete_writer = Arc::new(Mutex::new(image_delete_writer));
        let chat_writer = Arc::new(Mutex::new(chat_writer));

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

        // 画笔笔迹消息回调函数
        extern "C" fn on_draw_data_available(reader: *mut DDS_DataReader) {
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
                    MAX_INT32_VALUE as i32,
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
                            if let Ok(draw_msg) = serde_json::from_str::<serde_json::Value>(&s) {
                                if draw_msg["type"] == "Draw" {
                                    if let Some(ref received_strokes_clone) = RECEIVED_STROKES {
                                        let color_array = draw_msg["color"].as_array().unwrap();
                                        let color = egui::Color32::from_rgba_unmultiplied(
                                            color_array[0].as_u64().unwrap() as u8,
                                            color_array[1].as_u64().unwrap() as u8,
                                            color_array[2].as_u64().unwrap() as u8,
                                            color_array[3].as_u64().unwrap() as u8,
                                        );
                                        
                                        let stroke = DrawStroke {
                                            username: draw_msg["username"].as_str().unwrap_or("unknown").to_string(),
                                            color,
                                            start_x: draw_msg["start_x"].as_f64().unwrap_or(0.0) as f32,
                                            start_y: draw_msg["start_y"].as_f64().unwrap_or(0.0) as f32,
                                            end_x: draw_msg["end_x"].as_f64().unwrap_or(0.0) as f32,
                                            end_y: draw_msg["end_y"].as_f64().unwrap_or(0.0) as f32,
                                            stroke_width: draw_msg["stroke_width"].as_f64().unwrap_or(2.0) as f32,
                                        };
                                        
                                        let mut data = received_strokes_clone.lock().unwrap();
                                        data.push(stroke);
                                    }
                                }
                            }
                        }
                    }
                }

                DDS_BytesDataReader_return_loan(reader, &mut data_values, &mut sample_infos);
            }
        }

        // 图片消息回调函数
        extern "C" fn on_image_data_available(reader: *mut DDS_DataReader) {
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
                    MAX_INT32_VALUE as i32,
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
                            if let Ok(image_msg) = serde_json::from_str::<serde_json::Value>(&s) {
                                if let Some(ref received_images_clone) = RECEIVED_IMAGES {
                                    if let Some(image_data_b64) = image_msg["image_data"].as_str() {
                                        if let Ok(image_bytes) = general_purpose::STANDARD.decode(image_data_b64) {
                                            let mut data = received_images_clone.lock().unwrap();
                                            data.insert(
                                                image_msg["username"].as_str().unwrap_or("unknown").to_string(),
                                                ImageData {
                                                    username: image_msg["username"].as_str().unwrap_or("unknown").to_string(),
                                                    image_data: image_bytes,
                                                    width: image_msg["width"].as_u64().unwrap_or(0) as u32,
                                                    height: image_msg["height"].as_u64().unwrap_or(0) as u32,
                                                }
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                DDS_BytesDataReader_return_loan(reader, &mut data_values, &mut sample_infos);
            }
        }

        // 擦除消息回调函数
        extern "C" fn on_erase_data_available(reader: *mut DDS_DataReader) {
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
                    MAX_INT32_VALUE as i32,
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
                            if let Ok(erase_msg) = serde_json::from_str::<serde_json::Value>(&s) {
                                if let Some(ref received_erases_clone) = RECEIVED_ERASES {
                                    let mut data = received_erases_clone.lock().unwrap();
                                    data.push(EraseOperation {
                                        username: erase_msg["username"].as_str().unwrap_or("unknown").to_string(),
                                        x: erase_msg["x"].as_f64().unwrap_or(0.0) as f32,
                                        y: erase_msg["y"].as_f64().unwrap_or(0.0) as f32,
                                        radius: erase_msg["radius"].as_f64().unwrap_or(20.0) as f32,
                                    });
                                }
                            }
                        }
                    }
                }

                DDS_BytesDataReader_return_loan(reader, &mut data_values, &mut sample_infos);
    }
}

// 图片删除消息回调函数
extern "C" fn on_image_delete_data_available(reader: *mut DDS_DataReader) {
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
            MAX_INT32_VALUE as i32,
            DDS_ANY_SAMPLE_STATE,
            DDS_ANY_VIEW_STATE,
            DDS_ANY_INSTANCE_STATE,
        );

        for i in 0..sample_infos._length {
            let sample_ptr = DDS_BytesSeq_get_reference(&mut data_values, i);
            if !sample_ptr.is_null() {
                let sample = &*sample_ptr;
                let len = DDS_OctetSeq_get_length(&sample.value);
                let mut vec = Vec::new();

                for j in 0..len {
                    let bptr = DDS_OctetSeq_get_reference(&sample.value, j);
                    if !bptr.is_null() {
                        vec.push(*bptr);
                    }
                }

                if let Ok(s) = String::from_utf8(vec) {
                    if let Ok(delete_msg) = serde_json::from_str::<serde_json::Value>(&s) {
                        if let Some(ref received_image_deletes_clone) = RECEIVED_IMAGE_DELETES {
                            let mut data = received_image_deletes_clone.lock().unwrap();
                            data.push(ImageDeleteOperation {
                                username: delete_msg["username"].as_str().unwrap_or("unknown").to_string(),
                                image_id: delete_msg["image_id"].as_str().unwrap_or("").to_string(),
                            });
                        }
                    }
                }
            }
        }

        DDS_BytesDataReader_return_loan(reader, &mut data_values, &mut sample_infos);
    }
}

// 聊天消息回调函数
extern "C" fn on_chat_data_available(reader: *mut DDS_DataReader) {
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
            MAX_INT32_VALUE as i32,
            DDS_ANY_SAMPLE_STATE,
            DDS_ANY_VIEW_STATE,
            DDS_ANY_INSTANCE_STATE,
        );

        for i in 0..sample_infos._length {
            let sample_ptr = DDS_BytesSeq_get_reference(&mut data_values, i);
            if !sample_ptr.is_null() {
                let sample = &*sample_ptr;
                let len = DDS_OctetSeq_get_length(&sample.value);
                let mut vec = Vec::new();

                for j in 0..len {
                    let bptr = DDS_OctetSeq_get_reference(&sample.value, j);
                    if !bptr.is_null() {
                        vec.push(*bptr);
                    }
                }

                if let Ok(s) = String::from_utf8(vec) {
                    if let Ok(chat_msg) = serde_json::from_str::<serde_json::Value>(&s) {
                        if let Some(ref received_chat_messages_clone) = RECEIVED_CHAT_MESSAGES {
                            let mut data = received_chat_messages_clone.lock().unwrap();
                            data.push(ChatMessage {
                                username: chat_msg["username"].as_str().unwrap_or("unknown").to_string(),
                                message: chat_msg["message"].as_str().unwrap_or("").to_string(),
                                timestamp: chat_msg["timestamp"].as_str().unwrap_or("").to_string(),
                            });
                        }
                    }
                }
            }
        }

        DDS_BytesDataReader_return_loan(reader, &mut data_values, &mut sample_infos);
    }
}

// 鼠标消息回调函数
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

        // 创建图片消息的listener和reader
        let mut image_listener: DDS_DataReaderListener = mem::zeroed();
        image_listener.on_data_available = Some(on_image_data_available);

        let image_reader = DDS_Subscriber_create_datareader(
            subscriber,
            image_topic as *mut DDS_TopicDescription,
            &datareader_qos,
            &mut image_listener,
            DDS_STATUS_MASK_ALL,
        ) as *mut DDS_DataReader;
        
        // 创建画笔笔迹消息的listener和reader
        let mut draw_listener: DDS_DataReaderListener = mem::zeroed();
        draw_listener.on_data_available = Some(on_draw_data_available);

        let draw_reader = DDS_Subscriber_create_datareader(
            subscriber,
            draw_topic as *mut DDS_TopicDescription,
            &datareader_qos,
            &mut draw_listener,
            DDS_STATUS_MASK_ALL,
        ) as *mut DDS_DataReader;
        
        // 创建擦除消息的listener和reader
        let mut erase_listener: DDS_DataReaderListener = mem::zeroed();
        erase_listener.on_data_available = Some(on_erase_data_available);

        let erase_reader = DDS_Subscriber_create_datareader(
            subscriber,
            erase_topic as *mut DDS_TopicDescription,
            &datareader_qos,
            &mut erase_listener,
            DDS_STATUS_MASK_ALL,
        ) as *mut DDS_DataReader;
        
        // 创建图片删除消息的listener和reader
        let mut image_delete_listener: DDS_DataReaderListener = mem::zeroed();
        image_delete_listener.on_data_available = Some(on_image_delete_data_available);

        let image_delete_reader = DDS_Subscriber_create_datareader(
            subscriber,
            image_delete_topic as *mut DDS_TopicDescription,
            &datareader_qos,
            &mut image_delete_listener,
            DDS_STATUS_MASK_ALL,
        ) as *mut DDS_DataReader;
        
        // 创建聊天消息的listener和reader
        let mut chat_listener: DDS_DataReaderListener = mem::zeroed();
        chat_listener.on_data_available = Some(on_chat_data_available);

        let chat_reader = DDS_Subscriber_create_datareader(
            subscriber,
            chat_topic as *mut DDS_TopicDescription,
            &datareader_qos,
            &mut chat_listener,
            DDS_STATUS_MASK_ALL,
        ) as *mut DDS_DataReader;


        let options = eframe::NativeOptions::default();
        eframe::run_native(
            "Shared Mouse Canvas",
            options,
            Box::new(move |cc| Box::new(MouseApp::new(received.clone(), received_images.clone(), received_strokes.clone(), received_erases.clone(), received_image_deletes.clone(), received_chat_messages.clone(), writer.clone(), image_writer.clone(), draw_writer.clone(), erase_writer.clone(), image_delete_writer.clone(), chat_writer.clone(), cc))),
        );
    }


    
}

struct MouseApp {
    received: Arc<Mutex<HashMap<String, MouseState>>>,
    received_images: Arc<Mutex<HashMap<String, ImageData>>>,
    received_strokes: Arc<Mutex<Vec<DrawStroke>>>,
    received_erases: Arc<Mutex<Vec<EraseOperation>>>,
    received_image_deletes: Arc<Mutex<Vec<ImageDeleteOperation>>>,
    received_chat_messages: Arc<Mutex<Vec<ChatMessage>>>,
    writer: Arc<Mutex<*mut DDS_DataWriter>>,
    image_writer: Arc<Mutex<*mut DDS_DataWriter>>,
    draw_writer: Arc<Mutex<*mut DDS_DataWriter>>,
    erase_writer: Arc<Mutex<*mut DDS_DataWriter>>,
    image_delete_writer: Arc<Mutex<*mut DDS_DataWriter>>,
    chat_writer: Arc<Mutex<*mut DDS_DataWriter>>,
    my_color: egui::Color32,
    texture_cache: HashMap<String, egui::TextureHandle>, // 纹理缓存
    is_draw_mode: bool, // 是否处于画笔模式
    is_erase_mode: bool, // 是否处于擦除模式
    is_drawing: bool, // 是否正在绘制
    last_draw_pos: Option<egui::Pos2>, // 上一个绘制位置
    local_strokes: Vec<DrawStroke>, // 本地笔迹存储
    chat_input: String, // 聊天输入框内容
}

impl MouseApp {
    fn new(
        received: Arc<Mutex<HashMap<String, MouseState>>>,
        received_images: Arc<Mutex<HashMap<String, ImageData>>>,
        received_strokes: Arc<Mutex<Vec<DrawStroke>>>,
        received_erases: Arc<Mutex<Vec<EraseOperation>>>,
        received_image_deletes: Arc<Mutex<Vec<ImageDeleteOperation>>>,
        received_chat_messages: Arc<Mutex<Vec<ChatMessage>>>,
        writer: Arc<Mutex<*mut DDS_DataWriter>>,
        image_writer: Arc<Mutex<*mut DDS_DataWriter>>,
        draw_writer: Arc<Mutex<*mut DDS_DataWriter>>,
        erase_writer: Arc<Mutex<*mut DDS_DataWriter>>,
        image_delete_writer: Arc<Mutex<*mut DDS_DataWriter>>,
        chat_writer: Arc<Mutex<*mut DDS_DataWriter>>,
        cc: &eframe::CreationContext,
    ) -> Self {
        // 配置中文字体支持
        let mut fonts = egui::FontDefinitions::default();
        
        // 添加中文字体（使用系统默认字体）
        fonts.font_data.insert(
            "chinese".to_owned(),
            egui::FontData::from_static(include_bytes!("C:\\Windows\\Fonts\\msyh.ttc")),
        );
        
        // 将中文字体添加到字体族中
        fonts.families.get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "chinese".to_owned());
        
        fonts.families.get_mut(&egui::FontFamily::Monospace)
            .unwrap()
            .push("chinese".to_owned());
        
        // 应用字体配置
        cc.egui_ctx.set_fonts(fonts);
        
        Self {
            received,
            received_images,
            received_strokes,
            received_erases,
            received_image_deletes,
            received_chat_messages,
            writer,
            image_writer,
            draw_writer,
            erase_writer,
            image_delete_writer,
            chat_writer,
            my_color: egui::Color32::from_rgba_unmultiplied(255, 0, 0, 255), // 默认红色
            texture_cache: HashMap::new(), // 初始化纹理缓存
            is_draw_mode: false, // 默认鼠标模式
            is_erase_mode: false, // 默认非擦除模式
            is_drawing: false, // 默认未绘制
            last_draw_pos: None, // 初始化绘制位置
            local_strokes: Vec::new(), // 初始化本地笔迹
            chat_input: String::new(), // 初始化聊天输入框
        }
    }
    
    fn send_chat_message(&mut self) {
        if !self.chat_input.trim().is_empty() {
            let username = whoami::username();
            let timestamp = chrono::Local::now().format("%H:%M:%S").to_string();
            
            let chat_message = json!({
                "type": "Chat",
                "username": username,
                "message": self.chat_input.trim(),
                "timestamp": timestamp
            });
            
            let json_str = chat_message.to_string();
            let buffer = json_str.as_bytes();
            
            unsafe {
                let writer = self.chat_writer.lock().unwrap();
                let mut data: DDS_OctetSeq = mem::zeroed();
                DDS_OctetSeq_initialize(&mut data);
                DDS_OctetSeq_loan_contiguous(&mut data, buffer.as_ptr() as *mut u8, buffer.len() as u32, buffer.len() as u32);
                
                let mut bytes_data = DDS_Bytes { value: data };
                let instance_handle = DDS_BytesDataWriter_register_instance(*writer as *mut DDS_BytesDataWriter, &mut bytes_data);
                DDS_BytesDataWriter_write(*writer as *mut DDS_BytesDataWriter, &mut bytes_data, &instance_handle);
            }
            
            // 清空输入框
            self.chat_input.clear();
        }
    }
}

impl eframe::App for MouseApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 使用左右分栏布局
        egui::SidePanel::right("chat_panel")
            .resizable(true)
            .default_width(300.0)
            .min_width(250.0)
            .show(ctx, |ui| {
                // 聊天区域
                ui.heading("聊天室");
                
                // 聊天消息显示区域
                egui::ScrollArea::vertical()
                    .max_height(400.0)
                    .show(ui, |ui| {
                        if let Ok(messages) = self.received_chat_messages.lock() {
                            for message in messages.iter() {
                                ui.horizontal(|ui| {
                                    ui.label(format!("[{}] {}: {}", message.timestamp, message.username, message.message));
                                });
                            }
                        }
                    });
                
                // 聊天输入框和发送按钮
                ui.separator();
                ui.horizontal(|ui| {
                    let response = ui.text_edit_singleline(&mut self.chat_input);
                    
                    // 检测回车键发送消息
                    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        self.send_chat_message();
                    }
                    
                    if ui.button("发送").clicked() {
                        self.send_chat_message();
                    }
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| 
        {
            //颜色选择器
            ui.horizontal(|ui| {
                ui.color_edit_button_srgba(&mut self.my_color);
            });
            
            // 模式切换按钮
            ui.horizontal(|ui| {
                if ui.button("鼠标模式").clicked() {
                    self.is_draw_mode = false;
                    self.is_erase_mode = false;
                    self.is_drawing = false;
                    self.last_draw_pos = None;
                }
                if ui.button("画笔模式").clicked() {
                    self.is_draw_mode = true;
                    self.is_erase_mode = false;
                    self.is_drawing = false;
                    self.last_draw_pos = None;
                }
                if ui.button("擦除模式").clicked() {
                    self.is_draw_mode = false;
                    self.is_erase_mode = true;
                    self.is_drawing = false;
                    self.last_draw_pos = None;
                }
                
                // 显示当前模式
                let current_mode = if self.is_erase_mode {
                    "当前模式: 擦除"
                } else if self.is_draw_mode {
                    "当前模式: 画笔"
                } else {
                    "当前模式: 鼠标"
                };
                ui.label(current_mode);
            });
            
            // 图片选择和发送按钮
            ui.horizontal(|ui| {
                if ui.button("选择并发送图片").clicked() {
                    // 使用文件对话框选择图片文件
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("PNG图片", &["png"])
                        .pick_file() {
                        
                        let path_str = path.to_string_lossy();
                        match load_image_as_base64(&path_str) {
                            Ok((base64_data, width, height)) => {
                                let username = whoami::username();
                                let image_message = json!({
                                    "type": "Image",
                                    "username": username,
                                    "image_data": base64_data,
                                    "width": width,
                                    "height": height
                                });
                                
                                let json_str = image_message.to_string();
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
                                    
                                    let writer = *self.image_writer.lock().unwrap();
                                    let handle = DDS_BytesDataWriter_register_instance(writer as *mut DDS_BytesDataWriter, &mut data);
                                    DDS_BytesDataWriter_write(writer as *mut DDS_BytesDataWriter, &mut data, &handle);
                                }
                            }
                            Err(e) => {
                                eprintln!("加载图片失败: {}", e);
                            }
                        }
                    }
                }
            });

            let painter = ui.painter();

            // 显示接收到的图片
            let image_data = self.received_images.lock().unwrap();
            let mut y_offset = 50.0; // 图片显示的起始Y位置
            for (username, img_data) in image_data.iter() {
                // 检查纹理缓存，如果不存在则创建
                let texture_key = format!("image_{}", username);
                if !self.texture_cache.contains_key(&texture_key) {
                    if let Ok(image) = image::load_from_memory(&img_data.image_data) {
                        let rgba_image = image.to_rgba8();
                        let size = [image.width() as usize, image.height() as usize];
                        let pixels = rgba_image.as_flat_samples();
                        
                        let color_image = egui::ColorImage::from_rgba_unmultiplied(
                            size,
                            pixels.as_slice(),
                        );
                        
                        let texture = ctx.load_texture(
                            texture_key.clone(),
                            color_image,
                            egui::TextureOptions::default()
                        );
                        
                        self.texture_cache.insert(texture_key.clone(), texture);
                    }
                }
                
                // 使用缓存的纹理
                if let Some(texture) = self.texture_cache.get(&texture_key) {
                    
                    // 计算显示尺寸（限制最大宽度为200像素）
                    let max_width = 200.0;
                    let scale = if img_data.width as f32 > max_width {
                        max_width / img_data.width as f32
                    } else {
                        1.0
                    };
                    let display_size = egui::vec2(
                        img_data.width as f32 * scale,
                        img_data.height as f32 * scale
                    );
                    
                    // 显示图片
                    let image_rect = egui::Rect::from_min_size(
                        egui::pos2(10.0, y_offset+20.0),
                        display_size
                    );
                    
                    painter.image(
                        texture.id(),
                        image_rect,
                        egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                        egui::Color32::WHITE
                    );
                    
                    // 显示用户名
                    painter.text(
                        egui::pos2(120.0, y_offset ),
                        egui::Align2::LEFT_TOP,
                        format!("图片来自: {}", username),
                        egui::FontId::proportional(12.0),
                        egui::Color32::YELLOW,
                    );
                    
                    // 在图片右上角添加删除按钮
                    let delete_button_size = 20.0;
                    let delete_button_pos = egui::pos2(
                        image_rect.max.x - delete_button_size - 5.0,
                        image_rect.min.y + 5.0
                    );
                    let delete_button_rect = egui::Rect::from_min_size(
                        delete_button_pos,
                        egui::vec2(delete_button_size, delete_button_size)
                    );
                    
                    // 绘制删除按钮背景（红色圆形）
                    painter.circle_filled(
                        delete_button_rect.center(),
                        delete_button_size / 2.0,
                        egui::Color32::from_rgb(200, 50, 50)
                    );
                    
                    // 绘制删除按钮的X符号
                    let x_size = delete_button_size * 0.3;
                    let center = delete_button_rect.center();
                    painter.line_segment(
                        [egui::pos2(center.x - x_size, center.y - x_size), 
                         egui::pos2(center.x + x_size, center.y + x_size)],
                        egui::Stroke::new(2.0, egui::Color32::WHITE)
                    );
                    painter.line_segment(
                        [egui::pos2(center.x + x_size, center.y - x_size), 
                         egui::pos2(center.x - x_size, center.y + x_size)],
                        egui::Stroke::new(2.0, egui::Color32::WHITE)
                    );
                    
                    // 检测删除按钮点击
                    if ui.input(|i| i.pointer.any_click()) {
                        if let Some(pointer_pos) = ui.input(|i| i.pointer.interact_pos()) {
                            if delete_button_rect.contains(pointer_pos) {
                                // 触发删除操作
                                println!("删除图片: {}", username);
                                
                                // 创建图片删除操作数据
                                let delete_op = ImageDeleteOperation {
                                    username: whoami::username(),
                                    image_id: username.clone(),
                                };
                                
                                // 发送图片删除消息到DDS
                                if let Ok(writer) = self.image_delete_writer.lock() {
                                    let json_data = json!({
                                        "type": "ImageDelete",
                                        "username": delete_op.username,
                                        "image_id": delete_op.image_id
                                    });
                                    
                                    let json_str = json_data.to_string();
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
                                        
                                        let writer = *writer;
                                        let handle = DDS_BytesDataWriter_register_instance(writer as *mut DDS_BytesDataWriter, &mut data);
                                        DDS_BytesDataWriter_write(writer as *mut DDS_BytesDataWriter, &mut data, &handle);
                                    }
                                }
                                
                                // 立即从本地删除图片（避免等待网络同步）
                                // 注意：这里需要在循环外处理，因为我们正在遍历HashMap
                                // 我们将在循环后处理删除
                            }
                        }
                    }
                    
                    y_offset += display_size.y + 30.0; // 为下一张图片留出空间
                }
            }
            
            // 渲染本地画笔笔迹
            for stroke in &self.local_strokes {
                let start_pos = egui::pos2(stroke.start_x, stroke.start_y);
                let end_pos = egui::pos2(stroke.end_x, stroke.end_y);
                let color = egui::Color32::from_rgba_unmultiplied(
                    stroke.color[0], stroke.color[1], stroke.color[2], stroke.color[3],
                );
                
                painter.line_segment([start_pos, end_pos], egui::Stroke::new(stroke.stroke_width, color));
            }
            
            // 渲染远程接收到的画笔笔迹
            let received_strokes = self.received_strokes.lock().unwrap();
            for stroke in received_strokes.iter() {
                let start_pos = egui::pos2(stroke.start_x, stroke.start_y);
                let end_pos = egui::pos2(stroke.end_x, stroke.end_y);
                let color = egui::Color32::from_rgba_unmultiplied(
                    stroke.color[0], stroke.color[1], stroke.color[2], stroke.color[3],
                );
                
                painter.line_segment([start_pos, end_pos], egui::Stroke::new(stroke.stroke_width, color));
            }
            
            // 渲染鼠标圆点和坐标（显示在图片和画笔笔迹之上）
            let data = self.received.lock().unwrap();
            for mouse in data.values() {
                let pos = egui::pos2(mouse.x, mouse.y);
                let color = egui::Color32::from_rgba_unmultiplied(
                    mouse.color[0], mouse.color[1], mouse.color[2], mouse.color[3],
                );

                // 1. 画一个小圆点当作"鼠标"
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
            // 无论什么模式都发送鼠标位置信息（用于显示圆点）
            let mouse = json!({
                "username": username,
                "color": color_arr,
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

                let writer = *self.writer.lock().unwrap();
                let handle = DDS_BytesDataWriter_register_instance(writer as *mut DDS_BytesDataWriter, &mut data);
                DDS_BytesDataWriter_write(writer as *mut DDS_BytesDataWriter, &mut data, &handle);
            }
            
            // 画笔模式下的绘制逻辑
            if self.is_draw_mode {
                let is_mouse_down = ctx.input(|i| i.pointer.primary_down());
                
                if is_mouse_down {
                    if !self.is_drawing {
                        // 开始绘制
                        self.is_drawing = true;
                        self.last_draw_pos = Some(pos);
                    } else if let Some(last_pos) = self.last_draw_pos {
                        // 继续绘制，创建笔迹
                        let stroke = DrawStroke {
                            username: username.clone(),
                            color: self.my_color,
                            start_x: last_pos.x,
                            start_y: last_pos.y,
                            end_x: pos.x,
                            end_y: pos.y,
                            stroke_width: 2.0,
                        };
                        
                        // 添加到本地笔迹
                        self.local_strokes.push(stroke.clone());
                        
                        // 发送笔迹数据到DDS
                        let draw_message = json!({
                            "type": "Draw",
                            "username": stroke.username,
                            "color": [stroke.color.r(), stroke.color.g(), stroke.color.b(), stroke.color.a()],
                            "start_x": stroke.start_x,
                            "start_y": stroke.start_y,
                            "end_x": stroke.end_x,
                            "end_y": stroke.end_y,
                            "stroke_width": stroke.stroke_width
                        });
                        
                        let json_str = draw_message.to_string();
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
                            
                            let writer = *self.draw_writer.lock().unwrap();
                            let handle = DDS_BytesDataWriter_register_instance(writer as *mut DDS_BytesDataWriter, &mut data);
                            DDS_BytesDataWriter_write(writer as *mut DDS_BytesDataWriter, &mut data, &handle);
                        }
                        
                        self.last_draw_pos = Some(pos);
                    }
                } else {
                    // 鼠标松开，停止绘制
                    self.is_drawing = false;
                    self.last_draw_pos = None;
                }
            }
            
            // 擦除模式下的擦除逻辑
            if self.is_erase_mode {
                let is_mouse_down = ctx.input(|i| i.pointer.primary_down());
                
                if is_mouse_down {
                    let erase_radius = 20.0; // 擦除半径
                    
                    // 检查本地笔迹并删除与擦除区域相交的笔迹
                    self.local_strokes.retain(|stroke| {
                        !line_intersects_circle(
                            stroke.start_x, stroke.start_y,
                            stroke.end_x, stroke.end_y,
                            pos.x, pos.y, erase_radius
                        )
                    });
                    
                    // 检查远程笔迹并删除与擦除区域相交的笔迹
                    {
                        let mut received_strokes = self.received_strokes.lock().unwrap();
                        received_strokes.retain(|stroke| {
                            !line_intersects_circle(
                                stroke.start_x, stroke.start_y,
                                stroke.end_x, stroke.end_y,
                                pos.x, pos.y, erase_radius
                            )
                        });
                    }
                    
                    // 发送擦除操作到DDS
                    let erase_operation = EraseOperation {
                        username: username.clone(),
                        x: pos.x,
                        y: pos.y,
                        radius: erase_radius,
                    };
                    
                    let erase_message = json!({
                        "type": "Erase",
                        "username": erase_operation.username,
                        "x": erase_operation.x,
                        "y": erase_operation.y,
                        "radius": erase_operation.radius
                    });
                    
                    let json_str = erase_message.to_string();
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
                        
                        let writer = *self.erase_writer.lock().unwrap();
                        let handle = DDS_BytesDataWriter_register_instance(writer as *mut DDS_BytesDataWriter, &mut data);
                        DDS_BytesDataWriter_write(writer as *mut DDS_BytesDataWriter, &mut data, &handle);
                    }
                }
            }
        }

        // 处理接收到的擦除操作
        let received_erases = self.received_erases.lock().unwrap().clone();
        for erase_op in received_erases {
            // 删除与擦除区域相交的本地笔迹
            self.local_strokes.retain(|stroke| {
                !line_intersects_circle(
                    stroke.start_x, stroke.start_y,
                    stroke.end_x, stroke.end_y,
                    erase_op.x, erase_op.y, erase_op.radius
                )
            });
            
            // 删除与擦除区域相交的远程笔迹
            let mut remote_strokes = self.received_strokes.lock().unwrap();
            remote_strokes.retain(|stroke| {
                !line_intersects_circle(
                    stroke.start_x, stroke.start_y,
                    stroke.end_x, stroke.end_y,
                    erase_op.x, erase_op.y, erase_op.radius
                )
            });
        }
        
        // 清空已处理的擦除操作
        self.received_erases.lock().unwrap().clear();
        
        // 处理接收到的图片删除操作
        let received_image_deletes = self.received_image_deletes.lock().unwrap().clone();
        for delete_op in received_image_deletes {
            // 从本地图片缓存中删除图片
            {
                let mut received_images = self.received_images.lock().unwrap();
                received_images.remove(&delete_op.image_id);
            }
            
            // 从纹理缓存中删除图片
            self.texture_cache.remove(&delete_op.image_id);
            
            println!("已删除图片: {} (来自用户: {})", delete_op.image_id, delete_op.username);
        }
        
        // 清空已处理的图片删除操作
        self.received_image_deletes.lock().unwrap().clear();

        ctx.request_repaint();
    
    }
}