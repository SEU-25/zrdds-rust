use crate::dioxus_structs::{
    ChatMessage, DANMAKU_ENABLED, DrawStroke, EraseOperation, ImageDeleteOperation, MouseState,
    VideoDeleteOperation, UserColor, ImageItem, ImageQueue, ImageQueueDeleteOperation,
};
use crate::dioxus_structs::{ImageData as CustomImageData, VideoData as CustomVideoData};
use crate::utils::*;
use base64::{Engine as _, engine::general_purpose};
use dioxus::prelude::*;
use eframe::egui;
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio;
use crate::core::bytes::bytes::Bytes;
use crate::core::Writer;

// 应用状态枚举
#[derive(Clone, Debug, PartialEq)]
pub enum AppState {
    DomainInput,
    MainApp,
}

// Dioxus应用状态结构
#[derive(Clone, Debug)]
pub struct DioxusAppState {
    pub current_color: egui::Color32,
    pub draw_mode: DrawMode,
    pub stroke_width: f32,
    pub canvas_width: u32,
    pub canvas_height: u32,
    pub chat_input: String,
    pub danmaku_enabled: bool,
    pub current_image: Option<String>, // 当前显示的图片用户名（作为图片ID）
    pub current_video: Option<String>, // 当前显示的视频用户名（作为视频ID）
    pub user_colors: HashMap<String, egui::Color32>, // 用户颜色映射
    pub image_queue: Vec<crate::dioxus_structs::ImageItem>, // 本地图片队列
    pub current_image_index: usize, // 当前显示的图片索引
    pub app_state: AppState, // 应用当前状态
    pub domain_id: Option<u32>, // DDS域号
    pub domain_input: String, // 域号输入字段
    pub private_chat_enabled: bool, // 私聊模式开关
    pub selected_user: Option<String>, // 选中的私聊用户
}

#[derive(Clone, Debug, PartialEq)]
pub enum DrawMode {
    Mouse,
    Draw,
    Erase,
}

impl Default for DioxusAppState {
    fn default() -> Self {
        Self {
            current_color: egui::Color32::from_rgb(255, 0, 0),
            draw_mode: DrawMode::Mouse,
            stroke_width: 2.0,
            canvas_width: 1000,
            canvas_height: 600,
            chat_input: String::new(),
            danmaku_enabled: true,
            current_image: None,
            current_video: None,
            user_colors: HashMap::new(),
            image_queue: Vec::new(),
            current_image_index: 0,
            app_state: AppState::DomainInput,
            domain_id: None,
            domain_input: String::new(),
            private_chat_enabled: false,
            selected_user: None,
        }
    }
}

// 使用dioxus_structs.rs中的数据结构
// 为DanmakuMessage添加额外字段的扩展结构
#[derive(Clone, Debug)]
pub struct DioxusDanmakuMessage {
    pub username: String,
    pub message: String,
    pub x: f32,
    pub y: f32,
    pub color: egui::Color32,
    pub speed: f32,
    pub start_time: f64,
    pub id: String,
}

// 主应用组件Props
#[derive(Clone)]
pub struct DioxusAppProps {
    pub domain_id: u32,
    pub received: Arc<Mutex<HashMap<String, MouseState>>>,
    pub received_images: Arc<Mutex<HashMap<String, CustomImageData>>>,
    pub received_videos: Arc<Mutex<HashMap<String, CustomVideoData>>>,
    pub received_strokes: Arc<Mutex<Vec<DrawStroke>>>,
    pub received_erases: Arc<Mutex<Vec<EraseOperation>>>,
    pub received_image_deletes: Arc<Mutex<Vec<ImageDeleteOperation>>>,
    pub received_video_deletes: Arc<Mutex<Vec<VideoDeleteOperation>>>,
    pub received_chat_messages: Arc<Mutex<Vec<ChatMessage>>>,
    pub received_danmaku_messages: Arc<Mutex<Vec<crate::dioxus_structs::DanmakuMessage>>>,
    pub received_user_colors: Arc<Mutex<HashMap<String, UserColor>>>,
    pub received_image_queues: Arc<Mutex<HashMap<String, ImageQueue>>>,
    pub received_image_queue_deletes: Arc<Mutex<Vec<ImageQueueDeleteOperation>>>,
    pub writer: Arc<Mutex<Writer>>,
    pub video_writer: Arc<Mutex<Writer>>,
    pub draw_writer: Arc<Mutex<Writer>>,
    pub erase_writer: Arc<Mutex<Writer>>,
    pub image_delete_writer: Arc<Mutex<Writer>>,
    pub video_delete_writer: Arc<Mutex<Writer>>,
    pub image_queue_writer: Arc<Mutex<Writer>>,
    pub image_queue_delete_writer: Arc<Mutex<Writer>>,
    pub chat_writer: Arc<Mutex<Writer>>,
    pub danmaku_writer: Arc<Mutex<Writer>>,
    pub color_writer: Arc<Mutex<Writer>>,
}

impl PartialEq for DioxusAppProps {
    fn eq(&self, other: &Self) -> bool {
        // 对于指针类型，我们比较地址
        self.domain_id == other.domain_id
            && Arc::ptr_eq(&self.received, &other.received)
            && Arc::ptr_eq(&self.received_images, &other.received_images)
            && Arc::ptr_eq(&self.received_videos, &other.received_videos)
            && Arc::ptr_eq(&self.received_strokes, &other.received_strokes)
            && Arc::ptr_eq(&self.received_erases, &other.received_erases)
            && Arc::ptr_eq(&self.received_image_deletes, &other.received_image_deletes)
            && Arc::ptr_eq(&self.received_video_deletes, &other.received_video_deletes)
            && Arc::ptr_eq(&self.received_chat_messages, &other.received_chat_messages)
            && Arc::ptr_eq(&self.writer, &other.writer)
            && Arc::ptr_eq(&self.video_writer, &other.video_writer)
            && Arc::ptr_eq(&self.draw_writer, &other.draw_writer)
            && Arc::ptr_eq(&self.erase_writer, &other.erase_writer)
            && Arc::ptr_eq(&self.image_delete_writer, &other.image_delete_writer)
            && Arc::ptr_eq(&self.video_delete_writer, &other.video_delete_writer)
            && Arc::ptr_eq(&self.image_queue_delete_writer, &other.image_queue_delete_writer)
            && Arc::ptr_eq(&self.chat_writer, &other.chat_writer)
    }
}

