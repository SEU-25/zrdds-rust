use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::{ffi::CString, ptr, mem};
use zrdds::bindings::*;

mod structs;
mod utils;
mod dds_handlers;
mod app;

use structs::*;
use utils::*;
use dds_handlers::*;
use app::*;

fn main() {
    // 初始化共享状态
    let received: Arc<Mutex<HashMap<String, MouseState>>> = Arc::new(Mutex::new(HashMap::new()));
    let received_images: Arc<Mutex<HashMap<String, ImageData>>> = Arc::new(Mutex::new(HashMap::new()));
    let received_strokes: Arc<Mutex<Vec<DrawStroke>>> = Arc::new(Mutex::new(Vec::new()));
    let received_erases: Arc<Mutex<Vec<EraseOperation>>> = Arc::new(Mutex::new(Vec::new()));
    let received_image_deletes: Arc<Mutex<Vec<ImageDeleteOperation>>> = Arc::new(Mutex::new(Vec::new()));
    let received_chat_messages: Arc<Mutex<Vec<ChatMessage>>> = Arc::new(Mutex::new(Vec::new()));
    let received_videos: Arc<Mutex<HashMap<String, VideoData>>> = Arc::new(Mutex::new(HashMap::new()));
    let received_video_deletes: Arc<Mutex<Vec<VideoDeleteOperation>>> = Arc::new(Mutex::new(Vec::new()));

    unsafe {
        // 设置全局静态变量
        RECEIVED = Some(received.clone());
        RECEIVED_IMAGES = Some(received_images.clone());
        RECEIVED_STROKES = Some(received_strokes.clone());
        RECEIVED_ERASES = Some(received_erases.clone());
        RECEIVED_IMAGE_DELETES = Some(received_image_deletes.clone());
        RECEIVED_CHAT_MESSAGES = Some(received_chat_messages.clone());
        RECEIVED_VIDEOS = Some(received_videos.clone());
        RECEIVED_VIDEO_DELETES = Some(received_video_deletes.clone());
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
        let topic = DDS_DomainParticipant_create_topic(
            participant,
            topic_name.as_ptr(),
            type_name,
            topic_qos,
            ptr::null_mut(),
            DDS_STATUS_MASK_NONE,
        );

        // 创建图片topic
        let image_topic_name = CString::new("image_topic").unwrap();
        let image_topic = DDS_DomainParticipant_create_topic(
            participant,
            image_topic_name.as_ptr(),
            type_name,
            topic_qos,
            ptr::null_mut(),
            DDS_STATUS_MASK_NONE,
        );

        // 创建画笔topic
        let draw_topic_name = CString::new("draw_topic").unwrap();
        let draw_topic = DDS_DomainParticipant_create_topic(
            participant,
            draw_topic_name.as_ptr(),
            type_name,
            topic_qos,
            ptr::null_mut(),
            DDS_STATUS_MASK_NONE,
        );

        // 创建擦除topic
        let erase_topic_name = CString::new("erase_topic").unwrap();
        let erase_topic = DDS_DomainParticipant_create_topic(
            participant,
            erase_topic_name.as_ptr(),
            type_name,
            topic_qos,
            ptr::null_mut(),
            DDS_STATUS_MASK_NONE,
        );

        // 创建图片删除topic
        let image_delete_topic_name = CString::new("image_delete_topic").unwrap();
        let image_delete_topic = DDS_DomainParticipant_create_topic(
            participant,
            image_delete_topic_name.as_ptr(),
            type_name,
            topic_qos,
            ptr::null_mut(),
            DDS_STATUS_MASK_NONE,
        );

        // 创建聊天topic
        let chat_topic_name = CString::new("chat_topic").unwrap();
        let chat_topic = DDS_DomainParticipant_create_topic(
            participant,
            chat_topic_name.as_ptr(),
            type_name,
            topic_qos,
            ptr::null_mut(),
            DDS_STATUS_MASK_NONE,
        );

        // 创建视频topic
        let video_topic_name = CString::new("video_topic").unwrap();
        let video_topic = DDS_DomainParticipant_create_topic(
            participant,
            video_topic_name.as_ptr(),
            type_name,
            topic_qos,
            ptr::null_mut(),
            DDS_STATUS_MASK_NONE,
        );

        // 创建视频删除topic
        let video_delete_topic_name = CString::new("video_delete_topic").unwrap();
        let video_delete_topic = DDS_DomainParticipant_create_topic(
            participant,
            video_delete_topic_name.as_ptr(),
            type_name,
            topic_qos,
            ptr::null_mut(),
            DDS_STATUS_MASK_NONE,
        );

        // 创建publisher
        let publisher_qos: *const DDS_PublisherQos = unsafe {
            &raw const DDS_PUBLISHER_QOS_DEFAULT
        };
        let publisher = DDS_DomainParticipant_create_publisher(
            participant,
            publisher_qos,
            ptr::null_mut(),
            DDS_STATUS_MASK_NONE,
        );

        // 创建datawriter
        let datawriter_qos: *const DDS_DataWriterQos = unsafe {
            &raw const DDS_DATAWRITER_QOS_DEFAULT
        };
        let writer = DDS_Publisher_create_datawriter(
            publisher,
            topic,
            datawriter_qos,
            ptr::null_mut(),
            DDS_STATUS_MASK_NONE,
        ) as *mut DDS_DataWriter;

        let image_writer = DDS_Publisher_create_datawriter(
            publisher,
            image_topic,
            datawriter_qos,
            ptr::null_mut(),
            DDS_STATUS_MASK_NONE,
        ) as *mut DDS_DataWriter;

        let draw_writer = DDS_Publisher_create_datawriter(
            publisher,
            draw_topic,
            datawriter_qos,
            ptr::null_mut(),
            DDS_STATUS_MASK_NONE,
        ) as *mut DDS_DataWriter;

        let erase_writer = DDS_Publisher_create_datawriter(
            publisher,
            erase_topic,
            datawriter_qos,
            ptr::null_mut(),
            DDS_STATUS_MASK_NONE,
        ) as *mut DDS_DataWriter;

        let image_delete_writer = DDS_Publisher_create_datawriter(
            publisher,
            image_delete_topic,
            datawriter_qos,
            ptr::null_mut(),
            DDS_STATUS_MASK_NONE,
        ) as *mut DDS_DataWriter;

        let chat_writer = DDS_Publisher_create_datawriter(
            publisher,
            chat_topic,
            datawriter_qos,
            ptr::null_mut(),
            DDS_STATUS_MASK_NONE,
        ) as *mut DDS_DataWriter;

        let video_writer = DDS_Publisher_create_datawriter(
            publisher,
            video_topic,
            datawriter_qos,
            ptr::null_mut(),
            DDS_STATUS_MASK_NONE,
        ) as *mut DDS_DataWriter;

        let video_delete_writer = DDS_Publisher_create_datawriter(
            publisher,
            video_delete_topic,
            datawriter_qos,
            ptr::null_mut(),
            DDS_STATUS_MASK_NONE,
        ) as *mut DDS_DataWriter;

        // 包装在Arc<Mutex<>>中
        let writer = Arc::new(Mutex::new(writer));
        let image_writer = Arc::new(Mutex::new(image_writer));
        let draw_writer = Arc::new(Mutex::new(draw_writer));
        let erase_writer = Arc::new(Mutex::new(erase_writer));
        let image_delete_writer = Arc::new(Mutex::new(image_delete_writer));
        let chat_writer = Arc::new(Mutex::new(chat_writer));
        let video_writer = Arc::new(Mutex::new(video_writer));
        let video_delete_writer = Arc::new(Mutex::new(video_delete_writer));

        // 创建subscriber
        let subscriber_qos: *const DDS_SubscriberQos = unsafe {
            &raw const DDS_SUBSCRIBER_QOS_DEFAULT
        };
        let subscriber = DDS_DomainParticipant_create_subscriber(
            participant,
            subscriber_qos,
            ptr::null_mut(),
            DDS_STATUS_MASK_NONE,
        );

        let datareader_qos: *const DDS_DataReaderQos = unsafe {
            &raw const DDS_DATAREADER_QOS_DEFAULT
        };

        // 创建各种listener和reader
        let mut listener: DDS_DataReaderListener = mem::zeroed();
        listener.on_data_available = Some(on_data_available);

        let _reader = DDS_Subscriber_create_datareader(
            subscriber,
            topic as *mut DDS_TopicDescription,
            datareader_qos,
            &mut listener,
            DDS_STATUS_MASK_ALL,
        ) as *mut DDS_DataReader;

        let mut draw_listener: DDS_DataReaderListener = mem::zeroed();
        draw_listener.on_data_available = Some(on_draw_data_available);

        let _draw_reader = DDS_Subscriber_create_datareader(
            subscriber,
            draw_topic as *mut DDS_TopicDescription,
            datareader_qos,
            &mut draw_listener,
            DDS_STATUS_MASK_ALL,
        ) as *mut DDS_DataReader;

        let mut image_listener: DDS_DataReaderListener = mem::zeroed();
        image_listener.on_data_available = Some(on_image_data_available);

        let _image_reader = DDS_Subscriber_create_datareader(
            subscriber,
            image_topic as *mut DDS_TopicDescription,
            datareader_qos,
            &mut image_listener,
            DDS_STATUS_MASK_ALL,
        ) as *mut DDS_DataReader;

        let mut erase_listener: DDS_DataReaderListener = mem::zeroed();
        erase_listener.on_data_available = Some(on_erase_data_available);

        let _erase_reader = DDS_Subscriber_create_datareader(
            subscriber,
            erase_topic as *mut DDS_TopicDescription,
            datareader_qos,
            &mut erase_listener,
            DDS_STATUS_MASK_ALL,
        ) as *mut DDS_DataReader;

        let mut image_delete_listener: DDS_DataReaderListener = mem::zeroed();
        image_delete_listener.on_data_available = Some(on_image_delete_data_available);

        let _image_delete_reader = DDS_Subscriber_create_datareader(
            subscriber,
            image_delete_topic as *mut DDS_TopicDescription,
            datareader_qos,
            &mut image_delete_listener,
            DDS_STATUS_MASK_ALL,
        ) as *mut DDS_DataReader;
        
        let mut chat_listener: DDS_DataReaderListener = mem::zeroed();
        chat_listener.on_data_available = Some(on_chat_data_available);

        let _chat_reader = DDS_Subscriber_create_datareader(
            subscriber,
            chat_topic as *mut DDS_TopicDescription,
            datareader_qos,
            &mut chat_listener,
            DDS_STATUS_MASK_ALL,
        ) as *mut DDS_DataReader;

        let mut video_listener: DDS_DataReaderListener = mem::zeroed();
        video_listener.on_data_available = Some(on_video_data_available);

        let _video_reader = DDS_Subscriber_create_datareader(
            subscriber,
            video_topic as *mut DDS_TopicDescription,
            datareader_qos,
            &mut video_listener,
            DDS_STATUS_MASK_ALL,
        ) as *mut DDS_DataReader;

        let mut video_delete_listener: DDS_DataReaderListener = mem::zeroed();
        video_delete_listener.on_data_available = Some(on_video_delete_data_available);

        let _video_delete_reader = DDS_Subscriber_create_datareader(
            subscriber,
            video_delete_topic as *mut DDS_TopicDescription,
            datareader_qos,
            &mut video_delete_listener,
            DDS_STATUS_MASK_ALL,
        ) as *mut DDS_DataReader;

        // 启动eframe应用
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_maximized(true)
                .with_resizable(false),
            ..Default::default()
        };
        eframe::run_native(
            "Shared Mouse Canvas",
            options,
            Box::new(move |cc| {
                Ok(Box::new(MouseApp::new(
                    received.clone(),
                    received_images.clone(),
                    received_videos.clone(),
                    received_strokes.clone(),
                    received_erases.clone(),
                    received_image_deletes.clone(),
                    received_video_deletes.clone(),
                    received_chat_messages.clone(),
                    writer.clone(),
                    image_writer.clone(),
                    video_writer.clone(),
                    draw_writer.clone(),
                    erase_writer.clone(),
                    image_delete_writer.clone(),
                    video_delete_writer.clone(),
                    chat_writer.clone(),
                    cc,
                )))
            }),
        );
    }
}