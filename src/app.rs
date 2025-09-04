use std::{collections::HashMap, mem, sync::{Arc, Mutex}};
use eframe::egui;
use serde_json::json;
use zrdds::bindings::*;
use crate::structs::*;
use crate::utils::*;
use egui_video::{Player, AudioDevice};
use base64::{Engine as _, engine::general_purpose};

pub struct MouseApp {
    pub received: Arc<Mutex<HashMap<String, MouseState>>>,
    pub received_images: Arc<Mutex<HashMap<String, ImageData>>>,
    pub received_videos: Arc<Mutex<HashMap<String, VideoData>>>,
    pub received_strokes: Arc<Mutex<Vec<DrawStroke>>>,
    pub received_erases: Arc<Mutex<Vec<EraseOperation>>>,
    pub received_image_deletes: Arc<Mutex<Vec<ImageDeleteOperation>>>,
    pub received_video_deletes: Arc<Mutex<Vec<VideoDeleteOperation>>>,
    pub received_chat_messages: Arc<Mutex<Vec<ChatMessage>>>,
    pub writer: Arc<Mutex<*mut DDS_DataWriter>>,
    pub image_writer: Arc<Mutex<*mut DDS_DataWriter>>,
    pub video_writer: Arc<Mutex<*mut DDS_DataWriter>>,
    pub draw_writer: Arc<Mutex<*mut DDS_DataWriter>>,
    pub erase_writer: Arc<Mutex<*mut DDS_DataWriter>>,
    pub image_delete_writer: Arc<Mutex<*mut DDS_DataWriter>>,
    pub video_delete_writer: Arc<Mutex<*mut DDS_DataWriter>>,
    pub chat_writer: Arc<Mutex<*mut DDS_DataWriter>>,
    pub my_color: egui::Color32,
    pub texture_cache: HashMap<String, egui::TextureHandle>, // 纹理缓存
    pub is_draw_mode: bool, // 是否处于画笔模式
    pub is_erase_mode: bool, // 是否处于擦除模式
    pub is_drawing: bool, // 是否正在绘制
    pub last_draw_pos: Option<egui::Pos2>, // 上一个绘制位置
    pub local_strokes: Vec<DrawStroke>, // 本地笔迹存储
    pub chat_input: String, // 聊天输入框内容
    pub danmaku_enabled: bool, // 弹幕开关
    pub danmaku_messages: Vec<DanmakuMessage>, // 活跃的弹幕消息
    pub last_message_count: usize, // 上次处理的消息数量
    pub has_uploaded_image: bool, // 当前用户是否已上传图片
    pub has_uploaded_video: bool, // 当前用户是否已上传视频
    pub current_video_path: Option<String>, // 当前播放的视频文件路径
    pub video_player: Option<Player>, // egui-video播放器
    pub audio_device: Option<AudioDevice>, // 音频设备
}

impl MouseApp {
    pub fn new(
        received: Arc<Mutex<HashMap<String, MouseState>>>,
        received_images: Arc<Mutex<HashMap<String, ImageData>>>,
        received_videos: Arc<Mutex<HashMap<String, VideoData>>>,
        received_strokes: Arc<Mutex<Vec<DrawStroke>>>,
        received_erases: Arc<Mutex<Vec<EraseOperation>>>,
        received_image_deletes: Arc<Mutex<Vec<ImageDeleteOperation>>>,
        received_video_deletes: Arc<Mutex<Vec<VideoDeleteOperation>>>,
        received_chat_messages: Arc<Mutex<Vec<ChatMessage>>>,
        writer: Arc<Mutex<*mut DDS_DataWriter>>,
        image_writer: Arc<Mutex<*mut DDS_DataWriter>>,
        video_writer: Arc<Mutex<*mut DDS_DataWriter>>,
        draw_writer: Arc<Mutex<*mut DDS_DataWriter>>,
        erase_writer: Arc<Mutex<*mut DDS_DataWriter>>,
        image_delete_writer: Arc<Mutex<*mut DDS_DataWriter>>,
        video_delete_writer: Arc<Mutex<*mut DDS_DataWriter>>,
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
        
        // 设置字体
        cc.egui_ctx.set_fonts(fonts);

        Self {
            received,
            received_images,
            received_videos,
            received_strokes,
            received_erases,
            received_image_deletes,
            received_video_deletes,
            received_chat_messages,
            writer,
            image_writer,
            video_writer,
            draw_writer,
            erase_writer,
            image_delete_writer,
            video_delete_writer,
            chat_writer,
            my_color: egui::Color32::from_rgb(255, 0, 0),
            texture_cache: HashMap::new(),
            is_draw_mode: false,
            is_erase_mode: false,
            is_drawing: false,
            last_draw_pos: None,
            local_strokes: Vec::new(),
            chat_input: String::new(),
            danmaku_enabled: true,
            danmaku_messages: Vec::new(),
            last_message_count: 0,
            has_uploaded_image: false,
            has_uploaded_video: false,
            current_video_path: None,
            video_player: None,
            audio_device: None,
        }
    }