// 主应用组件
#[component]
pub fn DioxusApp(props: DioxusAppProps) -> Element {
    // 解构props
    let DioxusAppProps {
        domain_id,
        received,
        received_images,
        received_videos,
        received_strokes,
        received_erases,
        received_image_deletes,
        received_video_deletes,
        received_chat_messages,
        received_danmaku_messages,
        received_user_colors,
        received_image_queues,
        received_image_queue_deletes,
        writer,
        video_writer,
        draw_writer,
        erase_writer,
        image_delete_writer,
        video_delete_writer,
        image_queue_writer,
        image_queue_delete_writer: _,
        chat_writer,
        danmaku_writer: _,
        color_writer,
    } = props;

    // 应用状态
    let mut app_state = use_signal(|| {
        let mut state = DioxusAppState::default();
        // 设置域号
        state.domain_id = Some(domain_id);
        // 根据域号决定应用状态：域号为0时显示输入界面，否则显示主界面
        if domain_id == 0 {
            state.app_state = AppState::DomainInput;
        } else {
            state.app_state = AppState::MainApp;
        }
        // 初始化当前用户的颜色到user_colors映射中
        let username = get_username();
        state.user_colors.insert(username, state.current_color);
        state
    });
    let mut chat_messages = use_signal(|| Vec::<ChatMessage>::new());
    let mut danmaku_messages = use_signal(|| Vec::<DioxusDanmakuMessage>::new());
    let mut mouse_positions = use_signal(|| HashMap::<String, MouseState>::new());
    let mut images = use_signal(|| HashMap::<String, CustomImageData>::new());
    let mut videos = use_signal(|| HashMap::<String, CustomVideoData>::new());
    let mut strokes = use_signal(|| Vec::<DrawStroke>::new());
    let mut local_strokes = use_signal(|| Vec::<DrawStroke>::new());
    let is_drawing = use_signal(|| false);
    let last_mouse_pos = use_signal(|| (0.0f32, 0.0f32));

    // 定期更新数据（从DDS接收数据）
    use_future(move || {
        let received = received.clone();
        let received_images = received_images.clone();
        let received_videos = received_videos.clone();
        let received_strokes = received_strokes.clone();
        let received_erases = received_erases.clone();
        let received_image_deletes = received_image_deletes.clone();
        let received_video_deletes = received_video_deletes.clone();
        let received_chat_messages = received_chat_messages.clone();
        let received_danmaku_messages: Arc<Mutex<Vec<crate::dioxus_structs::DanmakuMessage>>> = received_danmaku_messages.clone();
        let received_user_colors = received_user_colors.clone();
        let received_image_queues = received_image_queues.clone();
        let received_image_queue_deletes = received_image_queue_deletes.clone();

        async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_millis(16)).await;

                // 处理图片删除操作（✅ drain）
                {
                    let mut q = received_image_deletes.lock().unwrap();
                    for delete_op in q.drain(..) {
                        println!(
                            "接收到图片删除操作: username={}, image_id={}",
                            delete_op.username, delete_op.image_id
                        );

                        // 1) 先从当前渲染态移除
                        {
                            let mut imgs = images.write();
                            let removed = imgs.remove(&delete_op.image_id).is_some();
                            println!("从 images 移除: {}", removed);
                        }

                        // 2) 同步把“来源快照”也移除，防止下一帧又同步回来
                        {
                            let mut snapshot = received_images.lock().unwrap();
                            snapshot.remove(&delete_op.image_id);
                        }
                    }
                }

                // 处理视频删除操作
                {
                    let mut q = received_video_deletes.lock().unwrap();
                    for delete_op in q.drain(..) {
                        videos.write().remove(&delete_op.video_id);
                        received_videos.lock().unwrap().remove(&delete_op.video_id);
                    }
                }

                // 处理图片队列删除操作
                {
                    let mut q = received_image_queue_deletes.lock().unwrap();
                    for delete_op in q.drain(..) {
                        println!(
                            "接收到图片队列删除操作: username={}",
                            delete_op.username
                        );
                        
                        // 清空本地图片队列（如果是其他用户的删除操作）
                        if delete_op.username != get_username() {
                            app_state.write().image_queue.clear();
                            app_state.write().current_image_index = 0;
                            println!("清空本地图片队列");
                        }
                    }
                }

                // 更新弹幕位置
                update_danmaku_positions(&mut danmaku_messages);

                // 更新鼠标位置
                {
                    let received_data = received.lock().unwrap();
                    let mut positions = mouse_positions.write();
                    positions.clear();
                    for (username, mouse_state) in received_data.iter() {
                        positions.insert(
                            username.clone(),
                            MouseState {
                                username: mouse_state.username.clone(),
                                color: mouse_state.color,
                                x: mouse_state.x,
                                y: mouse_state.y,
                            },
                        );
                    }
                }

                // 更新图片
                {
                    let received_data = received_images.lock().unwrap();
                    let mut imgs = images.write();
                    imgs.clear();
                    for (id, image_data) in received_data.iter() {
                        imgs.insert(
                            id.clone(),
                            CustomImageData {
                                username: image_data.username.clone(),
                                image_data: image_data.image_data.clone(),
                                width: image_data.width,
                                height: image_data.height,
                            },
                        );
                    }
                }

                // 更新视频
                {
                    let received_data = received_videos.lock().unwrap();
                    let mut vids = videos.write();
                    vids.clear();
                    for (id, video_data) in received_data.iter() {
                        vids.insert(
                            id.clone(),
                            CustomVideoData {
                                username: video_data.username.clone(),
                                video_data: video_data.video_data.clone(),
                                file_name: video_data.file_name.clone(),
                                file_size: video_data.file_size,
                            },
                        );
                    }
                }

                // 更新笔迹
                {
                    let received_data = received_strokes.lock().unwrap();
                    let mut strks = strokes.write();
                    strks.clear();
                    for stroke in received_data.iter() {
                        strks.push(DrawStroke {
                            username: stroke.username.clone(),
                            color: stroke.color,
                            start_x: stroke.start_x,
                            start_y: stroke.start_y,
                            end_x: stroke.end_x,
                            end_y: stroke.end_y,
                            stroke_width: stroke.stroke_width,
                            timestamp: stroke.timestamp,
                        });
                    }
                }

                // 处理擦除操作
                {
                    let received_data = received_erases.lock().unwrap();
                    for erase_op in received_data.iter() {
                        // 对本地笔迹和接收到的笔迹都应用擦除操作（只擦除擦除时刻之前的笔迹）
                        local_strokes.write().retain(|stroke| {
                            !(stroke.timestamp < erase_op.timestamp
                                && line_intersects_circle(
                                    stroke.start_x,
                                    stroke.start_y,
                                    stroke.end_x,
                                    stroke.end_y,
                                    erase_op.x,
                                    erase_op.y,
                                    erase_op.radius,
                                ))
                        });

                        strokes.write().retain(|stroke| {
                            !(stroke.timestamp < erase_op.timestamp
                                && line_intersects_circle(
                                    stroke.start_x,
                                    stroke.start_y,
                                    stroke.end_x,
                                    stroke.end_y,
                                    erase_op.x,
                                    erase_op.y,
                                    erase_op.radius,
                                ))
                        });
                    }
                }

                // 更新聊天消息
                {
                    let received_data = received_chat_messages.lock().unwrap();
                    let current_count = received_data.len();
                    let chat_count = chat_messages.read().len();

                    if current_count > chat_count {
                        let new_messages = &received_data[chat_count..];
                        for message in new_messages {
                            let chat_msg = ChatMessage {
                                username: message.username.clone(),
                                message: message.message.clone(),
                                timestamp: message.timestamp.clone(),
                                color: message.color,
                            };
                            chat_messages.write().push(chat_msg);

                            // 弹幕消息已由DDS处理器自动创建，无需在此处重复添加
                        }
                    }
                }

                // 更新弹幕消息
                {
                    let received_data = received_danmaku_messages.lock().unwrap();
                    let current_count = received_data.len();
                    let danmaku_count = danmaku_messages.read().len();

                    if current_count > danmaku_count {
                        let new_messages = &received_data[danmaku_count..];
                        for message in new_messages {
                            // 检查是否已存在相同ID的弹幕，避免重复添加
                            let existing_ids: Vec<String> = danmaku_messages
                                .read()
                                .iter()
                                .map(|m| m.id.clone())
                                .collect();
                            if !existing_ids.contains(&message.id) {
                                let danmaku_msg = DioxusDanmakuMessage {
                                    username: message.username.clone(),
                                    message: message.message.clone(),
                                    x: message.x,
                                    y: message.y,
                                    color: message.color,
                                    speed: message.speed,
                                    start_time: message.start_time,
                                    id: message.id.clone(),
                                };
                                danmaku_messages.write().push(danmaku_msg);
                            }
                        }
                    }
                }

                // 处理用户颜色更新
                {
                    let mut color_map = received_user_colors.lock().unwrap();
                    for (username, user_color) in color_map.drain() {
                        println!("接收到用户颜色更新: username={}, color={:?}", username, user_color.color);
                        
                        // 更新app_state中的用户颜色映射
                        app_state.write().user_colors.insert(username, user_color.color);
                    }
                }

                // 处理图片队列更新
                {
                    let mut image_queue_map = received_image_queues.lock().unwrap();
                    for (username, image_queue) in image_queue_map.drain() {
                        println!("接收到图片队列更新: username={}, images_count={}", username, image_queue.images.len());
                        
                        // 更新app_state中的图片队列
                        app_state.write().image_queue = image_queue.images;
                        app_state.write().current_image_index = image_queue.current_index;
                    }
                }
            }
        }
    });

    rsx! {
        match app_state.read().app_state {
            AppState::DomainInput => rsx! {
                div {
                    "style": "display: flex; justify-content: center; align-items: center; height: 100vh; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);",
                    
                    div {
                        "style": "background: white; padding: 40px; border-radius: 10px; box-shadow: 0 10px 25px rgba(0,0,0,0.2); text-align: center; min-width: 400px;",
                        
                        h1 {
                            "style": "color: #333; margin-bottom: 30px; font-size: 28px;",
                            "设置DDS域号"
                        }
                        
                        p {
                            "style": "color: #666; margin-bottom: 20px; font-size: 16px;",
                            "请输入1-150之间的域号"
                        }
                        
                        input {
                            "type": "number",
                            "min": "1",
                            "max": "150",
                            "placeholder": "输入域号 (1-150)",
                            "style": "width: 100%; padding: 12px; border: 2px solid #ddd; border-radius: 5px; font-size: 16px; margin-bottom: 20px; text-align: center;",
                            "value": "{app_state.read().domain_input}",
                            oninput: move |evt| {
                                app_state.write().domain_input = evt.value();
                            }
                        }
                        
                        button {
                            "style": "background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; border: none; padding: 12px 30px; border-radius: 5px; font-size: 16px; cursor: pointer; transition: transform 0.2s;",
                            "onmouseover": "this.style.transform='scale(1.05)'",
                            "onmouseout": "this.style.transform='scale(1)'",
                            onclick: move |_| {
                                let binding = app_state.read();
                                let input_value = binding.domain_input.trim();
                                if let Ok(domain) = input_value.parse::<u32>() {
                                    if domain >= 1 && domain <= 150 {
                                        drop(binding);
                                        
                                        // 重新启动应用并传递新的域号
                                        let current_exe = std::env::current_exe().unwrap();
                                        std::process::Command::new(current_exe)
                                            .arg(domain.to_string())
                                            .spawn()
                                            .expect("Failed to restart application");
                                        
                                        // 退出当前应用实例
                                        std::process::exit(0);
                                    }
                                }
                            },
                            "确认"
                        }
                    }
                }
            },
            AppState::MainApp => rsx! {
                div {
                    "style": "display: flex; height: 100vh; font-family: Arial, sans-serif; position: relative;",

                    // 右上角控制区域
                    div {
                        "style": "position: absolute; top: 10px; right: 10px; display: flex; align-items: center; gap: 10px; z-index: 1000;",
                        
                        // 域号显示
                        div {
                            "style": "background: rgba(0, 0, 0, 0.7); color: white; padding: 8px 12px; border-radius: 5px; font-size: 14px;",
                            "DDS域号: {app_state.read().domain_id.unwrap_or(0)}"
                        }
                        
                        // 返回域号输入界面按钮
                        button {
                            "style": "background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; border: none; padding: 8px 16px; border-radius: 5px; font-size: 14px; cursor: pointer; transition: transform 0.2s;",
                            "onmouseover": "this.style.transform='scale(1.05)'",
                            "onmouseout": "this.style.transform='scale(1)'",
                            onclick: move |_| {
                                // 切换到域号输入界面
                                app_state.write().app_state = AppState::DomainInput;
                                app_state.write().domain_input = String::new();
                            },
                            "切换域号"
                        }
                    }

                    // 主要内容区域
                    div {
                        "style": "flex: 1; display: flex;",

                        // 中央面板
                        CentralPanel {
                            app_state,
                            mouse_positions,
                            images,
                            videos,
                            strokes,
                            local_strokes,
                            danmaku_messages,
                            is_drawing,
                            last_mouse_pos,
                            writer: writer.clone(),
                            video_writer: video_writer.clone(),
                            draw_writer: draw_writer.clone(),
                            erase_writer: erase_writer.clone(),
                            image_delete_writer: image_delete_writer.clone(),
                            video_delete_writer: video_delete_writer.clone(),
                            image_queue_writer: image_queue_writer.clone(),
                            image_queue_delete_writer: props.image_queue_delete_writer.clone(),
                            color_writer: color_writer.clone(),
                        }

                // 右侧聊天面板
                ChatPanel {
                    chat_messages,
                    app_state,
                    chat_writer: chat_writer.clone(),
                    writer: writer.clone(),
                    danmaku_messages,
                    mouse_positions,
                }
            }

            // 弹幕层
            DanmakuOverlay {
                danmaku_messages: danmaku_messages
            }

            // 全局鼠标位置显示层
            GlobalMouseOverlay {
                mouse_positions: mouse_positions,
                app_state: app_state
            }
        }
            }
        }
    }
}

