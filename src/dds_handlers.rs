use std::{ffi::CString, ptr, mem};
use crate::bindings::*;
use std::sync::{Arc, Mutex};
use egui::Color32;
use base64::{Engine as _, engine::general_purpose};

use crate::dioxus_structs::*;
use crate::utils::color_from_json;

// 画笔笔迹消息回调函数
pub extern "C" fn on_draw_data_available(reader: *mut DDS_DataReader) {
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
                                let color = if color_array.len() >= 4 {
                                    Color32::from_rgba_unmultiplied(
                                        color_array[0].as_u64().unwrap() as u8,
                                        color_array[1].as_u64().unwrap() as u8,
                                        color_array[2].as_u64().unwrap() as u8,
                                        color_array[3].as_u64().unwrap() as u8,
                                    )
                                } else {
                                    Color32::from_rgb(
                                        color_array[0].as_u64().unwrap() as u8,
                                        color_array[1].as_u64().unwrap() as u8,
                                        color_array[2].as_u64().unwrap() as u8,
                                    )
                                };
                                
                                let stroke = DrawStroke {
                                    username: draw_msg["username"].as_str().unwrap_or("unknown").to_string(),
                                    color,
                                    start_x: draw_msg["start_x"].as_f64().unwrap_or(0.0) as f32,
                                    start_y: draw_msg["start_y"].as_f64().unwrap_or(0.0) as f32,
                                    end_x: draw_msg["end_x"].as_f64().unwrap_or(0.0) as f32,
                                    end_y: draw_msg["end_y"].as_f64().unwrap_or(0.0) as f32,
                                    stroke_width: draw_msg["stroke_width"].as_f64().unwrap_or(2.0) as f32,
                                    timestamp: draw_msg["timestamp"].as_u64().unwrap_or(0),
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

pub extern "C" fn on_danmaku_data_available(reader: *mut DDS_DataReader) {
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

                if let Ok(json_str) = String::from_utf8(vec) {
                     if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&json_str) {
                         if let Some(danmaku_message) = parse_danmaku_message(&json_value) {
                             if let Some(ref received_danmaku_messages) = RECEIVED_DANMAKU_MESSAGES {
                                 received_danmaku_messages.lock().unwrap().push(danmaku_message);
                             }
                         }
                     }
                }
            }
        }

        DDS_BytesDataReader_return_loan(reader, &mut data_values, &mut sample_infos);
        DDS_BytesSeq_finalize(&mut data_values);
        DDS_SampleInfoSeq_finalize(&mut sample_infos);
    }
}

