use std::pin::Pin;
use crate::bindings::*;
use crate::core::bytes::bytes::Bytes;
use crate::dds_simple_data_reader_listener;
use crate::dioxus_structs::*;
use crate::dioxus_structs::{CLEAR_OWN_STROKES_REQUEST, LOCAL_STROKES};
use crate::utils::color_from_json;
use base64::{Engine as _, engine::general_purpose};
use egui::Color32;
use serde_json::Value;
use std::borrow::Cow;
use crate::core::sample::sample_info::SampleInfo;
// for .decode()

/// 安全地从 JSON 中解析 Color32，支持 [r,g,b] 或 [r,g,b,a]
fn parse_color32_from_json(value: &Value) -> Color32 {
    // 空数组常量，具有 'static 生命周期，不会被临时丢弃
    const EMPTY: [Value; 0] = [];
    let arr: &[Value] = value.as_array().map(|v| v.as_slice()).unwrap_or(&EMPTY);

    // 读取第 i 个通道为 u8；缺失时取 0
    let get_u8 = |i: usize| -> u8 { arr.get(i).and_then(|v| v.as_u64()).unwrap_or(0) as u8 };

    match arr.len() {
        4 => Color32::from_rgba_unmultiplied(get_u8(0), get_u8(1), get_u8(2), get_u8(3)),
        _ => Color32::from_rgb(get_u8(0), get_u8(1), get_u8(2)),
    }
}

fn handle_one_draw_sample(sample: &Bytes, _info: &SampleInfo) {
    // Bytes -> Vec<u8>
    let len = unsafe { DDS_OctetSeq_get_length(&(*sample.as_ref()).value) };
    let mut buf = Vec::with_capacity(len as usize);
    for j in 0..len {
        let bptr = unsafe { DDS_OctetSeq_get_reference(&(*sample.as_ref()).value, j) };
        if !bptr.is_null() {
            unsafe {
                buf.push(*bptr);
            }
        }
    }

    // 解析 JSON 并入队
    if let Ok(s) = String::from_utf8(buf) {
        if let Ok(draw_msg) = serde_json::from_str::<Value>(&s) {
            if draw_msg.get("type").and_then(|v| v.as_str()) == Some("Draw") {
                let color = parse_color32_from_json(&draw_msg["color"]);
                let stroke = DrawStroke {
                    username: draw_msg["username"]
                        .as_str()
                        .unwrap_or("unknown")
                        .to_string(),
                    color,
                    start_x: draw_msg["start_x"].as_f64().unwrap_or(0.0) as f32,
                    start_y: draw_msg["start_y"].as_f64().unwrap_or(0.0) as f32,
                    end_x: draw_msg["end_x"].as_f64().unwrap_or(0.0) as f32,
                    end_y: draw_msg["end_y"].as_f64().unwrap_or(0.0) as f32,
                    stroke_width: draw_msg["stroke_width"].as_f64().unwrap_or(2.0) as f32,
                    timestamp: draw_msg["timestamp"].as_u64().unwrap_or(0),
                };

                unsafe {
                    if let Some(ref received_strokes) = RECEIVED_STROKES {
                        received_strokes.lock().unwrap().push(stroke);
                    }
                }
            }
        }
    }
}