// 中央面板组件Props
#[derive(Props, Clone)]
struct CentralPanelProps {
    app_state: Signal<DioxusAppState>,
    mouse_positions: Signal<HashMap<String, MouseState>>,
    images: Signal<HashMap<String, CustomImageData>>,
    videos: Signal<HashMap<String, CustomVideoData>>,
    strokes: Signal<Vec<DrawStroke>>,
    local_strokes: Signal<Vec<DrawStroke>>,
    danmaku_messages: Signal<Vec<DioxusDanmakuMessage>>,
    is_drawing: Signal<bool>,
    last_mouse_pos: Signal<(f32, f32)>,
    writer: Arc<Mutex<Writer>>,
    video_writer: Arc<Mutex<Writer>>,
    draw_writer: Arc<Mutex<Writer>>,
    erase_writer: Arc<Mutex<Writer>>,
    image_delete_writer: Arc<Mutex<Writer>>,
    video_delete_writer: Arc<Mutex<Writer>>,
    image_queue_writer: Arc<Mutex<Writer>>,
    image_queue_delete_writer: Arc<Mutex<Writer>>,
    color_writer: Arc<Mutex<Writer>>,
}

impl PartialEq for CentralPanelProps {
    fn eq(&self, other: &Self) -> bool {
        // 对于Signal类型，我们比较它们的值
        self.app_state == other.app_state &&
        self.mouse_positions == other.mouse_positions &&
        self.images == other.images &&
        self.videos == other.videos &&
        self.strokes == other.strokes &&
        self.local_strokes == other.local_strokes &&
        self.danmaku_messages == other.danmaku_messages &&
        self.is_drawing == other.is_drawing &&
        self.last_mouse_pos == other.last_mouse_pos &&
        // 对于Arc指针，我们比较地址
        Arc::ptr_eq(&self.writer, &other.writer) &&
        Arc::ptr_eq(&self.video_writer, &other.video_writer) &&
        Arc::ptr_eq(&self.draw_writer, &other.draw_writer) &&
        Arc::ptr_eq(&self.erase_writer, &other.erase_writer) &&
        Arc::ptr_eq(&self.image_delete_writer, &other.image_delete_writer) &&
        Arc::ptr_eq(&self.video_delete_writer, &other.video_delete_writer) &&
        Arc::ptr_eq(&self.image_queue_delete_writer, &other.image_queue_delete_writer) &&
        Arc::ptr_eq(&self.color_writer, &other.color_writer)
    }
}

// 中央面板组件
#[component]
fn CentralPanel(props: CentralPanelProps) -> Element {
    // 解构props
    let CentralPanelProps {
        mut app_state,
        mouse_positions,
        images,
        videos,
        strokes,
        local_strokes,
        danmaku_messages,
        is_drawing,
        last_mouse_pos,
        writer,
        video_writer,
        draw_writer,
        erase_writer,
        image_delete_writer,
        video_delete_writer,
        image_queue_writer,
        image_queue_delete_writer: _,
        color_writer,
    } = props;

    rsx! {
            div {
                "style": "flex: 1; padding: 20px; background: white; display: flex; flex-direction: column;",
                onmousemove: {
                    let writer_clone = writer.clone();
                    move |evt: MouseEvent| {
                        let rect = evt.page_coordinates();
                        let state = app_state.read();
                        send_mouse_position(rect.x as f32, rect.y as f32, state.current_color, writer_clone.clone());
                    }
                },

                // 工具栏
                div {
                    "style": "background: #f8f9fa; padding: 15px; border-radius: 8px; margin-bottom: 20px;
                     margin-right:20px;                    
                     box-shadow: 0 2px 4px rgba(0,0,0,0.1);
                     display:flex",

                    // h3 {
                    //     "style": "margin: 0 0 15px 0; color: #333;",
                    //     "绘图工具"
                    // }

                    // —— 颜色选择器（受控组件） ——
                    {
                        let cur = app_state.read().current_color; // 读一次
                        let cur_hex = format!("#{:02x}{:02x}{:02x}", cur.r(), cur.g(), cur.b());
                        let rgb_str = format!("RGB({}, {}, {})", cur.r(), cur.g(), cur.b());

                        rsx! {
                            div {
                                "style": "align-items:center; gap:10px;margin:20px;margin-bottom:20px;",

                                label { 
                                    "style": "font-weight: 500; color: #555; display: block; margin-bottom: 8px;",
                                    "画笔颜色:" }
                                // HoverCardUse {  }

                                input {
                                    r#type: "color",
                                    value: cur_hex,
                                    oninput: {
                                        let color_writer_clone = color_writer.clone();
                                        move |evt: FormEvent| {
                                            let s = evt.value();
                                            log::info!("color input -> {}", &s);
                                            if let Ok(c) = parse_color(&s) {
                                                // 更新当前用户的颜色
                                                let username = get_username();
                                                let mut state = app_state.write();
                                                state.current_color = c;
                                                // 同时更新用户颜色映射，确保本地鼠标指针颜色立即更新
                                                state.user_colors.insert(username, c);
                                                drop(state);
                                                // 发送颜色更新的DDS消息
                                                send_user_color(c, color_writer_clone.clone());
                                            } else {
                                                log::warn!("bad color: {}", s);
                                            }
                                        }
                                    }
                                }

                                // 这里把已经 format! 好的字符串作为文本
                                span { "{rgb_str}" }
                            }
                        }
                    }

                    // 模式切换按钮
                    div {
                        "style": "align-items:center; gap:10px;margin:20px;margin-bottom:20px;",
                        label {
                            "style": "font-weight: 500; color: #555; display: block; margin-bottom: 8px;",
                            "绘图模式:"
                        }
                        div {
                            "style": "display: flex; gap: 8px;",
                            button {
                                class: "button",
                                "data-style": "primary",
                                "style": if app_state.read().draw_mode == DrawMode::Mouse {
                                    "padding: 8px 16px; background: #007bff; color: white; border: none; border-radius: 6px; cursor: pointer; font-size: 14px; transition: all 0.2s; box-shadow: 0 2px 4px rgba(0,123,255,0.3);"
                                } else {
                                    "padding:  background: white; color: #333; border: 2px solid #ddd; border-radius: 6px; cursor: pointer; font-size: 14px; transition: all 0.2s;"
                                },
                                onclick: move |_| {
                                    let mut state = app_state.write();
                                    state.draw_mode = DrawMode::Mouse;
                                },
                                "鼠标"
                            }
                            button {
                                "style": if app_state.read().draw_mode == DrawMode::Draw {
                                    "padding: 8px 16px; background: #28a745; color: white; border: none; border-radius: 6px; cursor: pointer; font-size: 14px; transition: all 0.2s; box-shadow: 0 2px 4px rgba(40,167,69,0.3);"
                                } else {
                                    "padding:  background: white; color: #333; border: 2px solid #ddd; border-radius: 6px; cursor: pointer; font-size: 14px; transition: all 0.2s;"
                                },
                                onclick: move |_| {
                                    let mut state = app_state.write();
                                    state.draw_mode = DrawMode::Draw;
                                },
                                "画笔"
                            }
                            button {
                                "style": if app_state.read().draw_mode == DrawMode::Erase {
                                    "padding: 8px 16px; background: #dc3545; color: white; border: none; border-radius: 6px; cursor: pointer; font-size: 14px; transition: all 0.2s; box-shadow: 0 2px 4px rgba(220,53,69,0.3);"
                                } else {
                                    "padding: background: white; color: #333; border: 2px solid #ddd; border-radius: 6px; cursor: pointer; font-size: 14px; transition: all 0.2s;"
                                },
                                onclick: move |_| {
                                    let mut state = app_state.write();
                                    state.draw_mode = DrawMode::Erase;
                                },
                                "擦除"
                            }
                        }
                    }

                    // 媒体上传按钮
                    div {
                        "style": "align-items:center; gap:10px;margin:20px;margin-bottom:20px;",
                        label {
                            "style": "font-weight: 500; color: #555; display: block; margin-bottom: 8px;",
                            "媒体上传:"
                        }
                        div {
                            "style": "display: flex; gap: 8px; flex-wrap: wrap;",
                            {
                                let has_media = !images.read().is_empty() || !videos.read().is_empty();
                                let has_queue = !app_state.read().image_queue.is_empty();
                                
                                let queue_button_style = if has_media || has_queue {
                                    "padding: 8px 16px; background: #6c757d; color: white; border: none; border-radius: 6px; cursor: not-allowed; font-size: 14px; transition: all 0.2s; opacity: 0.6;"
                                } else {
                                    "padding: 8px 16px; background: #28a745; color: white; border: none; border-radius: 6px; cursor: pointer; font-size: 14px; transition: all 0.2s;"
                                };
                                
                                let video_button_style = if has_media || has_queue {
                                    "padding: 8px 16px; background: #6c757d; color: white; border: none; border-radius: 6px; cursor: not-allowed; font-size: 14px; transition: all 0.2s; opacity: 0.6;"
                                } else {
                                    "padding: 8px 16px; background: #6f42c1; color: white; border: none; border-radius: 6px; cursor: pointer; font-size: 14px; transition: all 0.2s;"
                                };

                                rsx! {
                                    button {
                                        "style": queue_button_style,
                                        disabled: has_media||has_queue,
                                        onclick: {
                                        let image_queue_writer = image_queue_writer.clone();
                                        move |_| {
                                            upload_images_to_queue(app_state, image_queue_writer.clone());
                                        }
                                    },
                                        "📁 上传图片"
                                    }
                                    button {
                                        "style": video_button_style,
                                        disabled: has_media||has_queue,
                                        onclick: move |_| {
                                            upload_video(video_writer.clone());
                                        },
                                        "🎥 上传视频"
                                    }
                                }
                            }
                        }
                    }

                    div {  
                        "style": "align-items:center; gap:10px;margin:20px;margin-bottom:20px;",
                        label {
                            "style": "font-weight: 500; color: #555; display: block; margin-bottom: 8px;",
                            "屏幕共享:"
                        }
                        ToggleSwitch{}
                    }
                    // ToggleSwitch{}
                }


                // 画布区域
                Canvas {
                    app_state,
                    mouse_positions,
                    images,
                    videos,
                    strokes,
                    local_strokes,
                    danmaku_messages,
                    is_drawing,
                    last_mouse_pos,
                    writer: writer.clone(),
                    draw_writer,
                    erase_writer,
                    image_delete_writer,
                    video_delete_writer,
                    image_queue_delete_writer: props.image_queue_delete_writer.clone(),
                }
            }
        }
}

