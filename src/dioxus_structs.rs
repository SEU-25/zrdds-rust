use eframe::egui;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

// 消息类型枚举
#[derive(Clone, Debug)]
pub enum MessageType {
    Mouse,
    Image,
    Draw,
    Erase,
    ImageDelete,
}

// 每个鼠标状态
#[derive(Clone)]
pub struct MouseState {
    pub username: String,
    pub color: egui::Color32,
    pub x: f32,
    pub y: f32,
}

// 图片数据结构
#[derive(Clone)]
pub struct ImageData {
    pub username: String,
    pub image_data: Vec<u8>, // PNG图片的原始字节数据
    pub width: u32,
    pub height: u32,
}

// 画笔笔迹数据结构
#[derive(Clone)]
pub struct DrawStroke {
    pub username: String,
    pub color: egui::Color32,
    pub start_x: f32,
    pub start_y: f32,
    pub end_x: f32,
    pub end_y: f32,
    pub stroke_width: f32,
    pub timestamp: u64, // 笔迹创建时间戳（毫秒）
}

// 擦除操作数据结构
#[derive(Clone)]
pub struct EraseOperation {
    pub username: String,
    pub x: f32,
    pub y: f32,
    pub radius: f32, // 擦除半径
    pub timestamp: u64, // 擦除操作时间戳（毫秒）
}

// 图片删除操作数据结构
#[derive(Clone)]
pub struct ImageDeleteOperation {
    pub username: String,
    pub image_id: String, // 图片的唯一标识符（使用用户名作为ID）
}

// 视频数据结构
#[derive(Clone)]
pub struct VideoData {
    pub username: String,
    pub video_data: Vec<u8>, // 视频文件的原始字节数据
    pub file_name: String,   // 视频文件名
    pub file_size: u64,      // 文件大小
}

// 视频删除操作数据结构
#[derive(Clone)]
pub struct VideoDeleteOperation {
    pub username: String,
    pub video_id: String, // 视频的唯一标识符（使用用户名作为ID）
}

// 聊天消息数据结构
#[derive(Clone)]
pub struct ChatMessage {
    pub username: String,
    pub message: String,
    pub timestamp: String, // 时间戳
}

// 弹幕数据结构
#[derive(Clone)]
pub struct DanmakuMessage {
    pub username: String,
    pub message: String,
    pub x: f32,           // 当前x位置
    pub y: f32,           // y位置
    pub speed: f32,       // 移动速度
    pub start_time: f64,  // 开始时间
    pub color: egui::Color32, // 弹幕颜色
}

// 全局共享状态
pub static mut RECEIVED: Option<Arc<Mutex<HashMap<String, MouseState>>>> = None;
pub static mut RECEIVED_IMAGES: Option<Arc<Mutex<HashMap<String, ImageData>>>> = None;
pub static mut RECEIVED_VIDEOS: Option<Arc<Mutex<HashMap<String, VideoData>>>> = None;
pub static mut RECEIVED_STROKES: Option<Arc<Mutex<Vec<DrawStroke>>>> = None;
pub static mut RECEIVED_ERASES: Option<Arc<Mutex<Vec<EraseOperation>>>> = None;
pub static mut RECEIVED_IMAGE_DELETES: Option<Arc<Mutex<Vec<ImageDeleteOperation>>>> = None;
pub static mut RECEIVED_VIDEO_DELETES: Option<Arc<Mutex<Vec<VideoDeleteOperation>>>> = None;
pub static mut RECEIVED_CHAT_MESSAGES: Option<Arc<Mutex<Vec<ChatMessage>>>> = None;