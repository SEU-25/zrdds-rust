use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};
use std::collections::HashMap;
use std::pin::Pin;
use std::ptr;
use std::sync::{Arc, Mutex};
use zrdds::bindings::*;
use zrdds::core::dp_listener::DPListener;
use zrdds::core::publisher_listener::PublisherListener;
use zrdds::core::subscriber_listener::SubscriberListener;
use zrdds::core::topic_listener::TopicListener;
use zrdds::core::type_support::{type_support_get_name, type_support_register_type};
use zrdds::core::writer_listener::WriterListener;
use zrdds::core::{
    DPFactory, PublisherQos, ReaderListener, ReaderQos, SubscriberQos, TopicQos, WriterQos,
};
use zrdds::dds_handlers::*;
use zrdds::dioxus_app::*;
use zrdds::dioxus_structs::{
    ChatMessage, DanmakuMessage, DrawStroke, EraseOperation, ImageDeleteOperation,
    ImageQueueDeleteOperation, MouseState, VideoDeleteOperation,
};
use zrdds::dioxus_structs::{
    DANMAKU_ENABLED, RECEIVED, RECEIVED_CHAT_MESSAGES, RECEIVED_DANMAKU_MESSAGES, RECEIVED_ERASES,
    RECEIVED_IMAGE_DELETES, RECEIVED_IMAGE_QUEUE_DELETES, RECEIVED_IMAGE_QUEUES, RECEIVED_IMAGES,
    RECEIVED_STROKES, RECEIVED_USER_COLORS, RECEIVED_VIDEO_DELETES, RECEIVED_VIDEOS,
};
use zrdds::dioxus_structs::{ImageData as CustomImageData, VideoData as CustomVideoData};
use zrdds::utils::*;

// 全局props变量
static mut DIOXUS_PROPS: Option<DioxusAppProps> = None;

// 应用包装函数
fn app_wrapper() -> Element {
    unsafe {
        if let Some(ref props) = DIOXUS_PROPS {
            DioxusApp(props.clone())
        } else {
            rsx! { div { "Error: Props not initialized" } }
        }
    }
}