// 画布组件Props
#[derive(Props, Clone)]
struct CanvasProps {
    app_state: Signal<DioxusAppState>,
    mouse_positions: Signal<HashMap<String, MouseState>>,
    images: Signal<HashMap<String, CustomImageData>>,
    videos: Signal<HashMap<String, CustomVideoData>>,
    strokes: Signal<Vec<DrawStroke>>,
    local_strokes: Signal<Vec<DrawStroke>>,
    danmaku_messages: Signal<Vec<DioxusDanmakuMessage>>,
    is_drawing: Signal<bool>,
    last_mouse_pos: Signal<(f32, f32)>,
    writer: Arc<Mutex<Writer>>,
    draw_writer: Arc<Mutex<Writer>>,
    erase_writer: Arc<Mutex<Writer>>,
    image_delete_writer: Arc<Mutex<Writer>>,
    video_delete_writer: Arc<Mutex<Writer>>,
    image_queue_delete_writer: Arc<Mutex<Writer>>,
}

impl PartialEq for CanvasProps {
    fn eq(&self, other: &Self) -> bool {
        // 对于Signal类型，我们比较它们的值
        self.app_state == other.app_state &&
        self.mouse_positions == other.mouse_positions &&
        self.images == other.images &&
        self.videos == other.videos &&
        self.strokes == other.strokes &&
        self.local_strokes == other.local_strokes &&
        self.danmaku_messages == other.danmaku_messages &&
        self.is_drawing == other.is_drawing &&
        self.last_mouse_pos == other.last_mouse_pos &&
        // 对于Arc指针，我们比较地址
        Arc::ptr_eq(&self.writer, &other.writer) &&
        Arc::ptr_eq(&self.draw_writer, &other.draw_writer) &&
        Arc::ptr_eq(&self.erase_writer, &other.erase_writer) &&
        Arc::ptr_eq(&self.image_delete_writer, &other.image_delete_writer) &&
        Arc::ptr_eq(&self.video_delete_writer, &other.video_delete_writer) &&
        Arc::ptr_eq(&self.image_queue_delete_writer, &other.image_queue_delete_writer)
    }
}