// ---------- 1) 处理“一条样本”的纯 Rust 安全函数 ----------
fn handle_one_user_color_sample(sample: &Bytes, _info: &SampleInfo) {
    // Bytes -> Vec<u8>
    let len = unsafe { DDS_OctetSeq_get_length(&(*sample.as_ref()).value) };
    let mut buf = Vec::with_capacity(len as usize);
    for j in 0..len {
        let bptr = unsafe { DDS_OctetSeq_get_reference(&(*sample.as_ref()).value, j) };
        if !bptr.is_null() {
            unsafe {
                buf.push(*bptr);
            }
        }
    }

    // 解析 JSON -> 仅处理 type == "UserColor"
    if let Ok(s) = String::from_utf8(buf) {
        if let Ok(color_msg) = serde_json::from_str::<Value>(&s) {
            if color_msg.get("type").and_then(|v| v.as_str()) == Some("UserColor") {
                let username = color_msg
                    .get("username")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown")
                    .to_string();

                // 解析颜色对象 {r,g,b,a}（缺省则用白色）
                let color = color_msg
                    .get("color")
                    .and_then(|v| v.as_object())
                    .and_then(|obj| {
                        let r = obj.get("r").and_then(|v| v.as_u64()).map(|x| x as u8)?;
                        let g = obj.get("g").and_then(|v| v.as_u64()).map(|x| x as u8)?;
                        let b = obj.get("b").and_then(|v| v.as_u64()).map(|x| x as u8)?;
                        let a = obj.get("a").and_then(|v| v.as_u64()).map(|x| x as u8)?;
                        Some(Color32::from_rgba_premultiplied(r, g, b, a))
                    })
                    .unwrap_or(Color32::WHITE);

                let timestamp = color_msg
                    .get("timestamp")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0);

                unsafe {
                    if let Some(ref received_user_colors) = RECEIVED_USER_COLORS {
                        let mut data = received_user_colors.lock().unwrap();
                        data.insert(
                            username.clone(),
                            UserColor {
                                username: username.clone(),
                                color,
                                timestamp,
                            },
                        );
                    }
                }

                // 可选：日志
                // println!("接收到用户颜色消息: username={}, color={:?}", username, color);
            }
        }
    }
}

fn handle_one_danmaku_sample(sample: &Bytes, _info: &SampleInfo) {
    // Bytes -> Vec<u8>
    let len = unsafe { DDS_OctetSeq_get_length(&(*sample.as_ref()).value) };
    let mut buf = Vec::with_capacity(len as usize);
    for j in 0..len {
        let bptr = unsafe { DDS_OctetSeq_get_reference(&(*sample.as_ref()).value, j) };
        if !bptr.is_null() {
            unsafe {
                buf.push(*bptr);
            }
        }
    }

    // 解析 JSON -> 业务对象 -> 入队
    if let Ok(json_str) = String::from_utf8(buf) {
        if let Ok(json_value) = serde_json::from_str::<Value>(&json_str) {
            if let Some(danmaku_message) = parse_danmaku_message(&json_value) {
                unsafe {
                    if let Some(ref received_danmaku_messages) = RECEIVED_DANMAKU_MESSAGES {
                        received_danmaku_messages
                            .lock()
                            .unwrap()
                            .push(danmaku_message);
                    }
                }
            }
        }
    }
}

// 解析弹幕消息的辅助函数
fn parse_danmaku_message(json_value: &serde_json::Value) -> Option<DanmakuMessage> {
    let username = json_value["username"]
        .as_str()
        .unwrap_or("unknown")
        .to_string();
    let message = json_value["message"].as_str().unwrap_or("").to_string();
    let start_time = json_value["start_time"].as_f64().unwrap_or(0.0);
    let danmaku_id = format!(
        "{}-{}-{}",
        username,
        message.len(),
        (start_time * 1000.0) as u64
    );

    Some(DanmakuMessage {
        username,
        message,
        x: json_value["x"].as_f64().unwrap_or(0.0) as f32,
        y: json_value["y"].as_f64().unwrap_or(0.0) as f32,
        speed: json_value["speed"].as_f64().unwrap_or(100.0) as f32,
        start_time,
        color: Color32::from_rgb(
            json_value["color"][0].as_u64().unwrap_or(255) as u8,
            json_value["color"][1].as_u64().unwrap_or(255) as u8,
            json_value["color"][2].as_u64().unwrap_or(255) as u8,
        ),
        id: danmaku_id,
    })
}

