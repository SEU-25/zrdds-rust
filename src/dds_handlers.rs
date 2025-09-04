use std::{ffi::CString, ptr, mem};
use zrdds::bindings::*;
use std::sync::{Arc, Mutex};
use egui::Color32;
use base64::{Engine as _, engine::general_purpose};

use crate::structs::*;
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
                                let color = Color32::from_rgba_unmultiplied(
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
                    if let Ok(video_msg) = serde_json::from_str::<serde_json::Value>(&s) {
                        if let Some(ref received_videos_clone) = RECEIVED_VIDEOS {
                            let video_data_base64 = video_msg["video_data"].as_str().unwrap_or("");
                            if let Ok(video_data) = general_purpose::STANDARD.decode(video_data_base64) {
                                let video = VideoData {
                                    username: video_msg["username"].as_str().unwrap_or("unknown").to_string(),
                                    video_data,
                                    file_name: video_msg["file_name"].as_str().unwrap_or("video.mp4").to_string(),
                                    file_size: video_msg["file_size"].as_u64().unwrap_or(0),
                                };
                                let mut data = received_videos_clone.lock().unwrap();
                                data.insert(video.username.clone(), video);
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

pub fn initialize_dds() -> Result<(
    Arc<Mutex<*mut DDS_DataWriter>>,
    Arc<Mutex<*mut DDS_DataWriter>>,
    Arc<Mutex<*mut DDS_DataWriter>>,
    Arc<Mutex<*mut DDS_DataWriter>>,
    Arc<Mutex<*mut DDS_DataWriter>>,
    Arc<Mutex<*mut DDS_DataWriter>>,
), Box<dyn std::error::Error>> {
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
        
        let topic_qos: *const DDS_TopicQos = unsafe {
            &raw const DDS_TOPIC_QOS_DEFAULT
        };

        // 创建各种topic
        let topic_name = CString::new("mouse_topic").unwrap();
        let topic = DDS_DomainParticipant_create_topic(
            participant,
            topic_name.as_ptr(),
            type_name,
            topic_qos,
            ptr::null_mut(),
            DDS_STATUS_MASK_NONE,
        );

        let image_topic_name = CString::new("image_topic").unwrap();
        let image_topic = DDS_DomainParticipant_create_topic(
            participant,
            image_topic_name.as_ptr(),
            type_name,
            topic_qos,
            ptr::null_mut(),
            DDS_STATUS_MASK_NONE,
        );
        
        let draw_topic_name = CString::new("draw_topic").unwrap();
        let draw_topic = DDS_DomainParticipant_create_topic(
            participant,
            draw_topic_name.as_ptr(),
            type_name,
            topic_qos,
            ptr::null_mut(),
            DDS_STATUS_MASK_NONE,
        );

        let erase_topic_name = CString::new("erase_topic").unwrap();
        let erase_topic = DDS_DomainParticipant_create_topic(
            participant,
            erase_topic_name.as_ptr(),
            type_name,
            topic_qos,
            ptr::null_mut(),
            DDS_STATUS_MASK_NONE,
        );
        
        let image_delete_topic_name = CString::new("image_delete_topic").unwrap();
        let image_delete_topic = DDS_DomainParticipant_create_topic(
            participant,
            image_delete_topic_name.as_ptr(),
            type_name,
            topic_qos,
            ptr::null_mut(),
            DDS_STATUS_MASK_NONE,
        );
        
        let chat_topic_name = CString::new("chat_topic").unwrap();
        let chat_topic = DDS_DomainParticipant_create_topic(
            participant,
            chat_topic_name.as_ptr(),
            type_name,
            topic_qos,
            ptr::null_mut(),
            DDS_STATUS_MASK_NONE,
        );

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

        // 创建各种DataWriter
        let writer = DDS_Publisher_create_datawriter(
            publisher,
            topic,
            &datawriter_qos,
            ptr::null_mut(),
            DDS_STATUS_MASK_NONE,
        ) as *mut DDS_DataWriter;

        let image_writer = DDS_Publisher_create_datawriter(
            publisher,
            image_topic,
            &datawriter_qos,
            ptr::null_mut(),
            DDS_STATUS_MASK_NONE,
        ) as *mut DDS_DataWriter;
        
        let draw_writer = DDS_Publisher_create_datawriter(
            publisher,
            draw_topic,
            &datawriter_qos,
            ptr::null_mut(),
            DDS_STATUS_MASK_NONE,
        ) as *mut DDS_DataWriter;
        
        let erase_writer = DDS_Publisher_create_datawriter(
            publisher,
            erase_topic,
            &datawriter_qos,
            ptr::null_mut(),
            DDS_STATUS_MASK_NONE,
        ) as *mut DDS_DataWriter;
        
        let image_delete_writer = DDS_Publisher_create_datawriter(
            publisher,
            image_delete_topic,
            &datawriter_qos,
            ptr::null_mut(),
            DDS_STATUS_MASK_NONE,
        ) as *mut DDS_DataWriter;
        
        let chat_writer = DDS_Publisher_create_datawriter(
            publisher,
            chat_topic,
            &datawriter_qos,
            ptr::null_mut(),
            DDS_STATUS_MASK_NONE,
        ) as *mut DDS_DataWriter;

        // 用 Arc<Mutex<>> 包装 writer
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

        let mut datareader_qos: DDS_DataReaderQos = mem::zeroed();
        DDS_Subscriber_get_default_datareader_qos(subscriber, &mut datareader_qos);
        datareader_qos.history.depth = 5;

        // 创建各种DataReader和Listener
        let mut listener: DDS_DataReaderListener = mem::zeroed();
        listener.on_data_available = Some(on_data_available);

        let _reader = DDS_Subscriber_create_datareader(
            subscriber,
            topic as *mut DDS_TopicDescription,
            &datareader_qos,
            &mut listener,
            DDS_STATUS_MASK_ALL,
        ) as *mut DDS_DataReader;

        let mut image_listener: DDS_DataReaderListener = mem::zeroed();
        image_listener.on_data_available = Some(on_image_data_available);

        let _image_reader = DDS_Subscriber_create_datareader(
            subscriber,
            image_topic as *mut DDS_TopicDescription,
            &datareader_qos,
            &mut image_listener,
            DDS_STATUS_MASK_ALL,
        ) as *mut DDS_DataReader;
        
        let mut draw_listener: DDS_DataReaderListener = mem::zeroed();
        draw_listener.on_data_available = Some(on_draw_data_available);

        let _draw_reader = DDS_Subscriber_create_datareader(
            subscriber,
            draw_topic as *mut DDS_TopicDescription,
            &datareader_qos,
            &mut draw_listener,
            DDS_STATUS_MASK_ALL,
        ) as *mut DDS_DataReader;
        
        let mut erase_listener: DDS_DataReaderListener = mem::zeroed();
        erase_listener.on_data_available = Some(on_erase_data_available);

        let _erase_reader = DDS_Subscriber_create_datareader(
            subscriber,
            erase_topic as *mut DDS_TopicDescription,
            &datareader_qos,
            &mut erase_listener,
            DDS_STATUS_MASK_ALL,
        ) as *mut DDS_DataReader;
        
        let mut image_delete_listener: DDS_DataReaderListener = mem::zeroed();
        image_delete_listener.on_data_available = Some(on_image_delete_data_available);

        let _image_delete_reader = DDS_Subscriber_create_datareader(
            subscriber,
            image_delete_topic as *mut DDS_TopicDescription,
            &datareader_qos,
            &mut image_delete_listener,
            DDS_STATUS_MASK_ALL,
        ) as *mut DDS_DataReader;
        
        let mut chat_listener: DDS_DataReaderListener = mem::zeroed();
        chat_listener.on_data_available = Some(on_chat_data_available);

        let _chat_reader = DDS_Subscriber_create_datareader(
            subscriber,
            chat_topic as *mut DDS_TopicDescription,
            &datareader_qos,
            &mut chat_listener,
            DDS_STATUS_MASK_ALL,
        ) as *mut DDS_DataReader;

        Ok((
            writer,
            image_writer,
            draw_writer,
            erase_writer,
            image_delete_writer,
            chat_writer,
        ))
    }
}