// 画布组件
#[component]
fn Canvas(props: CanvasProps) -> Element {
    // 解构props
    let CanvasProps {
        app_state,
        mouse_positions: _,
        images,
        videos,
        mut strokes,
        mut local_strokes,
        danmaku_messages: _,
        mut is_drawing,
        mut last_mouse_pos,
        writer,
        draw_writer,
        erase_writer,
        image_delete_writer,
        video_delete_writer,
        image_queue_delete_writer: _,
    } = props;
    let state = app_state.read();

    rsx! {
        div {
            "style": "flex: 1; 
                    position: relative; 
                    border: 2px solid #ccc;     
                    border-radius: 12px;         
                    background-color: white; 
                    overflow: hidden; 
                    margin-right: 20px;
                    box-shadow: 0 2px 6px rgba(0,0,0,0.1);
                    transition: all 0.2s;"
                    ,
            width: "full",
            height: "{state.canvas_height}px",

            // SVG画布用于绘制
            svg {
                "style": "position: absolute; top: 0; left: 0; pointer-events: none; z-index: 2;",
                width: "full",
                height: "{state.canvas_height}",

                // 渲染远程笔迹
                for stroke in strokes.read().iter() {
                    line {
                        x1: "{stroke.start_x}",
                        y1: "{stroke.start_y}",
                        x2: "{stroke.end_x}",
                        y2: "{stroke.end_y}",
                        stroke: format!("#{:02x}{:02x}{:02x}", stroke.color.r(), stroke.color.g(), stroke.color.b()),
                        stroke_width: "{stroke.stroke_width}",
                        stroke_linecap: "round"
                    }
                }

                // 渲染本地笔迹
                for stroke in local_strokes.read().iter() {
                    line {
                        x1: "{stroke.start_x}",
                        y1: "{stroke.start_y}",
                        x2: "{stroke.end_x}",
                        y2: "{stroke.end_y}",
                        stroke: format!("#{:02x}{:02x}{:02x}", stroke.color.r(), stroke.color.g(), stroke.color.b()),
                        stroke_width: "{stroke.stroke_width}",
                        stroke_linecap: "round"
                    }
                }

                // 鼠标位置渲染已移至GlobalMouseOverlay组件，在整个应用区域显示
            }

            // 背景图片层 - 显示图片队列
            {
                let state_read = app_state.read();
                if !state_read.image_queue.is_empty() && state_read.current_image_index < state_read.image_queue.len() {
                    let current_image = &state_read.image_queue[state_read.current_image_index];
                    let current_image_clone = current_image.clone();
                    let queue_len = state_read.image_queue.len();
                    let current_index = state_read.current_image_index;
                    drop(state_read);
                    
                    rsx! {
                        div {
                            "style": "position: absolute; top: 0; left: 0; width: 100%; height: 100%;",

                            // 背景图片
                            img {
                                src: format!("data:image/png;base64,{}", general_purpose::STANDARD.encode(&current_image_clone.image_data)),
                                "style": "width: 100%; height: 100%; object-fit: contain; opacity: 0.8;"
                            }
                        }

                        // 图片控制面板
                        div {
                            "style": "position: absolute; top: 10px; right: 10px; background: rgba(255,255,255,0.95); border: 1px solid #333; border-radius: 5px; padding: 8px; z-index: 9999; box-shadow: 0 2px 8px rgba(0,0,0,0.3);",

                            div {
                                "style": "display: flex; align-items: center; gap: 8px; font-size: 12px;",
                                span { "图片 {current_index + 1}/{queue_len}: {current_image_clone.id}" }
                                
                                // 左切换按钮
                                button {
                                    "style": "background-color: #007bff; color: white; border: none; border-radius: 3px; width: 24px; height: 24px; cursor: pointer; font-size: 12px; margin-right: 4px; display: flex; align-items: center; justify-content: center; font-weight: bold;",
                                    onclick: move |_| {
                                        switch_to_previous_image(app_state);
                                    },
                                    "‹"
                                }
                                
                                // 右切换按钮
                                button {
                                    "style": "background-color: #007bff; color: white; border: none; border-radius: 3px; width: 24px; height: 24px; cursor: pointer; font-size: 12px; margin-right: 4px; display: flex; align-items: center; justify-content: center; font-weight: bold;",
                                    onclick: move |_| {
                                        switch_to_next_image(app_state);
                                    },
                                    "›"
                                }
                                
                                // 删除队列按钮
                                button {
                                    "style": "background-color: #dc3545; color: white; border: none; border-radius: 3px; width: 24px; height: 24px; cursor: pointer; font-size: 12px; z-index: 2147483647; position: relative; display: flex; align-items: center; justify-content: center; font-weight: bold; pointer-events:auto;",
                                    onclick: move |_| {
                                        delete_image_queue(app_state, props.image_queue_delete_writer.clone());
                                    },
                                    "🗑"
                                }
                            }
                        }
                    }
                } else {
                    // 如果没有图片队列，显示远程图片
                    let images_guard = images.read();
                    if let Some((image_id, image_data)) = images_guard.iter().last() {
                        let image_id_clone = image_id.clone();
                        let image_data_clone = image_data.clone();
                        rsx! {
                            div {
                                "style": "position: absolute; top: 0; left: 0; width: 100%; height: 100%;",

                                // 背景图片
                                img {
                                    src: format!("data:image/png;base64,{}", general_purpose::STANDARD.encode(&image_data_clone.image_data)),
                                    "style": "width: 100%; height: 100%; object-fit: contain; opacity: 0.8;"
                                }
                            }

                            // 图片控制面板
                            div {
                                "style": "position: absolute; top: 10px; right: 10px; background: rgba(255,255,255,0.95); border: 1px solid #333; border-radius: 5px; padding: 8px; z-index: 9999; box-shadow: 0 2px 8px rgba(0,0,0,0.3);",

                                div {
                                    "style": "display: flex; align-items: center; gap: 8px; font-size: 12px;",
                                    span { "图片: {image_data_clone.username}" }
                                    button {
                                        "style": "background-color: #dc3545; color: white; border: none; border-radius: 3px; width: 24px; height: 24px; cursor: pointer; font-size: 12px; z-index: 2147483647; position: relative; display: flex; align-items: center; justify-content: center; font-weight: bold; pointer-events:auto;",
                                        onclick: move |_| {
                                            println!("删除按钮被点击!");
                                            delete_image(image_id_clone.clone(), image_delete_writer.clone());
                                        },
                                        "x"
                                    }
                                }
                            }
                        }
                    } else {
                        rsx! { div {} }
                    }
                }
            }

            // 视频显示
            {
                let videos_guard = videos.read();
                let images_guard = images.read();
                if images_guard.is_empty() {
                    if let Some((video_id, video_data)) = videos_guard.iter().last() {
                        let video_id_clone = video_id.clone();
                        let video_data_clone = video_data.clone();
                        rsx! {
                            div {
                                "style": "position: absolute; top: 10px; left: 10px; max-width: 500px; max-height: 400px; background: rgba(0, 0, 0, 0.8); border: 2px solid #444; border-radius: 8px; padding: 10px; z-index: 9999;",

                                // 视频控制面板
                                div {
                                    "style": "display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px; background: rgba(255, 255, 255, 0.9); padding: 5px 10px; border-radius: 4px;",
                                    span { 
                                        "style": "color: #333; font-weight: bold; font-size: 14px;",
                                        "用户: {video_data_clone.username}"
                                    }
                                    div {
                                        "style": "display: flex; gap: 5px;",
                                        span {
                                            "style": "color: #666; font-size: 12px;",
                                            "文件: {video_data_clone.file_name}"
                                        }
                                        button {
                                            "style": "background-color: #ff4444; color: white; border: none; border-radius: 50%; width: 24px; height: 24px; cursor: pointer; font-size: 16px; font-weight: bold; display: flex; align-items: center; justify-content: center; z-index: 10000; box-shadow: 0 2px 4px rgba(0,0,0,0.3);",
                                            onclick: move |_| {
                                                println!("视频删除按钮被点击!");
                                                delete_video(video_id_clone.clone(), video_delete_writer.clone());
                                            },
                                            "×"
                                        }
                                    }
                                }

                                // 视频播放器
                                video {
                                    controls: true,
                                    "style": "width: 100%; max-height: 350px; border-radius: 4px; background: #000;",
                                    src: format!("data:video/mp4;base64,{}", general_purpose::STANDARD.encode(&video_data_clone.video_data)),
                                    "Video not supported by your browser."
                                }
                            }
                        }
                    } else {
                        rsx! { div {} }
                    }
                } else {
                    rsx! { div {} }
                }
            }

            // 弹幕层已移至DanmakuOverlay组件，避免重复显示

            // 鼠标事件处理层
            div {
                "style": "position: absolute; top: 0; left: 0; width: 100%; height: 100%; cursor: crosshair; z-index: 3;",
                onmousemove: {
                    let writer_clone = writer.clone();
                    let draw_writer_clone = draw_writer.clone();
                    let erase_writer_clone = erase_writer.clone();
                    move |evt: MouseEvent| {
                        let canvas_rect = evt.element_coordinates();
                        let page_rect = evt.page_coordinates();
                        handle_mouse_move(
                            canvas_rect.x as f32,
                            canvas_rect.y as f32,
                            page_rect.x as f32,
                            page_rect.y as f32,
                            &app_state,
                            &mut is_drawing,
                            &mut last_mouse_pos,
                            &mut local_strokes,
                            &mut strokes,
                            writer_clone.clone(),
                            draw_writer_clone.clone(),
                            erase_writer_clone.clone()
                        );
                    }
                },
                onmousedown: {
                    let erase_writer_clone = erase_writer.clone();
                    move |evt: MouseEvent| {
                        let canvas_rect = evt.element_coordinates();
                        handle_mouse_down(
                            canvas_rect.x as f32,
                            canvas_rect.y as f32,
                            &app_state,
                            &mut is_drawing,
                            &mut last_mouse_pos,
                            &mut local_strokes,
                            &mut strokes,
                            erase_writer_clone.clone()
                        );
                    }
                },
                onmouseup: move |_| {
                    handle_mouse_up(&mut is_drawing);
                }
            }
        }
    }
}

// 聊天面板组件Props
#[derive(Props, Clone)]
struct ChatPanelProps {
    chat_messages: Signal<Vec<ChatMessage>>,
    app_state: Signal<DioxusAppState>,
    chat_writer: Arc<Mutex<Writer>>,
    writer: Arc<Mutex<Writer>>,
    danmaku_messages: Signal<Vec<DioxusDanmakuMessage>>,
    mouse_positions: Signal<HashMap<String, MouseState>>,
}

impl PartialEq for ChatPanelProps {
    fn eq(&self, other: &Self) -> bool {
        // 对于Signal类型，我们比较它们的值
        self.chat_messages == other.chat_messages &&
        self.app_state == other.app_state &&
        self.danmaku_messages == other.danmaku_messages &&
        // 对于Arc指针，我们比较地址
        Arc::ptr_eq(&self.chat_writer, &other.chat_writer) &&
        Arc::ptr_eq(&self.writer, &other.writer)
    }
}