    pub fn update_danmaku(&mut self, ctx: &egui::Context) {
        let current_time = ctx.input(|i| i.time);
        
        // 更新弹幕位置
        for danmaku in &mut self.danmaku_messages {
            let elapsed = current_time - danmaku.start_time;
            danmaku.x -= danmaku.speed * elapsed as f32 * 60.0; // 假设60fps
        }
        
        // 移除已经移出屏幕的弹幕
        self.danmaku_messages.retain(|danmaku| danmaku.x > -200.0);
    }

    pub fn add_danmaku(&mut self, message: &ChatMessage, ctx: &egui::Context) {
        let screen_rect = ctx.screen_rect();
        let start_x = screen_rect.width();
        
        // 随机选择Y位置（避免重叠）
        let y_positions = [100.0, 150.0, 200.0, 250.0, 300.0, 350.0, 400.0];
        let y_index = self.danmaku_messages.len() % y_positions.len();
        let y = y_positions[y_index];
        
        let danmaku = DanmakuMessage {
            username: message.username.clone(),
            message: format!("{}: {}", message.username, message.message),
            x: start_x,
            y,
            speed: 0.02, // 移动速度
            start_time: ctx.input(|i| i.time),
            color: self.my_color, // 使用当前鼠标颜色
        };
        
        self.danmaku_messages.push(danmaku);
    }

    pub fn send_chat_message(&mut self) {
        if !self.chat_input.trim().is_empty() {
            let username = whoami::username();
            let timestamp = chrono::Utc::now().format("%H:%M:%S").to_string();
            
            let chat_message = json!({
                "type": "Chat",
                "username": username,
                "message": self.chat_input.trim(),
                "timestamp": timestamp
            });
            
            let json_str = chat_message.to_string();
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
                
                let writer = *self.chat_writer.lock().unwrap();
                let handle = DDS_BytesDataWriter_register_instance(writer as *mut DDS_BytesDataWriter, &mut data);
                DDS_BytesDataWriter_write(writer as *mut DDS_BytesDataWriter, &mut data, &handle);
            }
            
            self.chat_input.clear();
        }
    }
}