// 启动应用的主要逻辑
fn start_app(domain_id: u32, user_type: u32) {
    // 初始化共享状态
    let received: Arc<Mutex<HashMap<String, MouseState>>> = Arc::new(Mutex::new(HashMap::new()));
    let received_images: Arc<Mutex<HashMap<String, CustomImageData>>> =
        Arc::new(Mutex::new(HashMap::new()));
    let received_strokes: Arc<Mutex<Vec<DrawStroke>>> = Arc::new(Mutex::new(Vec::new()));
    let received_erases: Arc<Mutex<Vec<EraseOperation>>> = Arc::new(Mutex::new(Vec::new()));
    let received_image_deletes: Arc<Mutex<Vec<ImageDeleteOperation>>> =
        Arc::new(Mutex::new(Vec::new()));
    let received_chat_messages: Arc<Mutex<Vec<ChatMessage>>> = Arc::new(Mutex::new(Vec::new()));
    let received_danmaku_messages: Arc<Mutex<Vec<DanmakuMessage>>> =
        Arc::new(Mutex::new(Vec::new()));
    let received_videos: Arc<Mutex<HashMap<String, CustomVideoData>>> =
        Arc::new(Mutex::new(HashMap::new()));
    let received_video_deletes: Arc<Mutex<Vec<VideoDeleteOperation>>> =
        Arc::new(Mutex::new(Vec::new()));
    let received_user_colors: Arc<Mutex<HashMap<String, zrdds::dioxus_structs::UserColor>>> =
        Arc::new(Mutex::new(HashMap::new()));
    let received_image_queues: Arc<Mutex<HashMap<String, zrdds::dioxus_structs::ImageQueue>>> =
        Arc::new(Mutex::new(HashMap::new()));
    let received_image_queue_deletes: Arc<Mutex<Vec<ImageQueueDeleteOperation>>> =
        Arc::new(Mutex::new(Vec::new()));
    let danmaku_enabled: Arc<Mutex<bool>> = Arc::new(Mutex::new(true));

    unsafe {
        // 设置全局静态变量
        RECEIVED = Some(received.clone());
        RECEIVED_IMAGES = Some(received_images.clone());
        RECEIVED_STROKES = Some(received_strokes.clone());
        RECEIVED_ERASES = Some(received_erases.clone());
        RECEIVED_IMAGE_DELETES = Some(received_image_deletes.clone());
        RECEIVED_CHAT_MESSAGES = Some(received_chat_messages.clone());
        RECEIVED_DANMAKU_MESSAGES = Some(received_danmaku_messages.clone());
        RECEIVED_VIDEOS = Some(received_videos.clone());
        RECEIVED_VIDEO_DELETES = Some(received_video_deletes.clone());
        RECEIVED_USER_COLORS = Some(received_user_colors.clone());
        RECEIVED_IMAGE_QUEUES = Some(received_image_queues.clone());
        RECEIVED_IMAGE_QUEUE_DELETES = Some(received_image_queue_deletes.clone());
        DANMAKU_ENABLED = Some(danmaku_enabled.clone());
    }

    // DDS 初始化
    let factory = DPFactory::instance().unwrap();

    let dp_qos = factory.default_qos();

    let participant = factory
        .create_dp(
            &factory,
            domain_id,
            &dp_qos,
            &DPListener {
                raw: ptr::null_mut(),
            },
            DDS_STATUS_MASK_NONE,
        )
        .unwrap();
    let type_name = type_support_get_name();
    type_support_register_type(&participant, type_name.as_str());

    let topic_name = String::from("mouse_topic");
    let topic_qos = TopicQos::default_qos();
    let topic = participant
        .create_topic(
            &participant,
            topic_name.as_str(),
            type_name.as_str(),
            &topic_qos,
            &TopicListener {
                raw: ptr::null_mut(),
            },
            DDS_STATUS_MASK_NONE,
        )
        .unwrap();

    // 创建图片topic
    let image_topic_name = String::from("image_topic");
    let image_topic = participant
        .create_topic(
            &participant,
            image_topic_name.as_str(),
            type_name.as_str(),
            &topic_qos,
            &TopicListener {
                raw: ptr::null_mut(),
            },
            DDS_STATUS_MASK_NONE,
        )
        .unwrap();

    // 创建画笔topic
    let draw_topic_name = String::from("draw_topic");
    let draw_topic = participant
        .create_topic(
            &participant,
            draw_topic_name.as_str(),
            type_name.as_str(),
            &topic_qos,
            &TopicListener {
                raw: ptr::null_mut(),
            },
            DDS_STATUS_MASK_NONE,
        )
        .unwrap();

    // 创建擦除topic
    let erase_topic_name = String::from("erase_topic");
    let erase_topic = participant
        .create_topic(
            &participant,
            erase_topic_name.as_str(),
            type_name.as_str(),
            &topic_qos,
            &TopicListener {
                raw: ptr::null_mut(),
            },
            DDS_STATUS_MASK_NONE,
        )
        .unwrap();

    // 创建图片删除topic
    let image_delete_topic_name = String::from("image_delete_topic");
    let image_delete_topic = participant
        .create_topic(
            &participant,
            image_delete_topic_name.as_str(),
            type_name.as_str(),
            &topic_qos,
            &TopicListener {
                raw: ptr::null_mut(),
            },
            DDS_STATUS_MASK_NONE,
        )
        .unwrap();

    // 创建聊天topic
    let chat_topic_name = String::from("chat_topic");
    let chat_topic = participant
        .create_topic(
            &participant,
            chat_topic_name.as_str(),
            type_name.as_str(),
            &topic_qos,
            &TopicListener {
                raw: ptr::null_mut(),
            },
            DDS_STATUS_MASK_NONE,
        )
        .unwrap();

    // 创建视频topic
    let video_topic_name = String::from("video_topic");
    let video_topic = participant
        .create_topic(
            &participant,
            video_topic_name.as_str(),
            type_name.as_str(),
            &topic_qos,
            &TopicListener {
                raw: ptr::null_mut(),
            },
            DDS_STATUS_MASK_NONE,
        )
        .unwrap();

    // 创建视频删除topic
    let video_delete_topic_name = String::from("video_delete_topic");
    let video_delete_topic = participant
        .create_topic(
            &participant,
            video_delete_topic_name.as_str(),
            type_name.as_str(),
            &topic_qos,
            &TopicListener {
                raw: ptr::null_mut(),
            },
            DDS_STATUS_MASK_NONE,
        )
        .unwrap();

    // 创建弹幕topic
    let danmaku_topic_name = String::from("danmaku_topic");
    let danmaku_topic = participant
        .create_topic(
            &participant,
            danmaku_topic_name.as_str(),
            type_name.as_str(),
            &topic_qos,
            &TopicListener {
                raw: ptr::null_mut(),
            },
            DDS_STATUS_MASK_NONE,
        )
        .unwrap();

    // 创建用户颜色topic
    let user_color_topic_name = String::from("user_color_topic");
    let user_color_topic = participant
        .create_topic(
            &participant,
            user_color_topic_name.as_str(),
            type_name.as_str(),
            &topic_qos,
            &TopicListener {
                raw: ptr::null_mut(),
            },
            DDS_STATUS_MASK_NONE,
        )
        .unwrap();

    // 创建图片队列topic
    let image_queue_topic_name = String::from("image_queue_topic");
    let image_queue_topic = participant
        .create_topic(
            &participant,
            image_queue_topic_name.as_str(),
            type_name.as_str(),
            &topic_qos,
            &TopicListener {
                raw: ptr::null_mut(),
            },
            DDS_STATUS_MASK_NONE,
        )
        .unwrap();

    // 创建图片队列删除topic
    let image_queue_delete_topic_name = String::from("image_queue_delete_topic");
    let image_queue_delete_topic = participant
        .create_topic(
            &participant,
            image_queue_delete_topic_name.as_str(),
            type_name.as_str(),
            &topic_qos,
            &TopicListener {
                raw: ptr::null_mut(),
            },
            DDS_STATUS_MASK_NONE,
        )
        .unwrap();

    // 创建publisher
    let publisher_qos = PublisherQos::default_qos();
    let publisher = participant
        .create_publisher(
            &participant,
            &publisher_qos,
            &PublisherListener {
                raw: ptr::null_mut(),
            },
            DDS_STATUS_MASK_NONE,
        )
        .unwrap();

    // datawriter qos的可靠性为可靠、持久化
    let mut writer_qos = WriterQos::new();
    let writer_qos_default = WriterQos::new();

    publisher.publisher_get_default_writer_qos(&mut writer_qos);

    writer_qos = writer_qos.get_for_now(Box::new(|p: WriterQos| -> Pin<Box<DDS_DataWriterQos>> {
        let mut inner = p.inner.unwrap();
        inner.reliability.kind = DDS_ReliabilityQosPolicyKind_DDS_RELIABLE_RELIABILITY_QOS;
        inner.durability.kind = DDS_DurabilityQosPolicyKind_DDS_TRANSIENT_LOCAL_DURABILITY_QOS;
        inner.history.kind = DDS_HistoryQosPolicyKind_DDS_KEEP_ALL_HISTORY_QOS;
        inner
    }));

    let writer = publisher
        .create_writer(
            &topic,
            &writer_qos_default,
            &mut WriterListener::none(),
            DDS_STATUS_MASK_NONE,
        )
        .unwrap();

    let draw_writer = publisher
        .create_writer(
            &draw_topic,
            &writer_qos,
            &mut WriterListener::none(),
            DDS_STATUS_MASK_NONE,
        )
        .unwrap();

    let erase_writer = publisher
        .create_writer(
            &erase_topic,
            &writer_qos,
            &mut WriterListener::none(),
            DDS_STATUS_MASK_NONE,
        )
        .unwrap();

    let image_delete_writer = publisher
        .create_writer(
            &image_delete_topic,
            &writer_qos,
            &mut WriterListener::none(),
            DDS_STATUS_MASK_NONE,
        )
        .unwrap();

    let chat_writer = publisher
        .create_writer(
            &chat_topic,
            &writer_qos,
            &mut WriterListener::none(),
            DDS_STATUS_MASK_NONE,
        )
        .unwrap();

    let video_writer = publisher
        .create_writer(
            &video_topic,
            &writer_qos,
            &mut WriterListener::none(),
            DDS_STATUS_MASK_NONE,
        )
        .unwrap();

    let video_delete_writer = publisher
        .create_writer(
            &video_delete_topic,
            &writer_qos,
            &mut WriterListener::none(),
            DDS_STATUS_MASK_NONE,
        )
        .unwrap();

    let danmaku_writer = publisher
        .create_writer(
            &danmaku_topic,
            &writer_qos,
            &mut WriterListener::none(),
            DDS_STATUS_MASK_NONE,
        )
        .unwrap();

    let user_color_writer = publisher
        .create_writer(
            &user_color_topic,
            &writer_qos,
            &mut WriterListener::none(),
            DDS_STATUS_MASK_NONE,
        )
        .unwrap();

    let image_queue_writer = publisher
        .create_writer(
            &image_queue_topic,
            &writer_qos,
            &mut WriterListener::none(),
            DDS_STATUS_MASK_NONE,
        )
        .unwrap();

    let image_queue_delete_writer = publisher
        .create_writer(
            &image_queue_delete_topic,
            &writer_qos,
            &mut WriterListener::none(),
            DDS_STATUS_MASK_NONE,
        )
        .unwrap();

    // 包装在Arc<Mutex<>>中
    let writer = Arc::new(Mutex::new(writer));
    let draw_writer = Arc::new(Mutex::new(draw_writer));
    let erase_writer = Arc::new(Mutex::new(erase_writer));
    let image_delete_writer = Arc::new(Mutex::new(image_delete_writer));
    let chat_writer = Arc::new(Mutex::new(chat_writer));
    let video_writer = Arc::new(Mutex::new(video_writer));
    let video_delete_writer = Arc::new(Mutex::new(video_delete_writer));
    let danmaku_writer = Arc::new(Mutex::new(danmaku_writer));
    let user_color_writer = Arc::new(Mutex::new(user_color_writer));
    let image_queue_writer = Arc::new(Mutex::new(image_queue_writer));
    let image_queue_delete_writer = Arc::new(Mutex::new(image_queue_delete_writer));

    // 创建subscriber
    let subscriber_qos = SubscriberQos::default_qos();

    let subscriber = participant
        .create_subscriber(
            &participant,
            &subscriber_qos,
            &SubscriberListener {
                raw: ptr::null_mut(),
            },
            DDS_STATUS_MASK_NONE,
        )
        .unwrap();

    let mut reader_qos = ReaderQos::new();
    let reader_qos_default = ReaderQos::new();

    subscriber.subscriber_get_default_reader_qos(&mut reader_qos);
    reader_qos = reader_qos.get_for_now(Box::new(|p: ReaderQos| -> Pin<Box<DDS_DataReaderQos>> {
        let mut inner = p.inner.unwrap();
        inner.reliability.kind = DDS_ReliabilityQosPolicyKind_DDS_RELIABLE_RELIABILITY_QOS;
        inner.durability.kind = DDS_DurabilityQosPolicyKind_DDS_TRANSIENT_LOCAL_DURABILITY_QOS;
        inner.history.kind = DDS_HistoryQosPolicyKind_DDS_KEEP_LAST_HISTORY_QOS;
        inner
    }));

    // 创建各种listener和reader
    let mut listener = ReaderListener::new();
    listener.set_on_data_available(mouse_on_data_available);

    let _reader = subscriber.create_reader(
        &topic.get_description(),
        &reader_qos_default,
        &mut listener,
        DDS_STATUS_MASK_ALL,
    );

    let mut draw_listener = ReaderListener::new();
    draw_listener.set_on_data_available(draw_on_data_available);

    let _draw_reader = subscriber.create_reader(
        &draw_topic.get_description(),
        &reader_qos,
        &mut draw_listener,
        DDS_STATUS_MASK_ALL,
    );

    let mut image_listener = ReaderListener::new();
    image_listener.set_on_data_available(image_on_data_available);

    let _image_reader = subscriber.create_reader(
        &image_topic.get_description(),
        &reader_qos,
        &mut image_listener,
        DDS_STATUS_MASK_ALL,
    );

    let mut erase_listener = ReaderListener::new();
    erase_listener.set_on_data_available(erase_on_data_available);

    let _erase_reader = subscriber.create_reader(
        &erase_topic.get_description(),
        &reader_qos,
        &mut erase_listener,
        DDS_STATUS_MASK_ALL,
    );

    let mut image_delete_listener = ReaderListener::new();
    image_delete_listener.set_on_data_available(image_delete_on_data_available);

    let _image_delete_reader = subscriber.create_reader(
        &image_delete_topic.get_description(),
        &reader_qos,
        &mut image_delete_listener,
        DDS_STATUS_MASK_ALL,
    );

    let mut chat_listener = ReaderListener::new();
    chat_listener.set_on_data_available(chat_on_data_available);

    let _chat_reader = subscriber.create_reader(
        &chat_topic.get_description(),
        &reader_qos,
        &mut chat_listener,
        DDS_STATUS_MASK_ALL,
    );

    let mut video_listener = ReaderListener::new();
    video_listener.set_on_data_available(video_on_data_available);

    let _video_reader = subscriber.create_reader(
        &&video_topic.get_description(),
        &reader_qos,
        &mut video_listener,
        DDS_STATUS_MASK_ALL,
    );

    let mut video_delete_listener = ReaderListener::new();
    video_delete_listener.set_on_data_available(video_delete_on_data_available);

    let _video_delete_reader = subscriber.create_reader(
        &video_delete_topic.get_description(),
        &reader_qos,
        &mut video_delete_listener,
        DDS_STATUS_MASK_ALL,
    );

    let mut danmaku_listener = ReaderListener::new();
    danmaku_listener.set_on_data_available(danmaku_on_data_available);

    let _danmaku_reader = subscriber.create_reader(
        &danmaku_topic.get_description(),
        &reader_qos,
        &mut danmaku_listener,
        DDS_STATUS_MASK_ALL,
    );

    let mut user_color_listener = ReaderListener::new();
    user_color_listener.set_on_data_available(user_color_on_data_available);

    let _user_color_reader = subscriber.create_reader(
        &user_color_topic.get_description(),
        &reader_qos,
        &mut user_color_listener,
        DDS_STATUS_MASK_ALL,
    );

    let mut image_queue_listener = ReaderListener::new();
    image_queue_listener.set_on_data_available(image_queue_on_data_available);

    let _image_queue_reader = subscriber.create_reader(
        &image_queue_topic.get_description(),
        &reader_qos,
        &mut image_queue_listener,
        DDS_STATUS_MASK_ALL,
    );

    let mut image_queue_delete_listener = ReaderListener::new();
    image_queue_delete_listener.set_on_data_available(image_queue_delete_on_data_available);

    let _image_queue_delete_reader = subscriber.create_reader(
        &image_queue_delete_topic.get_description(),
        &reader_qos,
        &mut image_queue_delete_listener,
        DDS_STATUS_MASK_ALL,
    );

    // 配置窗口
    let config = Config::new()
        .with_window(
            WindowBuilder::new()
                .with_title("在线课堂")
                .with_inner_size(dioxus_desktop::LogicalSize::new(1920, 780))
                .with_maximized(true)
                .with_resizable(false), //.with_min_inner_size(dioxus_desktop::LogicalSize::new(800, 600))
                                        //.with_maximized(true)
        )
        .with_custom_head(
            r#"
            <style>
                body {
                    margin: 0;
                    padding: 0;
                    font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
                    overflow: hidden;
                }
                
                * {
                    box-sizing: border-box;
                }
                
                button {
                    cursor: pointer;
                    border: 1px solid #ccc;
                    border-radius: 4px;
                    transition: background-color 0.2s;
                }
                
                button:hover {
                    background-color: #f0f0f0;
                }
                
                input[type="text"], input[type="color"], input[type="range"] {
                    border: 1px solid #ccc;
                    border-radius: 4px;
                    padding: 4px;
                }
                
                input[type="text"]:focus {
                    outline: none;
                    border-color: #007bff;
                    box-shadow: 0 0 0 2px rgba(0, 123, 255, 0.25);
                }
                
                .canvas-container {
                    position: relative;
                    border: 2px solid #333;
                    background-color: white;
                    overflow: hidden;
                }
                
                .danmaku {
                    position: absolute;
                    font-weight: bold;
                    text-shadow: 1px 1px 1px rgba(0,0,0,0.5);
                    pointer-events: none;
                    white-space: nowrap;
                    animation: danmaku-move 10s linear;
                }
                
                @keyframes danmaku-move {
                    from {
                        transform: translateX(0);
                    }
                    to {
                        transform: translateX(-100vw);
                    }
                }
                
                .chat-message {
                    margin-bottom: 8px;
                    padding: 8px;
                    background-color: white;
                    border-radius: 6px;
                    border: 1px solid #e0e0e0;
                    box-shadow: 0 1px 2px rgba(0,0,0,0.1);
                }
                
                .chat-username {
                    font-weight: bold;
                    color: #007bff;
                    font-size: 12px;
                    margin-bottom: 2px;
                }
                
                .chat-content {
                    margin-bottom: 2px;
                    line-height: 1.4;
                }
                
                .chat-timestamp {
                    font-size: 10px;
                    color: #666;
                }
                
                .toolbar {
                    display: flex;
                    align-items: center;
                    gap: 15px;
                    padding: 12px;
                    border-bottom: 1px solid #ccc;
                    background: linear-gradient(to bottom, #f8f9fa, #e9ecef);
                    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
                }
                
                .mode-button {
                    padding: 8px 12px;
                    border: 1px solid #ccc;
                    border-radius: 4px;
                    background-color: #fff;
                    transition: all 0.2s;
                }
                
                .mode-button.active {
                    background-color: #007bff;
                    color: white;
                    border-color: #007bff;
                }
                
                .mode-button:hover {
                    background-color: #f0f0f0;
                }
                
                .mode-button.active:hover {
                    background-color: #0056b3;
                }
                
                .upload-button {
                    padding: 8px 12px;
                    border: none;
                    border-radius: 4px;
                    color: white;
                    font-weight: bold;
                    transition: background-color 0.2s;
                }
                
                .upload-image {
                    background-color: #28a745;
                }
                
                .upload-image:hover {
                    background-color: #218838;
                }
                
                .upload-video {
                    background-color: #17a2b8;
                }
                
                .upload-video:hover {
                    background-color: #138496;
                }
                
                .chat-panel {
                    width: 320px;
                    display: flex;
                    flex-direction: column;
                    border-left: 1px solid #ccc;
                    background-color: #f8f9fa;
                }
                
                .chat-header {
                    padding: 12px;
                    border-bottom: 1px solid #ccc;
                    background: linear-gradient(to bottom, #e9ecef, #dee2e6);
                }
                
                .chat-messages {
                    flex: 1;
                    overflow-y: auto;
                    padding: 10px;
                    max-height: calc(100vh - 200px);
                }
                
                .chat-input-area {
                    padding: 12px;
                    border-top: 1px solid #ccc;
                    background-color: #fff;
                }
                
                .chat-input-container {
                    display: flex;
                    gap: 8px;
                }
                
                .chat-input {
                    flex: 1;
                    padding: 8px;
                    border: 1px solid #ccc;
                    border-radius: 4px;
                    font-size: 14px;
                }
                
                .send-button {
                    padding: 8px 16px;
                    background-color: #007bff;
                    color: white;
                    border: none;
                    border-radius: 4px;
                    font-weight: bold;
                    cursor: pointer;
                    transition: background-color 0.2s;
                }
                
                .send-button:hover {
                    background-color: #0056b3;
                }
                
                .media-container {
                    position: absolute;
                    top: 50%;
                    left: 50%;
                    transform: translate(-50%, -50%);
                    max-width: 90%;
                    max-height: 90%;
                }
                
                .media-delete-button {
                    position: absolute;
                    top: 5px;
                    right: 5px;
                    background-color: #dc3545;
                    color: white;
                    border: none;
                    border-radius: 50%;
                    width: 24px;
                    height: 24px;
                    cursor: pointer;
                    font-size: 14px;
                    font-weight: bold;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                }
                
                .media-delete-button:hover {
                    background-color: #c82333;
                }
                
                .mouse-cursor {
                    position: absolute;
                    pointer-events: none;
                    z-index: 100;
                }
                
                .stroke-line {
                    stroke-linecap: round;
                    stroke-linejoin: round;
                }
            </style>
        "#
            .to_string(),
        );

    // 设置全局props
    unsafe {
        DIOXUS_PROPS = Some(DioxusAppProps {
            domain_id,
            user_type,
            received: received.clone(),
            received_images: received_images.clone(),
            received_videos: received_videos.clone(),
            received_strokes: received_strokes.clone(),
            received_erases: received_erases.clone(),
            received_image_deletes: received_image_deletes.clone(),
            received_video_deletes: received_video_deletes.clone(),
            received_chat_messages: received_chat_messages.clone(),
            received_danmaku_messages: received_danmaku_messages.clone(),
            received_user_colors: received_user_colors.clone(),
            received_image_queues: received_image_queues.clone(),
            received_image_queue_deletes: received_image_queue_deletes.clone(),
            writer: writer.clone(),
            video_writer: video_writer.clone(),
            draw_writer: draw_writer.clone(),
            erase_writer: erase_writer.clone(),
            image_delete_writer: image_delete_writer.clone(),
            video_delete_writer: video_delete_writer.clone(),
            chat_writer: chat_writer.clone(),
            danmaku_writer: danmaku_writer.clone(),
            color_writer: user_color_writer.clone(),
            image_queue_writer: image_queue_writer.clone(),
            image_queue_delete_writer: image_queue_delete_writer.clone(),
        });
    }

    // 启动应用
    LaunchBuilder::new().with_cfg(config).launch(app_wrapper);
}

fn main() {
    // 从命令行参数读取域号和用户类型，如果没有参数则使用默认值
    let args: Vec<String> = std::env::args().collect();
    let domain_id = if args.len() > 1 {
        args[1].parse::<u32>().unwrap_or(0)
    } else {
        0
    };

    let user_type = if args.len() > 2 {
        args[2].parse::<u32>().unwrap_or(1)
    } else {
        1
    };

    // 使用域号和用户类型启动应用，域号为0时会显示域号输入界面
    start_app(domain_id, user_type);
}