fn handle_one_image_sample(sample: &Bytes, _info: &SampleInfo) {
    // Bytes -> Vec<u8>
    let len = unsafe { DDS_OctetSeq_get_length(&(*sample.as_ref()).value) };
    let mut buf = Vec::with_capacity(len as usize);
    for j in 0..len {
        let bptr = unsafe { DDS_OctetSeq_get_reference(&(*sample.as_ref()).value, j) };
        if !bptr.is_null() {
            unsafe {
                buf.push(*bptr);
            }
        }
    }

    // JSON 解析
    let Ok(s) = String::from_utf8(buf) else {
        return;
    };
    let Ok(image_msg) = serde_json::from_str::<Value>(&s) else {
        return;
    };

    // 读取字段（容错）
    let username = image_msg
        .get("username")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown");
    let (width, height) = (
        image_msg.get("width").and_then(|v| v.as_u64()).unwrap_or(0) as u32,
        image_msg
            .get("height")
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as u32,
    );

    // 允许 image_data 为裸 base64 或 data URL（如 data:image/png;base64,XXXX）
    let image_data_field = match image_msg.get("image_data") {
        Some(Value::String(s)) => Some(Cow::from(s.as_str())),
        _ => None,
    };
    let Some(img_str) = image_data_field else {
        return;
    };

    // 如果是 data URL，切掉逗号前缀
    let base64_part = img_str
        .rsplit_once(',')
        .map(|(_, b64)| b64)
        .unwrap_or(&*img_str);

    // 解码
    let Ok(image_bytes) = general_purpose::STANDARD.decode(base64_part) else {
        return;
    };

    // 写入共享缓存
    unsafe {
        if let Some(ref received_images) = RECEIVED_IMAGES {
            let mut map = received_images.lock().unwrap();
            map.insert(
                username.to_string(),
                ImageData {
                    username: username.to_string(),
                    image_data: image_bytes,
                    width,
                    height,
                },
            );
        }
    }
}

fn handle_one_erase_sample(sample: &Bytes, _info: &SampleInfo) {
    // Bytes -> Vec<u8>
    let len = unsafe { DDS_OctetSeq_get_length(&(*sample.as_ref()).value) };
    let mut buf = Vec::with_capacity(len as usize);
    for j in 0..len {
        let bptr = unsafe { DDS_OctetSeq_get_reference(&(*sample.as_ref()).value, j) };
        if !bptr.is_null() {
            unsafe {
                buf.push(*bptr);
            }
        }
    }

    // 解析 JSON -> 入队
    let Ok(s) = String::from_utf8(buf) else {
        return;
    };
    let Ok(erase_msg) = serde_json::from_str::<Value>(&s) else {
        return;
    };

    // 检查是否为清空所有笔迹的消息
    if let Some(msg_type) = erase_msg.get("type").and_then(|v| v.as_str()) {
        if msg_type == "clear_all" {
            // 清空所有接收到的笔迹数据
            unsafe {
                if let Some(ref received_strokes) = RECEIVED_STROKES {
                    let mut data = received_strokes.lock().unwrap();
                    data.clear();
                    println!("[DDS] 收到清空所有笔迹消息，已清空接收到的笔迹数据");
                }
                // 设置清空自己笔迹的请求标志
                if let Some(ref clear_request) = CLEAR_OWN_STROKES_REQUEST {
                    if let Ok(mut request) = clear_request.lock() {
                        *request = true;
                        println!("[DDS] 收到清空所有笔迹消息，已设置清空自己笔迹的请求");
                    }
                }
            }
            return;
        } else if msg_type == "clear_user" {
            // 清空特定用户的笔迹数据
            let target_username = erase_msg
                .get("username")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown")
                .to_string();
            
            unsafe {
                if let Some(ref received_strokes) = RECEIVED_STROKES {
                    let mut data = received_strokes.lock().unwrap();
                    data.retain(|stroke| stroke.username != target_username);
                    println!("[DDS] 收到清空用户 {} 笔迹消息，已清空该用户的笔迹数据", target_username);
                }
            }
            return;
        }
    }

    let username = erase_msg
        .get("username")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown")
        .to_string();
    let x = erase_msg.get("x").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
    let y = erase_msg.get("y").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
    let radius = erase_msg
        .get("radius")
        .and_then(|v| v.as_f64())
        .unwrap_or(20.0) as f32;
    let timestamp = erase_msg
        .get("timestamp")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);

    unsafe {
        if let Some(ref received_erases) = RECEIVED_ERASES {
            let mut data = received_erases.lock().unwrap();
            data.push(EraseOperation {
                username,
                x,
                y,
                radius,
                timestamp,
            });
        }
    }
}