impl eframe::App for MouseApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 右侧面板 - 聊天区域
        egui::SidePanel::right("chat_panel")
            .resizable(true)
            .default_width(300.0)
            .show(ctx, |ui| {
                ui.heading("聊天室");
                
                // 弹幕开关
                ui.horizontal(|ui| {
                    ui.checkbox(&mut self.danmaku_enabled, "启用弹幕");
                });
                
                ui.separator();
                
                // 聊天消息显示区域
                egui::ScrollArea::vertical()
                    .max_height(400.0)
                    .show(ui, |ui| {
                        let messages = self.received_chat_messages.lock().unwrap();
                        for message in messages.iter() {
                            ui.horizontal(|ui| {
                                ui.label(format!("[{}]", message.timestamp));
                                ui.label(format!("{}: {}", message.username, message.message));
                            });
                        }
                    });
                
                // 检查是否有新消息需要生成弹幕
                let new_messages = {
                    if let Ok(messages) = self.received_chat_messages.lock() {
                        let current_count = messages.len();
                        if current_count > self.last_message_count {
                            // 克隆新消息
                            let new_msgs: Vec<ChatMessage> = messages.iter().skip(self.last_message_count).cloned().collect();
                            self.last_message_count = current_count;
                            new_msgs
                        } else {
                            Vec::new()
                        }
                    } else {
                         Vec::new()
                     }
                };
                
                // 为新消息生成弹幕
                if self.danmaku_enabled {
                    for message in &new_messages {
                        self.add_danmaku(message, ctx);
                    }
                }
                
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

        egui::CentralPanel::default().show(ctx, |ui| {
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
            
            // 图片和视频选择和发送按钮
            ui.horizontal(|ui| {
                // 检查当前用户是否已上传图片或视频
                let current_username = whoami::username();
                let has_current_user_image = {
                    let images = self.received_images.lock().unwrap();
                    images.contains_key(&current_username)
                };
                let has_current_user_video = {
                    let videos = self.received_videos.lock().unwrap();
                    videos.contains_key(&current_username)
                };
                
                let has_any_media = has_current_user_image || has_current_user_video;
                
                // 图片上传按钮
                let image_button_text = if has_current_user_image {
                    "已上传图片"
                } else if has_current_user_video {
                    "已有视频，无法上传图片"
                } else {
                    "选择并发送图片"
                };
                
                if ui.add_enabled(!has_any_media, egui::Button::new(image_button_text)).clicked() {
                        // 使用文件对话框选择图片文件
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("PNG图片", &["png"])
                            .add_filter("JPEG图片", &["jpg", "jpeg"])
                            .add_filter("所有图片", &["png", "jpg", "jpeg"])
                            .pick_file() {
                            
                            match load_image_as_base64(&path.to_string_lossy()) {
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
                                    
                                    println!("图片上传成功: {}x{}", width, height);
                                }
                                Err(e) => {
                                    eprintln!("加载图片失败: {}", e);
                                }
                            }
                        }
                    }
                
                // 视频上传按钮
                let video_button_text = if has_current_user_video {
                    "已上传视频"
                } else if has_current_user_image {
                    "已有图片，无法上传视频"
                } else {
                    "选择并发送视频"
                };
                
                if ui.add_enabled(!has_any_media, egui::Button::new(video_button_text)).clicked() {
                    // 使用文件对话框选择视频文件
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("MP4视频", &["mp4"])
                        .add_filter("AVI视频", &["avi"])
                        .add_filter("MOV视频", &["mov"])
                        .add_filter("所有视频", &["mp4", "avi", "mov", "mkv", "webm"])
                        .pick_file() {
                        
                        match load_video_as_base64(&path.to_string_lossy()) {
                            Ok((base64_data, file_name, file_size)) => {
                                let username = whoami::username();
                                let video_message = json!({
                                    "type": "Video",
                                    "username": username,
                                    "video_data": base64_data,
                                    "file_name": file_name,
                                    "file_size": file_size
                                });
                                
                                let json_str = video_message.to_string();
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
                                    
                                    let writer = *self.video_writer.lock().unwrap();
                                    let handle = DDS_BytesDataWriter_register_instance(writer as *mut DDS_BytesDataWriter, &mut data);
                                    DDS_BytesDataWriter_write(writer as *mut DDS_BytesDataWriter, &mut data, &handle);
                                }
                                
                                println!("视频上传成功: {} ({}字节)", file_name, file_size);
                            }
                            Err(e) => {
                                eprintln!("加载视频失败: {}", e);
                            }
                        }
                    }
                }
             });
 
            // 更新弹幕动画
            self.update_danmaku(ctx);
            
            // 在窗口中间绘制固定大小的画框
            let screen_rect = ui.max_rect();
            let frame_size = egui::vec2(600.0, 400.0); // 固定画框大小
            let frame_center = screen_rect.center();
            let frame_rect = egui::Rect::from_center_size(frame_center, frame_size);
            
            // 使用单独的作用域来处理painter
            {
                let painter = ui.painter();
                
                // 渲染弹幕
                for danmaku in &self.danmaku_messages {
                    painter.text(
                        egui::pos2(danmaku.x, danmaku.y),
                        egui::Align2::LEFT_CENTER,
                        &danmaku.message,
                        egui::FontId::proportional(18.0),
                        danmaku.color,
                    );
                }
                
                // 绘制画框边框
                painter.rect_stroke(
                    frame_rect,
                    5.0, // 圆角
                    egui::Stroke::new(3.0, egui::Color32::WHITE)
                );
                
                // 绘制画框背景
                painter.rect_filled(
                    frame_rect,
                    5.0,
                    egui::Color32::from_rgba_premultiplied(0, 0, 0, 100) // 半透明黑色背景
                );
            }
            
            // 显示接收到的图片在画框中
            let image_data = self.received_images.lock().unwrap();
            if let Some((username, img_data)) = image_data.iter().last() { // 只显示最新的图片
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
                    // 计算图片在画框中的显示尺寸（保持宽高比，适应画框）
                    let img_aspect = img_data.width as f32 / img_data.height as f32;
                    let frame_aspect = frame_size.x / frame_size.y;
                    
                    let display_size = if img_aspect > frame_aspect {
                        // 图片更宽，以宽度为准
                        let width = frame_size.x - 20.0; // 留边距
                        let height = width / img_aspect;
                        egui::vec2(width, height)
                    } else {
                        // 图片更高，以高度为准
                        let height = frame_size.y - 20.0; // 留边距
                        let width = height * img_aspect;
                        egui::vec2(width, height)
                    };
                    
                    // 在画框中心显示图片
                    let image_rect = egui::Rect::from_center_size(frame_center, display_size);
                    
                    // 在画框右上角添加删除按钮
                    let delete_button_size = 30.0;
                    let delete_button_pos = egui::pos2(
                        frame_rect.max.x - delete_button_size - 10.0,
                        frame_rect.min.y + 10.0
                    );
                    let delete_button_rect = egui::Rect::from_min_size(
                        delete_button_pos,
                        egui::vec2(delete_button_size, delete_button_size)
                    );
                    
                    // 使用单独的作用域处理图片相关的painter
                    {
                        let img_painter = ui.painter();
                        img_painter.image(
                            texture.id(),
                            image_rect,
                            egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                            egui::Color32::WHITE
                        );
                        
                        // 在画框上方显示用户名
                        img_painter.text(
                            egui::pos2(frame_rect.center().x, frame_rect.min.y - 25.0),
                            egui::Align2::CENTER_BOTTOM,
                            format!("图片来自: {}", username),
                            egui::FontId::proportional(16.0),
                            egui::Color32::YELLOW,
                        );
                        
                        // 绘制删除按钮背景（红色圆形）
                        img_painter.circle_filled(
                            delete_button_rect.center(),
                            delete_button_size / 2.0,
                            egui::Color32::from_rgb(200, 50, 50)
                        );
                        
                        // 绘制删除按钮的X符号
                        let x_size = delete_button_size * 0.3;
                        let center = delete_button_rect.center();
                        img_painter.line_segment(
                            [egui::pos2(center.x - x_size, center.y - x_size), 
                             egui::pos2(center.x + x_size, center.y + x_size)],
                            egui::Stroke::new(3.0, egui::Color32::WHITE)
                        );
                        img_painter.line_segment(
                            [egui::pos2(center.x + x_size, center.y - x_size), 
                             egui::pos2(center.x - x_size, center.y + x_size)],
                            egui::Stroke::new(3.0, egui::Color32::WHITE)
                        );
                    }
                    
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
                    
                    // 由于现在只显示最新图片，不需要y_offset
                }
            }
            
            // 显示接收到的视频在画框中（如果没有图片的话）
            let video_data = self.received_videos.lock().unwrap();
            if image_data.is_empty() && !video_data.is_empty() {
                if let Some((username, vid_data)) = video_data.iter().last() { // 只显示最新的视频
                    // 在画框中心显示视频播放器界面
                    let video_rect = egui::Rect::from_center_size(frame_center, egui::vec2(400.0, 300.0));
                    
                    // 如果还没有初始化视频播放器，先初始化
                    if self.video_player.is_none() {
                        // 保存视频文件到临时目录
                        let video_bytes = &vid_data.video_data; // video_data已经是解码后的字节数据
                        {
                            let temp_dir = std::env::temp_dir();
                            let video_path = temp_dir.join(&vid_data.file_name);
                            
                            if let Ok(_) = std::fs::write(&video_path, video_bytes) {
                                self.current_video_path = Some(video_path.to_string_lossy().to_string());
                                
                                // 初始化音频设备
                                if self.audio_device.is_none() {
                                    match egui_video::AudioDevice::new() {
                                        Ok(device) => self.audio_device = Some(device),
                                        Err(e) => eprintln!("初始化音频设备失败: {}", e),
                                    }
                                }
                                
                                // 初始化视频播放器
                                match egui_video::Player::new(ctx, &video_path.to_string_lossy().to_string()) {
                                    Ok(mut player) => {
                                        // 如果有音频设备，添加音频支持
                                        let final_player = if let Some(ref mut audio_device) = self.audio_device {
                                            player.with_audio(audio_device).unwrap_or_else(|_| {
                                                // 如果添加音频失败，重新创建播放器
                                                egui_video::Player::new(ctx, &video_path.to_string_lossy().to_string()).unwrap_or_else(|_| {
                                                    panic!("无法创建视频播放器")
                                                })
                                            })
                                        } else {
                                            player
                                        };
                                        self.video_player = Some(final_player);
                                        println!("视频播放器初始化成功: {}", vid_data.file_name);
                                    },
                                    Err(e) => eprintln!("初始化视频播放器失败: {}", e),
                                }
                            } else {
                                eprintln!("保存视频文件失败");
                            }
                        }
                    }
                    
                    // 如果视频播放器已初始化，显示视频
                    if let Some(ref mut player) = self.video_player {
                        // 计算视频在指定区域内的适当显示尺寸（保持宽高比）
                        let video_size = player.size;
                        let video_aspect = video_size.x / video_size.y;
                        let rect_aspect = video_rect.width() / video_rect.height();
                        
                        let display_size = if video_aspect > rect_aspect {
                            // 视频更宽，以宽度为准
                            let width = video_rect.width() - 20.0; // 留边距
                            let height = width / video_aspect;
                            egui::vec2(width, height)
                        } else {
                            // 视频更高，以高度为准
                            let height = video_rect.height() - 20.0; // 留边距
                            let width = height * video_aspect;
                            egui::vec2(width, height)
                        };
                        
                        // 在指定区域内显示视频
                        ui.allocate_new_ui(egui::UiBuilder::new().max_rect(video_rect), |ui| {
                            player.ui(ui, display_size);
                        });
                    } else {
                        // 如果播放器未初始化，显示加载界面
                        let video_painter = ui.painter();
                        video_painter.rect_filled(
                            video_rect,
                            10.0,
                            egui::Color32::from_rgb(40, 40, 40)
                        );
                        
                        video_painter.rect_stroke(
                            video_rect,
                            10.0,
                            egui::Stroke::new(2.0, egui::Color32::GRAY)
                        );
                        
                        video_painter.text(
                            egui::pos2(video_rect.center().x, video_rect.center().y),
                            egui::Align2::CENTER_CENTER,
                            "正在加载视频...",
                            egui::FontId::proportional(18.0),
                            egui::Color32::WHITE,
                        );
                    }
                    
                    // 在画框上方显示用户名和视频信息
                    {
                        let info_painter = ui.painter();
                        info_painter.text(
                            egui::pos2(frame_rect.center().x, frame_rect.min.y - 45.0),
                            egui::Align2::CENTER_BOTTOM,
                            format!("视频来自: {}", username),
                            egui::FontId::proportional(16.0),
                            egui::Color32::YELLOW,
                        );
                        
                        info_painter.text(
                            egui::pos2(frame_rect.center().x, frame_rect.min.y - 25.0),
                            egui::Align2::CENTER_BOTTOM,
                            format!("文件: {} ({:.1}MB)", vid_data.file_name, vid_data.file_size as f64 / (1024.0 * 1024.0)),
                            egui::FontId::proportional(12.0),
                            egui::Color32::LIGHT_GRAY,
                        );
                        
                        // 在画框右上角添加删除按钮
                        let delete_button_size = 30.0;
                        let delete_button_pos = egui::pos2(
                            frame_rect.max.x - delete_button_size - 10.0,
                            frame_rect.min.y + 10.0
                        );
                        let delete_button_rect = egui::Rect::from_min_size(
                            delete_button_pos,
                            egui::vec2(delete_button_size, delete_button_size)
                        );
                        
                        // 绘制删除按钮背景（红色圆形）
                        info_painter.circle_filled(
                            delete_button_rect.center(),
                            delete_button_size / 2.0,
                            egui::Color32::from_rgb(200, 50, 50)
                        );
                        
                        // 绘制删除按钮的X符号
                        let x_size = delete_button_size * 0.3;
                        let center = delete_button_rect.center();
                        info_painter.line_segment(
                            [egui::pos2(center.x - x_size, center.y - x_size), 
                             egui::pos2(center.x + x_size, center.y + x_size)],
                            egui::Stroke::new(3.0, egui::Color32::WHITE)
                        );
                        info_painter.line_segment(
                            [egui::pos2(center.x + x_size, center.y - x_size), 
                             egui::pos2(center.x - x_size, center.y + x_size)],
                            egui::Stroke::new(3.0, egui::Color32::WHITE)
                        );
                        
                        // 检测删除按钮点击
                        if ui.input(|i| i.pointer.any_click()) {
                            if let Some(pointer_pos) = ui.input(|i| i.pointer.interact_pos()) {
                                if delete_button_rect.contains(pointer_pos) {
                                // 触发删除操作
                                println!("删除视频: {}", username);
                                
                                // 创建视频删除操作数据
                                let delete_op = VideoDeleteOperation {
                                    username: whoami::username(),
                                    video_id: username.clone(),
                                };
                                
                                // 发送视频删除消息到DDS
                                if let Ok(writer) = self.video_delete_writer.lock() {
                                    let json_data = json!({
                                        "type": "VideoDelete",
                                        "username": delete_op.username,
                                        "video_id": delete_op.video_id
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
                            }
                        }
                    }
                }
                }
            }
            
            // 使用单独的作用域处理画笔和鼠标相关的painter
            {
                let draw_painter = ui.painter();
                
                // 渲染本地画笔笔迹
                for stroke in &self.local_strokes {
                    let start_pos = egui::pos2(stroke.start_x, stroke.start_y);
                    let end_pos = egui::pos2(stroke.end_x, stroke.end_y);
                    let color = egui::Color32::from_rgba_unmultiplied(
                        stroke.color[0], stroke.color[1], stroke.color[2], stroke.color[3],
                    );
                    
                    draw_painter.line_segment([start_pos, end_pos], egui::Stroke::new(stroke.stroke_width, color));
                }
                
                // 渲染远程接收到的画笔笔迹
                let received_strokes = self.received_strokes.lock().unwrap();
                for stroke in received_strokes.iter() {
                    let start_pos = egui::pos2(stroke.start_x, stroke.start_y);
                    let end_pos = egui::pos2(stroke.end_x, stroke.end_y);
                    let color = egui::Color32::from_rgba_unmultiplied(
                        stroke.color[0], stroke.color[1], stroke.color[2], stroke.color[3],
                    );
                    
                    draw_painter.line_segment([start_pos, end_pos], egui::Stroke::new(stroke.stroke_width, color));
                }
                
                // 渲染鼠标圆点和坐标（显示在图片和画笔笔迹之上）
                let data = self.received.lock().unwrap();
                for mouse in data.values() {
                    let pos = egui::pos2(mouse.x, mouse.y);
                    let color = egui::Color32::from_rgba_unmultiplied(
                        mouse.color[0], mouse.color[1], mouse.color[2], mouse.color[3],
                    );

                    // 1. 画一个小圆点当作"鼠标"
                    draw_painter.circle_filled(pos, 6.0, color);

                    // 2. 在圆点旁边显示坐标 (文字)
                    let text = format!("{} ({:.0}, {:.0})", mouse.username, mouse.x, mouse.y);
                    draw_painter.text(
                        pos + egui::vec2(10.0, -10.0),         // 偏移一点，不挡住圆点
                        egui::Align2::LEFT_TOP,
                        text,
                        egui::FontId::proportional(14.0),
                        egui::Color32::WHITE,
                    );
                }
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
        
        // 处理接收到的视频删除操作
        let received_video_deletes = self.received_video_deletes.lock().unwrap().clone();
        for delete_op in received_video_deletes {
            // 从本地视频缓存中删除视频
            {
                let mut received_videos = self.received_videos.lock().unwrap();
                received_videos.remove(&delete_op.video_id);
            }
            
            println!("已删除视频: {} (来自用户: {})", delete_op.video_id, delete_op.username);
        }
        
        // 清空已处理的视频删除操作
        self.received_video_deletes.lock().unwrap().clear();
        ctx.request_repaint();
    
    }
}