// 解析弹幕消息的辅助函数
fn parse_danmaku_message(json_value: &serde_json::Value) -> Option<DanmakuMessage> {
    let username = json_value["username"].as_str().unwrap_or("unknown").to_string();
    let message = json_value["message"].as_str().unwrap_or("").to_string();
    let start_time = json_value["start_time"].as_f64().unwrap_or(0.0);
    let danmaku_id = format!("{}-{}-{}", username, message.len(), (start_time * 1000.0) as u64);
    
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

// 图片消息回调函数
pub extern "C" fn on_image_data_available(reader: *mut DDS_DataReader) {
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
pub extern "C" fn on_erase_data_available(reader: *mut DDS_DataReader) {
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
                                timestamp: erase_msg["timestamp"].as_u64().unwrap_or(0),
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
pub extern "C" fn on_image_delete_data_available(reader: *mut DDS_DataReader) {
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
pub extern "C" fn on_chat_data_available(reader: *mut DDS_DataReader) {
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
                        let username = chat_msg["username"].as_str().unwrap_or("unknown").to_string();
                        let message = chat_msg["message"].as_str().unwrap_or("").to_string();
                        let timestamp = chat_msg["timestamp"].as_str().unwrap_or("").to_string();
                        
                        // 解析颜色信息
                        let color = if let Some(color_obj) = chat_msg["color"].as_object() {
                            if let (Some(r), Some(g), Some(b), Some(a)) = (
                                color_obj["r"].as_u64(),
                                color_obj["g"].as_u64(),
                                color_obj["b"].as_u64(),
                                color_obj["a"].as_u64()
                            ) {
                                Color32::from_rgba_premultiplied(r as u8, g as u8, b as u8, a as u8)
                            } else {
                                Color32::WHITE // 默认颜色
                            }
                        } else {
                            Color32::WHITE // 默认颜色
                        };
                        
                        // 添加到聊天消息
                        if let Some(ref received_chat_messages_clone) = RECEIVED_CHAT_MESSAGES {
                            let mut data = received_chat_messages_clone.lock().unwrap();
                            data.push(ChatMessage {
                                username: username.clone(),
                                message: message.clone(),
                                timestamp: timestamp.clone(),
                                color: color,
                            });
                        }
                        
                        // 同时添加到弹幕消息
                        if let Some(ref received_danmaku_messages_clone) = RECEIVED_DANMAKU_MESSAGES {
                            let mut data = received_danmaku_messages_clone.lock().unwrap();
                            let current_time = std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap_or_default()
                                .as_secs_f64();
                            let y_position = (data.len() % 10) as f32 * 50.0 + 50.0;
                            let danmaku_id = format!("{}-{}-{}", username, message.len(), (current_time * 1000.0) as u64);
                            data.push(DanmakuMessage {
                                username: username,
                                message: message,
                                start_time: current_time,
                                x: 1200.0, // 从屏幕最右侧开始
                                y: y_position, // 分层显示
                                color: color,
                                speed: 100.0,
                                id: danmaku_id,
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
pub extern "C" fn on_data_available(reader: *mut DDS_DataReader) {
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
                    if let Ok(mouse_state) = serde_json::from_str::<serde_json::Value>(&s) {
                        if let Some(ref received_clone) = RECEIVED {
                            let mut data = received_clone.lock().unwrap();
                            data.insert(
                                mouse_state["username"].to_string(),
                                MouseState {
                                    username: mouse_state["username"].to_string(),
                                    color: color_from_json(&mouse_state["color"]),
                                    x: mouse_state["x"].as_f64().unwrap() as f32,
                                    y: mouse_state["y"].as_f64().unwrap() as f32,
                                }
                            );
                        }
                    }
                }
            }
        }

        DDS_BytesDataReader_return_loan(reader, &mut data_values, &mut sample_infos);
    }
}

// 视频消息回调函数
pub extern "C" fn on_video_data_available(reader: *mut DDS_DataReader) {
    unsafe {
        println!("1");
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

        println!("2");

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


                println!("3");
                if let Ok(s) = String::from_utf8(vec) {
                    println!("4");
                    if let Ok(video_msg) = serde_json::from_str::<serde_json::Value>(&s) {
                        println!("5");
                        if let Some(ref received_videos_clone) = RECEIVED_VIDEOS {
                            let video_data_base64 = video_msg["video_data"].as_str().unwrap_or("");
                            if let Ok(video_data) = general_purpose::STANDARD.decode(video_data_base64) {
                                println!("6");
                                let video = VideoData {
                                    username: video_msg["username"].as_str().unwrap_or("unknown").to_string(),
                                    video_data,
                                    file_name: video_msg["file_name"].as_str().unwrap_or("video.mp4").to_string(),
                                    file_size: video_msg["file_size"].as_u64().unwrap_or(0),
                                };
                                let file_name = video.file_name.clone();
                                let file_size = video.file_size;
                                let mut data = received_videos_clone.lock().unwrap();
                                data.insert(video.username.clone(), video);
                                println!("接收到视频: {} ({}字节)", file_name, file_size);
                            } else {
                                eprintln!("视频数据base64解码失败");
                            }
                        }
                    }
                }
            }
        }

        DDS_BytesDataReader_return_loan(reader, &mut data_values, &mut sample_infos);
    }
}

// 视频删除消息回调函数
pub extern "C" fn on_video_delete_data_available(reader: *mut DDS_DataReader) {
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
                        if let Some(ref received_video_deletes_clone) = RECEIVED_VIDEO_DELETES {
                            let mut data = received_video_deletes_clone.lock().unwrap();
                            data.push(VideoDeleteOperation {
                                username: delete_msg["username"].as_str().unwrap_or("unknown").to_string(),
                                video_id: delete_msg["video_id"].as_str().unwrap_or("").to_string(),
                            });
                        }
                    }
                }
            }
        }

        DDS_BytesDataReader_return_loan(reader, &mut data_values, &mut sample_infos);
    }
}