fn handle_one_image_delete_sample(sample: &Bytes, _info: &SampleInfo) {
    // Bytes -> Vec<u8>
    let len = unsafe { DDS_OctetSeq_get_length(&(*sample.as_ref()).value) };
    let mut buf = Vec::with_capacity(len as usize);
    for j in 0..len {
        let bptr = unsafe { DDS_OctetSeq_get_reference(&(*sample.as_ref()).value, j) };
        if !bptr.is_null() {
            unsafe {
                buf.push(*bptr);
            }
        }
    }

    // 解析 JSON -> 入队
    let Ok(s) = String::from_utf8(buf) else {
        return;
    };
    let Ok(delete_msg) = serde_json::from_str::<Value>(&s) else {
        return;
    };

    let username = delete_msg
        .get("username")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown")
        .to_string();
    let image_id = delete_msg
        .get("image_id")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    unsafe {
        if let Some(ref received_image_deletes) = RECEIVED_IMAGE_DELETES {
            let mut data = received_image_deletes.lock().unwrap();
            data.push(ImageDeleteOperation { username, image_id });
        }
    }
}

// 聊天消息回调函数
fn handle_one_chat_sample(sample: &Bytes, _info: &SampleInfo) {
    // Bytes -> Vec<u8>
    let len = unsafe { DDS_OctetSeq_get_length(&(*sample.as_ref()).value) };
    let mut buf = Vec::with_capacity(len as usize);
    for j in 0..len {
        let bptr = unsafe { DDS_OctetSeq_get_reference(&(*sample.as_ref()).value, j) };
        if !bptr.is_null() {
            unsafe {
                buf.push(*bptr);
            }
        }
    }

    // JSON 解析
    let Ok(s) = String::from_utf8(buf) else {
        return;
    };
    let Ok(chat_msg) = serde_json::from_str::<Value>(&s) else {
        return;
    };

    // 基本字段
    let username = chat_msg
        .get("username")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown")
        .to_string();
    let message = chat_msg
        .get("message")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let timestamp = chat_msg
        .get("timestamp")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    // 颜色：对象 {r,g,b,a}，缺省则白色
    let color = chat_msg
        .get("color")
        .and_then(|v| v.as_object())
        .and_then(|obj| {
            let r = obj.get("r").and_then(|v| v.as_u64()).map(|x| x as u8)?;
            let g = obj.get("g").and_then(|v| v.as_u64()).map(|x| x as u8)?;
            let b = obj.get("b").and_then(|v| v.as_u64()).map(|x| x as u8)?;
            let a = obj.get("a").and_then(|v| v.as_u64()).map(|x| x as u8)?;
            Some(Color32::from_rgba_premultiplied(r, g, b, a))
        })
        .unwrap_or(Color32::WHITE);

    // 1) 写入聊天消息队列
    unsafe {
        if let Some(ref received_chat_messages) = RECEIVED_CHAT_MESSAGES {
            let mut data = received_chat_messages.lock().unwrap();
            data.push(ChatMessage {
                username: username.clone(),
                message: message.clone(),
                timestamp: timestamp.clone(),
                color,
            });
        }
    }

    // 2) 弹幕（根据开关）
    let danmaku_enabled = match unsafe { (&raw const DANMAKU_ENABLED).read() } {
        Some(flag) => *flag.lock().unwrap(),
        None => true, // 默认启用
    };

    if danmaku_enabled {
        unsafe {
            if let Some(ref received_danmaku_messages) = RECEIVED_DANMAKU_MESSAGES {
                let mut list = received_danmaku_messages.lock().unwrap();
                let current_time = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs_f64();
                let y_position = (list.len() % 10) as f32 * 50.0 + 50.0; // 分层显示
                let danmaku_id = format!(
                    "{}-{}-{}",
                    username,
                    message.len(),
                    (current_time * 1000.0) as u64
                );

                list.push(DanmakuMessage {
                    username,
                    message,
                    start_time: current_time,
                    x: 1720.0,     // 从屏幕最右侧开始
                    y: y_position, // 分层显示
                    color,
                    speed: 100.0,
                    id: danmaku_id,
                });
            }
        }
    }
}