// 聊天面板组件
#[component]
fn ChatPanel(props: ChatPanelProps) -> Element {
    // 解构props
    let ChatPanelProps {
        chat_messages,
        mut app_state,
        chat_writer,
        writer,
        danmaku_messages,
        mouse_positions,
    } = props;
    rsx! {
        div {
            "style": "width: 300px; display: flex; flex-direction: column; border-left: 1px solid #ccc; background-color: #f9f9f9;",
            onmousemove: {
                let writer_clone = writer.clone();
                move |evt: MouseEvent| {
                    let rect = evt.page_coordinates();
                    let state = app_state.read();
                    send_mouse_position(rect.x as f32, rect.y as f32, state.current_color, writer_clone.clone());
                }
            },

            // 聊天标题和弹幕开关
            div {
                "style": "padding: 10px; border-bottom: 1px solid #ccc; background-color: #e9ecef;",
                
                // 标题和域号显示
                div {
                    "style": "display: flex; justify-content: space-between; align-items: center; margin-bottom: 10px;",
                    h3 { "style": "margin: 0;", "聊天室" }
                    
                    // 域号显示
                    if let Some(domain_id) = app_state.read().domain_id {
                        div {
                            "style": "background-color: #007bff; color: white; padding: 4px 8px; border-radius: 4px; font-size: 12px; font-weight: bold;",
                            "域号: {domain_id}"
                        }
                    }
                }
                
                label {
                    "style": "display: flex; align-items: center; gap: 5px;",
                    input {
                        r#type: "checkbox",
                        checked: app_state.read().danmaku_enabled,
                        onchange: move |evt| {
                            let enabled = evt.checked();
                            app_state.write().danmaku_enabled = enabled;
                            // 同步更新全局弹幕开关状态
                            unsafe {
                                if let Some(ref global_enabled) = DANMAKU_ENABLED {
                                    if let Ok(mut enabled_guard) = global_enabled.lock() {
                                        *enabled_guard = enabled;
                                    }
                                }
                            }
                        }
                    }
                    "启用弹幕"
                }
            }

            // 消息列表
            div {
                "style": "flex: 1; overflow-y: auto; padding: 10px; max-height: 400px;",
                for message in chat_messages.read().iter() {
                    div {
                        "style": "margin-bottom: 8px; padding: 8px; background-color: white; border-radius: 6px; border: 1px solid #ddd; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                        div {
                            "style": "display: flex; justify-content: space-between; align-items: center; margin-bottom: 4px;",
                            span {
                                "style": "font-weight: bold; color: #007bff; font-size: 13px;",
                                "{message.username}"
                            }
                            span {
                                "style": "font-size: 11px; color: #666;",
                                "{message.timestamp}"
                            }
                        }
                        div {
                            "style": "color: #333; line-height: 1.4; font-size: 14px;",
                            "{message.message}"
                        }
                    }
                }
            }

            // 输入区域
            div {
                "style": "padding: 10px; border-top: 1px solid #ccc;",
                div {
                    "style": "display: flex; gap: 5px;",
                    input {
                        "style": "flex: 1; padding: 8px; border: 1px solid #ddd; border-radius: 4px; font-size: 14px; outline: none; transition: border-color 0.2s;",
                        r#type: "text",
                        placeholder: "输入消息...",
                        value: app_state.read().chat_input.clone(),
                        oninput: move |evt| {
                            app_state.write().chat_input = evt.value();
                        },
                        onkeypress: {
                            let chat_writer_clone = chat_writer.clone();
                            move |evt: KeyboardEvent| {
                                if evt.key() == Key::Enter {
                                    let message = app_state.read().chat_input.clone();
                                    if !message.trim().is_empty() {
                                        let _danmaku_clone = danmaku_messages.clone();
                                        let current_color = app_state.read().current_color;

                                        send_chat_message(message, current_color, chat_writer_clone.clone());

                                        let private_chat_enabled = app_state.read().private_chat_enabled;
                                        let selected_user = app_state.read().selected_user.clone();
                                        send_chat_message_with_private(message, current_color, chat_writer_clone.clone(), &mut danmaku_clone, app_state.read().danmaku_enabled, private_chat_enabled, selected_user);

                                        app_state.write().chat_input.clear();
                                    }
                                }
                            }
                        }
                    }

                    button {
                        class: "button",
                        "data-style": "primary",
                        "style": "padding: 8px 16px; background: #007bff; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 14px; transition: background-color 0.2s;",
                        onclick: {
                            let chat_writer_clone = chat_writer.clone();
                            move |_| {
                                let message = app_state.read().chat_input.clone();
                                if !message.trim().is_empty() {
                                    let _danmaku_clone = danmaku_messages.clone();
                                    let current_color = app_state.read().current_color;

                                    send_chat_message(message, current_color, chat_writer_clone.clone());

                                    let private_chat_enabled = app_state.read().private_chat_enabled;
                                    let selected_user = app_state.read().selected_user.clone();
                                    send_chat_message_with_private(message, current_color, chat_writer_clone.clone(), &mut danmaku_clone, app_state.read().danmaku_enabled, private_chat_enabled, selected_user);

                                    app_state.write().chat_input.clear();
                                }
                            }
                        },
                        "发送"
                    }
                }
                
                // 私聊控制区域
                div {
                    "style": "padding: 10px; border-top: 1px solid #eee; background-color: #f8f9fa;",
                    
                    // 私聊模式开关
                    div {
                        "style": "display: flex; align-items: center; gap: 8px; margin-bottom: 8px;",
                        
                        // 开关组件
                        div {
                            "style": format!(
                                "width: 40px; height: 20px; border-radius: 10px; background-color: {}; position: relative; cursor: pointer; transition: background-color 0.2s;",
                                if app_state.read().private_chat_enabled { "#007bff" } else { "#ccc" }
                            ),
                            onclick: move |_| {
                                let current = app_state.read().private_chat_enabled;
                                app_state.write().private_chat_enabled = !current;
                                if !current == false {
                                    app_state.write().selected_user = None;
                                }
                            },
                            
                            // 开关圆点
                            div {
                                "style": format!(
                                    "width: 16px; height: 16px; border-radius: 50%; background-color: white; position: absolute; top: 2px; left: {}; transition: left 0.2s;",
                                    if app_state.read().private_chat_enabled { "22px" } else { "2px" }
                                )
                            }
                        }
                        
                        span {
                            "style": "font-size: 14px; color: #333;",
                            "私聊模式"
                        }
                    }
                    
                    // 用户选择下拉框（仅在私聊模式开启时显示）
                    if app_state.read().private_chat_enabled {
                        div {
                            "style": "margin-top: 8px;",
                            
                            label {
                                "style": "display: block; font-size: 12px; color: #666; margin-bottom: 4px;",
                                "选择私聊用户:"
                            }
                            
                            select {
                                "style": "width: 100%; padding: 6px; border: 1px solid #ddd; border-radius: 4px; font-size: 14px; background-color: white;",
                                value: app_state.read().selected_user.clone().unwrap_or_default(),
                                onchange: move |evt| {
                                    let selected = evt.value();
                                    if selected.is_empty() {
                                        app_state.write().selected_user = None;
                                    } else {
                                        app_state.write().selected_user = Some(selected);
                                    }
                                },
                                
                                option {
                                    value: "",
                                    "请选择用户"
                                }
                                
                                // 显示在线用户列表（过滤掉自己）
                                for (username, _) in mouse_positions.read().iter() {
                                    if username != &get_username() {
                                        option {
                                            key: "{username}",
                                            value: "{username}",
                                            "{username}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// 弹幕组件
#[component]
fn DanmakuOverlay(danmaku_messages: Signal<Vec<DioxusDanmakuMessage>>) -> Element {
    rsx! {
        div {
            "style": "position: fixed; top: 0; left: 0; width: 100%; height: 100%; pointer-events: none; z-index: 1000; overflow: hidden;",

            for message in danmaku_messages.read().iter() {
                div {
                    key: "{message.id}",
                    "style": format!(
                        "position: absolute; left: {}px; top: {}px; color: rgb({},{},{}); font-size: 16px; font-weight: bold;  rgba(0,0,0,0.8); white-space: nowrap;
                        font-family:'KeinannPOP'
                        ",
                        message.x, message.y, message.color.r(), message.color.g(), message.color.b()
                    ),
                    "{message.username}: {message.message}"
                }
            }
        }
    }
}

// 全局鼠标位置显示组件
#[component]
fn GlobalMouseOverlay(
    mouse_positions: Signal<HashMap<String, MouseState>>,
    app_state: Signal<DioxusAppState>,
) -> Element {

    // let user_color = app_state.read().user_colors.get("20141")
    //                             .copied()
    //                             .unwrap_or(egui::Color32::from_rgb(255, 0, 0)); // 默认红色
    //                         log::info!("Rendering mouse pointer for  with color rgb({},{},{})", user_color.r(), user_color.g(), user_color.b());
    rsx! {
        
        div {
            "style": "position: fixed; top: 0; left: 0; width: 100%; height: 100%; pointer-events: none; z-index: 500; overflow: hidden;",

            for (username, mouse_state) in mouse_positions.read().iter() {
                div {
                    key: "{username}",
                    "style": format!(
                        "position: absolute; left: {}px; top: {}px; transform: translate(-50%, -50%);",
                        mouse_state.x, mouse_state.y
                    ),

                    // 鼠标圆点
                    div {
                        "style": {
                            //获取该用户的颜色，如果没有则使用默认颜色
                            let user_color = app_state.read().user_colors.get(username)
                                .copied()
                                .unwrap_or(egui::Color32::from_rgb(255, 0, 0)); // 默认红色
                            log::info!("Rendering mouse pointer for user {} with color rgb({},{},{})", username, user_color.r(), user_color.g(), user_color.b());
                            format!(
                                "width: 10px; height: 10px; border-radius: 50%; background-color: rgb({},{},{}); border: 2px solid white; box-shadow: 0 0 4px rgba(0,0,0,0.3);",
                                user_color.r(), user_color.g(), user_color.b()
                            )
                        }
                    }

                    // 用户名标签
                    div {
                        "style": "position: absolute; left: 15px; top: -15px; background-color: rgba(0,0,0,0.8); color: white; padding: 2px 6px; border-radius: 4px; font-size: 12px; white-space: nowrap;",
                        "{username}"
                    }
                }
            }
        }
    }
}

// 更稳健的解析：保留你的返回类型
fn parse_color(hex: &str) -> Result<egui::Color32, ()> {
    let s = hex.trim();
    if s.len() != 7 || !s.starts_with('#') {
        return Err(());
    }
    let r = u8::from_str_radix(&s[1..3], 16).map_err(|_| ())?;
    let g = u8::from_str_radix(&s[3..5], 16).map_err(|_| ())?;
    let b = u8::from_str_radix(&s[5..7], 16).map_err(|_| ())?;
    Ok(egui::Color32::from_rgb(r, g, b))
}

fn update_danmaku_positions(danmaku_messages: &mut Signal<Vec<DioxusDanmakuMessage>>) {
    let mut messages = danmaku_messages.write();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64();

    // 更新弹幕位置
    for message in messages.iter_mut() {
        let elapsed = now - message.start_time;
        // 从屏幕最右侧开始向左移动
        message.x = 1720.0 - (elapsed as f32 * message.speed);
    }

    // 移除超出屏幕或过期的弹幕
    messages.retain(|msg| msg.x > -100.0 && (now - msg.start_time) < 60.0);
}

fn add_danmaku_message(
    text: String,
    color: egui::Color32,
    danmaku_messages: &mut Signal<Vec<DioxusDanmakuMessage>>,
) {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64();
    let danmaku_id = format!("danmaku-{}", current_time);

    let danmaku_msg = DioxusDanmakuMessage {
        username: "用户".to_string(),
        message: text,
        x: 1200.0,                     // 从右侧开始
        y: rng.gen_range(50.0..600.0), // 随机Y位置
        color: color,                  // 使用传入的颜色
        speed: rng.gen_range(80.0..120.0),
        start_time: current_time,
        id: danmaku_id,
    };

    danmaku_messages.write().push(danmaku_msg);
}

fn send_chat_message(
    message: String,
    color: egui::Color32,
    chat_writer: Arc<Mutex<Writer>>,
) {
    if message.trim().is_empty() {
        return;
    }

    let chat_message = ChatMessage {
        username: get_username(),
        message: message.clone(),
        timestamp: get_current_timestamp(),
        color: color,
    };

    let json_message = json!({
        "type": "Chat",
        "username": chat_message.username,
        "message": chat_message.message,
        "timestamp": chat_message.timestamp,
        "color": {
            "r": color.r(),
            "g": color.g(),
            "b": color.b(),
            "a": color.a()
        }
    });

    send_dds_message(&json_message.to_string(), &chat_writer);

    // 不在这里添加弹幕，让DDS接收处理统一处理
    // 这样可以避免重复添加弹幕的问题
}

// 支持私聊的消息发送函数
fn send_chat_message_with_private(
    message: String,
    color: egui::Color32,
    chat_writer: Arc<Mutex<Writer>>,
    danmaku_messages: &mut Signal<Vec<DioxusDanmakuMessage>>,
    danmaku_enabled: bool,
    private_chat_enabled: bool,
    selected_user: Option<String>,
) {
    if message.trim().is_empty() {
        return;
    }

    let chat_message = ChatMessage {
        username: get_username(),
        message: message.clone(),
        timestamp: get_current_timestamp(),
        color: color,
    };

    let mut json_message = json!({
        "type": "Chat",
        "username": chat_message.username,
        "message": chat_message.message,
        "timestamp": chat_message.timestamp,
        "color": {
            "r": color.r(),
            "g": color.g(),
            "b": color.b(),
            "a": color.a()
        }
    });

    // 如果启用私聊模式且选择了用户，添加私聊标志
    if private_chat_enabled {
        if let Some(target_user) = selected_user {
            if !target_user.is_empty() {
                json_message["private_chat"] = json!({
                    "enabled": true,
                    "target_user": target_user
                });
            }
        }
    }

    send_dds_message(&json_message.to_string(), &chat_writer);

    // 不在这里添加弹幕，让DDS接收处理统一处理
    // 这样可以避免重复添加弹幕的问题
}

fn handle_mouse_move(
    canvas_x: f32,
    canvas_y: f32,
    page_x: f32,
    page_y: f32,
    app_state: &Signal<DioxusAppState>,
    is_drawing: &mut Signal<bool>,
    last_mouse_pos: &mut Signal<(f32, f32)>,
    local_strokes: &mut Signal<Vec<DrawStroke>>,
    strokes: &mut Signal<Vec<DrawStroke>>,
    writer: Arc<Mutex<Writer>>,
    draw_writer: Arc<Mutex<Writer>>,
    erase_writer: Arc<Mutex<Writer>>,
) {
    let state = app_state.read();
    let (last_x, last_y) = last_mouse_pos.read().clone();

    // 发送全局鼠标位置
    send_mouse_position(page_x, page_y, state.current_color, writer);

    if *is_drawing.read() {
        match state.draw_mode {
            DrawMode::Draw => {
                // 创建新的笔迹
                let stroke = DrawStroke {
                    username: get_username(),
                    color: state.current_color,
                    start_x: last_x,
                    start_y: last_y,
                    end_x: canvas_x,
                    end_y: canvas_y,
                    stroke_width: state.stroke_width,
                    timestamp: get_current_timestamp_millis(),
                };

                local_strokes.write().push(stroke.clone());
                send_draw_stroke(&stroke, draw_writer);
            }
            DrawMode::Erase => {
                // 拖动擦除
                handle_erase(canvas_x, canvas_y, local_strokes, strokes, erase_writer);
            }
            _ => {}
        }
    }

    last_mouse_pos.set((canvas_x, canvas_y));
}

fn handle_mouse_down(
    x: f32,
    y: f32,
    app_state: &Signal<DioxusAppState>,
    is_drawing: &mut Signal<bool>,
    last_mouse_pos: &mut Signal<(f32, f32)>,
    local_strokes: &mut Signal<Vec<DrawStroke>>,
    strokes: &mut Signal<Vec<DrawStroke>>,
    erase_writer: Arc<Mutex<Writer>>,
) {
    let state = app_state.read();

    match state.draw_mode {
        DrawMode::Draw => {
            is_drawing.set(true);
            last_mouse_pos.set((x, y));
        }
        DrawMode::Erase => {
            is_drawing.set(true);
            handle_erase(x, y, local_strokes, strokes, erase_writer);
        }
        DrawMode::Mouse => {
            // 鼠标模式不做特殊处理
        }
    }
}

fn handle_mouse_up(is_drawing: &mut Signal<bool>) {
    is_drawing.set(false);
}

fn handle_erase(
    x: f32,
    y: f32,
    local_strokes: &mut Signal<Vec<DrawStroke>>,
    strokes: &mut Signal<Vec<DrawStroke>>,
    erase_writer: Arc<Mutex<Writer>>,
) {
    let erase_radius = 30.0;
    let erase_timestamp = get_current_timestamp_millis();

    // 删除本地笔迹（只删除擦除时刻之前的笔迹）
    local_strokes.write().retain(|stroke| {
        !(stroke.timestamp < erase_timestamp
            && line_intersects_circle(
                stroke.start_x,
                stroke.start_y,
                stroke.end_x,
                stroke.end_y,
                x,
                y,
                erase_radius,
            ))
    });

    // 删除远程笔迹（只删除擦除时刻之前的笔迹）
    strokes.write().retain(|stroke| {
        !(stroke.timestamp < erase_timestamp
            && line_intersects_circle(
                stroke.start_x,
                stroke.start_y,
                stroke.end_x,
                stroke.end_y,
                x,
                y,
                erase_radius,
            ))
    });

    // 发送擦除操作
    let erase_operation = EraseOperation {
        username: get_username(),
        x,
        y,
        radius: erase_radius,
        timestamp: erase_timestamp,
    };

    let json_message = json!({
        "type": "Erase",
        "username": erase_operation.username,
        "x": erase_operation.x,
        "y": erase_operation.y,
        "radius": erase_operation.radius,
        "timestamp": erase_operation.timestamp
    });

    send_dds_message(&json_message.to_string(), &erase_writer);
}

fn send_mouse_position(
    x: f32,
    y: f32,
    color: egui::Color32,
    writer: Arc<Mutex<Writer>>,
) {
    let mouse_state = MouseState {
        username: get_username(),
        color,
        x,
        y,
    };

    let json_message = json!({
        "type": "Mouse",
        "username": mouse_state.username,
        "color": [mouse_state.color.r(), mouse_state.color.g(), mouse_state.color.b()],
        "x": mouse_state.x,
        "y": mouse_state.y
    });

    send_dds_message(&json_message.to_string(), &writer);
}

fn send_draw_stroke(stroke: &DrawStroke, draw_writer: Arc<Mutex<Writer>>) {
    let draw_stroke = DrawStroke {
        username: stroke.username.clone(),
        color: stroke.color,
        start_x: stroke.start_x,
        start_y: stroke.start_y,
        end_x: stroke.end_x,
        end_y: stroke.end_y,
        stroke_width: stroke.stroke_width,
        timestamp: stroke.timestamp,
    };

    let json_message = json!({
        "type": "Draw",
        "username": draw_stroke.username,
        "color": [draw_stroke.color.r(), draw_stroke.color.g(), draw_stroke.color.b()],
        "start_x": draw_stroke.start_x,
        "start_y": draw_stroke.start_y,
        "end_x": draw_stroke.end_x,
        "end_y": draw_stroke.end_y,
        "stroke_width": draw_stroke.stroke_width,
        "timestamp": draw_stroke.timestamp
    });

    send_dds_message(&json_message.to_string(), &draw_writer);
}

fn delete_image(image_id: String, image_delete_writer: Arc<Mutex<Writer>>) {
    println!("删除图片请求: image_id = {}", image_id);

    let delete_op = ImageDeleteOperation {
        username: get_username(),
        image_id: image_id.clone(),
    };

    let json_message = json!({
        "type": "ImageDelete",
        "username": delete_op.username,
        "image_id": delete_op.image_id
    });

    println!("发送删除消息: {}", json_message.to_string());
    send_dds_message(&json_message.to_string(), &image_delete_writer);
}

fn delete_video(video_id: String, video_delete_writer: Arc<Mutex<Writer>>) {
    let delete_op = VideoDeleteOperation {
        username: get_username(),
        video_id,
    };

    let json_message = json!({
        "type": "VideoDelete",
        "username": delete_op.username,
        "video_id": delete_op.video_id
    });

    send_dds_message(&json_message.to_string(), &video_delete_writer);

    println!("图片删除成功: {}", json_message.to_string());
}



// 新增：上传多张图片到本地队列
fn upload_images_to_queue(mut app_state: Signal<DioxusAppState>, image_queue_writer: Arc<Mutex<Writer>>) {
    spawn(async move {
        // 使用异步文件对话框选择多个文件
        let files = rfd::AsyncFileDialog::new()
            .add_filter("图片文件", &["png", "jpg", "jpeg", "gif", "bmp", "webp"])
            .set_title("选择多张图片文件")
            .pick_files()
            .await;

        if let Some(files) = files {
            let mut new_images = Vec::new();
            
            for file in files {
                match tokio::fs::read(file.path()).await {
                    Ok(image_data) => {
                        let timestamp = get_current_timestamp_millis();
                        
                        // 获取图片尺寸
                        let (width, height) = match image::open(file.path()) {
                            Ok(img) => (img.width(), img.height()),
                            Err(_) => (0, 0),
                        };
                        
                        let image_item = ImageItem {
                            id: format!("{}-{}", get_username(), timestamp),
                            username: get_username(),
                            image_data,
                            width,
                            height,
                            timestamp,
                        };
                        
                        new_images.push(image_item);
                        println!("成功加载图片: {}", file.file_name());
                    }
                    Err(e) => {
                        println!("读取图片文件失败 {}: {}", file.file_name(), e);
                    }
                }
            }
            
            if !new_images.is_empty() {
                // 添加到本地队列
                app_state.write().image_queue.extend(new_images.clone());
                
                // 如果是第一次添加图片，设置当前索引为0
                if app_state.read().image_queue.len() == new_images.len() {
                    app_state.write().current_image_index = 0;
                }
                
                // 发送DDS消息通知其他客户端
                let image_queue = ImageQueue {
                    username: get_username(),
                    images: new_images.clone(),
                    current_index: app_state.read().current_image_index,
                    timestamp: get_current_timestamp_millis(),
                };
                send_image_queue_data(&image_queue.images, image_queue.current_index, image_queue_writer);
                println!("成功添加 {} 张图片到队列，当前队列长度: {}", new_images.len(), app_state.read().image_queue.len());
            }
        }
    });
}

// 切换到上一张图片
fn switch_to_previous_image(mut app_state: Signal<DioxusAppState>) {
    let mut state = app_state.write();
    if !state.image_queue.is_empty() {
        if state.current_image_index > 0 {
            state.current_image_index -= 1;
        } else {
            state.current_image_index = state.image_queue.len() - 1;
        }
        println!("切换到上一张图片，当前索引: {}", state.current_image_index);
    }
}

// 切换到下一张图片
fn switch_to_next_image(mut app_state: Signal<DioxusAppState>) {
    let mut state = app_state.write();
    if !state.image_queue.is_empty() {
        if state.current_image_index < state.image_queue.len() - 1 {
            state.current_image_index += 1;
        } else {
            state.current_image_index = 0;
        }
        println!("切换到下一张图片，当前索引: {}", state.current_image_index);
    }
}

// 删除整个图片队列
fn delete_image_queue(mut app_state: Signal<DioxusAppState>, image_queue_delete_writer: Arc<Mutex<Writer>>) {
    let mut state = app_state.write();
    let queue_size = state.image_queue.len();
    state.image_queue.clear();
    state.current_image_index = 0;
    
    // 发送DDS删除消息
    send_image_queue_delete(image_queue_delete_writer);
    println!("删除图片队列，共删除 {} 张图片", queue_size);
}

fn upload_video(video_writer: Arc<Mutex<Writer>>) {
    // 使用spawn异步处理文件选择，避免阻塞UI线程
    spawn(async move {
        // 使用异步文件对话框
        if let Some(file_path) = rfd::AsyncFileDialog::new()
            .add_filter("视频文件", &["mp4", "avi", "mov", "mkv", "webm", "flv", "wmv"])
            .set_title("选择视频文件")
            .pick_file()
            .await
        {
            // 异步读取文件数据
            match tokio::fs::read(file_path.path()).await {
                Ok(video_data) => {
                    let file_name = file_path.file_name();
                    let file_size = video_data.len() as u64;
                    
                    println!("视频文件选择成功: {} ({} bytes)", file_name, file_size);
                    
                    // 发送视频数据通过DDS
                    send_video_data(video_data, file_name, file_size, video_writer);
                }
                Err(e) => {
                    println!("读取视频文件失败: {}", e);
                }
            }
        }
    });
}

fn send_dds_message(message: &str, writer: &Arc<Mutex<Writer>>) {
    let buffer = message.as_bytes();
    let mut data = Bytes::new();
    data.octet_seq_initialize();

        data.octet_seq_loan_contiguous(
            buffer,
            buffer.len() as u32,
            buffer.len() as u32,
        );

        let handle = writer.lock().unwrap().writer_register_instance(&mut data);
        writer.lock().unwrap().write(&data, &handle);
}

fn get_username() -> String {
    whoami::username()
}

fn get_current_timestamp() -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    format!("{:02}:{:02}", (timestamp / 60) % 60, timestamp % 60)
}

fn get_current_timestamp_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}



fn send_video_data(
    video_data: Vec<u8>,
    file_name: String,
    file_size: u64,
    video_writer: Arc<Mutex<Writer>>,
) {
    let username = get_username();

    // 将视频数据编码为base64字符串
    let video_data_b64 = general_purpose::STANDARD.encode(&video_data);

    let json_message = json!({
        "username": username,
        "video_data": video_data_b64,
        "file_name": file_name,
        "file_size": file_size
    });

    send_dds_message(&json_message.to_string(), &video_writer);
    println!(
        "视频DDS消息发送完成: {} ({} bytes)",
        file_name,
        video_data.len()
    );
}



fn send_user_color(
    color: egui::Color32,
    color_writer: Arc<Mutex<Writer>>,
) {
    let color_json = json!({
        "type": "UserColor",
        "username": get_username(),
        "color": {
            "r": color.r(),
            "g": color.g(),
            "b": color.b(),
            "a": color.a()
        },
        "timestamp": get_current_timestamp_millis()
    });
    
    let message = color_json.to_string();
    send_dds_message(&message, &color_writer);
    println!("发送用户颜色更新: {:?}", color);
}

// 发送图片队列数据
fn send_image_queue_data(
    images: &Vec<ImageItem>,
    current_index: usize,
    image_queue_writer: Arc<Mutex<Writer>>,
) {
    let username = get_username();
    let timestamp = get_current_timestamp_millis();
    
    // 将图片数据转换为JSON格式
    let images_json: Vec<serde_json::Value> = images.iter().map(|img| {
        let image_data_b64 = general_purpose::STANDARD.encode(&img.image_data);
        json!({
            "id": img.id,
            "username": img.username,
            "image_data": image_data_b64,
            "width": img.width,
            "height": img.height,
            "timestamp": img.timestamp
        })
    }).collect();
    
    let json_message = json!({
        "type": "ImageQueue",
        "username": username,
        "images": images_json,
        "current_index": current_index,
        "timestamp": timestamp
    });
    
    send_dds_message(&json_message.to_string(), &image_queue_writer);
    println!("发送图片队列数据: {} 张图片，当前索引: {}", images.len(), current_index);
}

// 发送图片队列删除消息
fn send_image_queue_delete(
    image_queue_delete_writer: Arc<Mutex<Writer>>,
) {
    let username = get_username();
    
    let json_message = json!({
        "type": "ImageQueueDelete",
        "username": username
    });
    
    send_dds_message(&json_message.to_string(), &image_queue_delete_writer);
    println!("发送图片队列删除消息: {}", username);
}


#[component]
pub fn ToggleSwitch() -> Element {
    // 开关状态
    let mut checked = use_signal(|| false);

    // 开关外壳样式
    let switch_style = format!(
        "width: 50px;
         height: 26px;
         border-radius: 9999px;
         background-color: {};
         position: relative;
         cursor: pointer;
         transition: background-color 0.2s;",
        if checked() { "blue" } else { "#d1d5db" } 
    );

    // 圆点样式
    let thumb_style = format!(
        "width: 22px;
         height: 22px;
         border-radius: 50%;
         background-color: white;
         position: absolute;
         top: 2px;
         left: {};
         transition: left 0.2s;",
        if checked() { "26px" } else { "2px" }
    );

    rsx! {
        div {
            "style": "{switch_style}",
            onclick: move |_| {
                checked.set(!checked());
            },
            div {
                "style": "{thumb_style}",
            }
        }
    }
}

// 域号输入界面组件Props