fn handle_one_mouse_sample(sample: &Bytes, _info: &SampleInfo) {
    let len = unsafe { DDS_OctetSeq_get_length(&(*sample.as_ref()).value) };
    let mut buf = Vec::with_capacity(len as usize);
    for j in 0..len {
        let bptr = unsafe { DDS_OctetSeq_get_reference(&(*sample.as_ref()).value, j) };
        if !bptr.is_null() {
            unsafe {
                buf.push(*bptr);
            }
        }
    }

    if let Ok(s) = String::from_utf8(buf) {
        if let Ok(mouse_state) = serde_json::from_str::<serde_json::Value>(&s) {
            unsafe {
                if let Some(ref received_clone) = RECEIVED {
                    let mut data = received_clone.lock().unwrap();
                    let username = mouse_state["username"]
                        .as_str()
                        .unwrap_or("unknown")
                        .to_string();
                    let x = mouse_state["x"].as_f64().unwrap_or_default() as f32;
                    let y = mouse_state["y"].as_f64().unwrap_or_default() as f32;
                    data.insert(
                        username.clone(),
                        MouseState {
                            username,
                            color: color_from_json(&mouse_state["color"]),
                            x,
                            y,
                        },
                    );
                }
            }
        }
    }
}

// 视频消息回调函数
fn handle_one_video_sample(sample: &Bytes, _info: &SampleInfo) {
    // Bytes -> Vec<u8>
    let len = unsafe { DDS_OctetSeq_get_length(&(*sample.as_ref()).value) };
    let mut buf = Vec::with_capacity(len as usize);
    for j in 0..len {
        let bptr = unsafe { DDS_OctetSeq_get_reference(&(*sample.as_ref()).value, j) };
        if !bptr.is_null() {
            unsafe {
                buf.push(*bptr);
            }
        }
    }

    // JSON 解析
    let Ok(s) = String::from_utf8(buf) else {
        return;
    };
    let Ok(video_msg) = serde_json::from_str::<Value>(&s) else {
        return;
    };

    // 基本字段
    let username = video_msg
        .get("username")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown")
        .to_string();
    let file_name = video_msg
        .get("file_name")
        .and_then(|v| v.as_str())
        .unwrap_or("video.mp4")
        .to_string();
    let file_size = video_msg
        .get("file_size")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);

    // 读取 video_data（允许 data URL；自动裁掉逗号前缀）
    let video_data_str = match video_msg.get("video_data").and_then(|v| v.as_str()) {
        Some(s) => s,
        None => {
            eprintln!("视频消息缺少 video_data 字段");
            return;
        }
    };
    let base64_part = video_data_str
        .rsplit_once(',')
        .map(|(_, b64)| b64)
        .unwrap_or(video_data_str);
    let Ok(video_data) = general_purpose::STANDARD.decode(base64_part) else {
        eprintln!("视频数据 base64 解码失败");
        return;
    };

    // 存入共享容器
    unsafe {
        if let Some(ref received_videos) = RECEIVED_VIDEOS {
            let mut map = received_videos.lock().unwrap();
            map.insert(
                username.clone(),
                VideoData {
                    username: username.clone(),
                    video_data,
                    file_name: file_name.clone(),
                    file_size,
                },
            );
            println!(
                "接收到视频: {} ({} 字节) 来自 {}",
                file_name, file_size, username
            );
        }
    }
}

// 视频删除消息回调函数
fn handle_one_video_delete_sample(sample: &Bytes, _info: &SampleInfo) {
    // Bytes -> Vec<u8>
    let len = unsafe { DDS_OctetSeq_get_length(&(*sample.as_ref()).value) };
    let mut buf = Vec::with_capacity(len as usize);
    for j in 0..len {
        let bptr = unsafe { DDS_OctetSeq_get_reference(&(*sample.as_ref()).value, j) };
        if !bptr.is_null() {
            unsafe {
                buf.push(*bptr);
            }
        }
    }

    // 解析 JSON -> 入队
    let Ok(s) = String::from_utf8(buf) else {
        return;
    };
    let Ok(delete_msg) = serde_json::from_str::<Value>(&s) else {
        return;
    };

    let username = delete_msg
        .get("username")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown")
        .to_string();
    let video_id = delete_msg
        .get("video_id")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    unsafe {
        if let Some(ref received_video_deletes) = RECEIVED_VIDEO_DELETES {
            let mut data = received_video_deletes.lock().unwrap();
            data.push(VideoDeleteOperation { username, video_id });
        }
    }
}

// 图片队列消息处理函数
fn handle_one_image_queue_sample(sample: &Bytes, _info: &SampleInfo) {
    // Bytes -> Vec<u8>
    let len = unsafe { DDS_OctetSeq_get_length(&(*sample.as_ref()).value) };
    let mut buf = Vec::with_capacity(len as usize);
    for j in 0..len {
        let bptr = unsafe { DDS_OctetSeq_get_reference(&(*sample.as_ref()).value, j) };
        if !bptr.is_null() {
            unsafe {
                buf.push(*bptr);
            }
        }
    }

    // JSON 解析
    let Ok(s) = String::from_utf8(buf) else {
        return;
    };
    let Ok(queue_msg) = serde_json::from_str::<Value>(&s) else {
        return;
    };

    // 读取字段
    let username = queue_msg
        .get("username")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown")
        .to_string();
    let current_index = queue_msg
        .get("current_index")
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as usize;
    let timestamp = queue_msg
        .get("timestamp")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);

    // 解析图片列表
    let images_array = match queue_msg.get("images").and_then(|v| v.as_array()) {
        Some(arr) => arr,
        None => return,
    };

    let mut images = Vec::new();
    for img_value in images_array {
        let id = img_value
            .get("id")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let img_username = img_value
            .get("username")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();
        let width = img_value.get("width").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
        let height = img_value
            .get("height")
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as u32;
        let img_timestamp = img_value
            .get("timestamp")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);

        // 解析图片数据
        let image_data_str = match img_value.get("image_data").and_then(|v| v.as_str()) {
            Some(s) => s,
            None => continue,
        };

        let base64_part = image_data_str
            .rsplit_once(',')
            .map(|(_, b64)| b64)
            .unwrap_or(image_data_str);
        let Ok(image_data) = general_purpose::STANDARD.decode(base64_part) else {
            continue;
        };

        images.push(ImageItem {
            id,
            username: img_username,
            image_data,
            width,
            height,
            timestamp: img_timestamp,
        });
    }

    // 写入共享缓存
    unsafe {
        if let Some(ref received_image_queues) = RECEIVED_IMAGE_QUEUES {
            let mut map = received_image_queues.lock().unwrap();
            map.insert(
                username.clone(),
                ImageQueue {
                    username,
                    images,
                    current_index,
                    timestamp,
                },
            );
        }
    }
}

// 图片队列删除消息处理函数
fn handle_one_image_queue_delete_sample(sample: &Bytes, _info: &SampleInfo) {
    // Bytes -> Vec<u8>
    let len = unsafe { DDS_OctetSeq_get_length(&(*sample.as_ref()).value) };
    let mut buf = Vec::with_capacity(len as usize);
    for j in 0..len {
        let bptr = unsafe { DDS_OctetSeq_get_reference(&(*sample.as_ref()).value, j) };
        if !bptr.is_null() {
            unsafe {
                buf.push(*bptr);
            }
        }
    }

    // 解析 JSON -> 入队
    let Ok(s) = String::from_utf8(buf) else {
        return;
    };
    let Ok(delete_msg) = serde_json::from_str::<Value>(&s) else {
        return;
    };

    let username = delete_msg
        .get("username")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown")
        .to_string();

    unsafe {
        if let Some(ref received_image_queue_deletes) = RECEIVED_IMAGE_QUEUE_DELETES {
            let mut data = received_image_queue_deletes.lock().unwrap();
            data.push(ImageQueueDeleteOperation { username });
        }
    }
}

// 显式给出 $rdr/$samp/$in 的名字（想叫什么都行）
dds_simple_data_reader_listener!(mouse, DDS_Bytes, |_r, samp, inf| {
    // 指针 -> 引用，然后交给安全函数
    let sample_ref_raw: Pin<Box<DDS_Bytes>> = Box::pin(unsafe { *samp });
    let sample_ref: &Bytes = &Bytes {
        inner: Some(sample_ref_raw),
    };
    let info_ref_raw: &mut DDS_SampleInfo = unsafe { &mut *inf };
let info_ref: &SampleInfo = &SampleInfo { raw: info_ref_raw };
    handle_one_mouse_sample(sample_ref, info_ref);
});

dds_simple_data_reader_listener!(draw, DDS_Bytes, |_r, samp, inf| {
    // 指针 -> 引用，然后交给安全函数
    let sample_ref_raw: Pin<Box<DDS_Bytes>> = Box::pin(unsafe { *samp });
    let sample_ref: &Bytes = &Bytes {
        inner: Some(sample_ref_raw),
    };
    let info_ref_raw: &mut DDS_SampleInfo = unsafe { &mut *inf };
let info_ref: &SampleInfo = &SampleInfo { raw: info_ref_raw };
    handle_one_draw_sample(sample_ref, info_ref);
});

// ---------- 2) 宏调用：生成 extern "C" 回调 ----------
dds_simple_data_reader_listener!(user_color, DDS_Bytes, |_r, samp, inf| {
    let sample_ref_raw: Pin<Box<DDS_Bytes>> = Box::pin(unsafe { *samp });
    let sample_ref: &Bytes = &Bytes {
        inner: Some(sample_ref_raw),
    };
    let info_ref_raw: &mut DDS_SampleInfo = unsafe { &mut *inf };
let info_ref: &SampleInfo = &SampleInfo { raw: info_ref_raw };
    handle_one_user_color_sample(sample_ref, info_ref);
});

dds_simple_data_reader_listener!(danmaku, DDS_Bytes, |_r, samp, inf| {
    let sample_ref_raw: Pin<Box<DDS_Bytes>> = Box::pin(unsafe { *samp });
    let sample_ref: &Bytes = &Bytes {
        inner: Some(sample_ref_raw),
    };
    let info_ref_raw: &mut DDS_SampleInfo = unsafe { &mut *inf };
let info_ref: &SampleInfo = &SampleInfo { raw: info_ref_raw };
    handle_one_danmaku_sample(sample_ref, info_ref);
});

dds_simple_data_reader_listener!(image, DDS_Bytes, |_r, samp, inf| {
    let sample_ref_raw: Pin<Box<DDS_Bytes>> = Box::pin(unsafe { *samp });
    let sample_ref: &Bytes = &Bytes {
        inner: Some(sample_ref_raw),
    };
    let info_ref_raw: &mut DDS_SampleInfo = unsafe { &mut *inf };
let info_ref: &SampleInfo = &SampleInfo { raw: info_ref_raw };
    handle_one_image_sample(sample_ref, info_ref);
});

dds_simple_data_reader_listener!(erase, DDS_Bytes, |_r, samp, inf| {
    let sample_ref_raw: Pin<Box<DDS_Bytes>> = Box::pin(unsafe { *samp });
    let sample_ref: &Bytes = &Bytes {
        inner: Some(sample_ref_raw),
    };
    let info_ref_raw: &mut DDS_SampleInfo = unsafe { &mut *inf };
let info_ref: &SampleInfo = &SampleInfo { raw: info_ref_raw };
    handle_one_erase_sample(sample_ref, info_ref);
});

dds_simple_data_reader_listener!(image_delete, DDS_Bytes, |_r, samp, inf| {
    let sample_ref_raw: Pin<Box<DDS_Bytes>> = Box::pin(unsafe { *samp });
    let sample_ref: &Bytes = &Bytes {
        inner: Some(sample_ref_raw),
    };
    let info_ref_raw: &mut DDS_SampleInfo = unsafe { &mut *inf };
let info_ref: &SampleInfo = &SampleInfo { raw: info_ref_raw };
    handle_one_image_delete_sample(sample_ref, info_ref);
});

dds_simple_data_reader_listener!(chat, DDS_Bytes, |_r, samp, inf| {
    let sample_ref_raw: Pin<Box<DDS_Bytes>> = Box::pin(unsafe { *samp });
    let sample_ref: &Bytes = &Bytes {
        inner: Some(sample_ref_raw),
    };
    let info_ref_raw: &mut DDS_SampleInfo = unsafe { &mut *inf };
let info_ref: &SampleInfo = &SampleInfo { raw: info_ref_raw };
    handle_one_chat_sample(sample_ref, info_ref);
});

dds_simple_data_reader_listener!(video, DDS_Bytes, |_r, samp, inf| {
    let sample_ref_raw: Pin<Box<DDS_Bytes>> = Box::pin(unsafe { *samp });
    let sample_ref: &Bytes = &Bytes {
        inner: Some(sample_ref_raw),
    };
    let info_ref_raw: &mut DDS_SampleInfo = unsafe { &mut *inf };
let info_ref: &SampleInfo = &SampleInfo { raw: info_ref_raw };
    handle_one_video_sample(sample_ref, info_ref);
});

dds_simple_data_reader_listener!(video_delete, DDS_Bytes, |_r, samp, inf| {
    let sample_ref_raw: Pin<Box<DDS_Bytes>> = Box::pin(unsafe { *samp });
    let sample_ref: &Bytes = &Bytes {
        inner: Some(sample_ref_raw),
    };
    let info_ref_raw: &mut DDS_SampleInfo = unsafe { &mut *inf };
let info_ref: &SampleInfo = &SampleInfo { raw: info_ref_raw };
    handle_one_video_delete_sample(sample_ref, info_ref);
});

dds_simple_data_reader_listener!(image_queue, DDS_Bytes, |_r, samp, inf| {
    let sample_ref_raw: Pin<Box<DDS_Bytes>> = Box::pin(unsafe { *samp });
    let sample_ref: &Bytes = &Bytes {
        inner: Some(sample_ref_raw),
    };
    let info_ref_raw: &mut DDS_SampleInfo = unsafe { &mut *inf };
let info_ref: &SampleInfo = &SampleInfo { raw: info_ref_raw };
    handle_one_image_queue_sample(sample_ref, info_ref);
});

dds_simple_data_reader_listener!(image_queue_delete, DDS_Bytes, |_r, samp, inf| {
    let sample_ref_raw: Pin<Box<DDS_Bytes>> = Box::pin(unsafe { *samp });
    let sample_ref: &Bytes = &Bytes {
        inner: Some(sample_ref_raw),
    };
    let info_ref_raw: &mut DDS_SampleInfo = unsafe { &mut *inf };
let info_ref: &SampleInfo = &SampleInfo { raw: info_ref_raw };
    handle_one_image_queue_delete_sample(sample_ref, info_ref);
});
