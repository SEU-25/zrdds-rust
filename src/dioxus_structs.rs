use eframe::egui;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

// Color32 序列化支持
mod color32_serde {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use eframe::egui::Color32;

    pub fn serialize<S>(color: &Color32, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let [r, g, b, a] = color.to_array();
        (r, g, b, a).serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Color32, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (r, g, b, a) = <(u8, u8, u8, u8)>::deserialize(deserializer)?;
        Ok(Color32::from_rgba_unmultiplied(r, g, b, a))
    }
}

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
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct ImageData {
    pub username: String,
    pub image_data: Vec<u8>, // PNG图片的原始字节数据
    pub width: u32,
    pub height: u32,
}

// 单个图片项结构
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ImageItem {
    pub id: String,           // 图片唯一标识符
    pub username: String,
    pub image_data: Vec<u8>,  // PNG图片的原始字节数据
    pub width: u32,
    pub height: u32,
    pub timestamp: u64,       // 上传时间戳
}

// 图片队列数据结构
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ImageQueue {
    pub username: String,
    pub images: Vec<ImageItem>,  // 图片列表
    pub current_index: usize,    // 当前显示的图片索引
    pub timestamp: u64,          // 队列更新时间戳
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

// 图片队列删除操作数据结构
#[derive(Clone)]
pub struct ImageQueueDeleteOperation {
    pub username: String,
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
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct ChatMessage {
    pub username: String,
    pub message: String,
    pub timestamp: String, // 时间戳
    #[serde(with = "color32_serde")]
    pub color: egui::Color32, // 用户选择的颜色
}

// 弹幕消息数据结构
#[derive(Clone)]
pub struct DanmakuMessage {
    pub username: String,
    pub message: String,
    pub x: f32,           // 当前x位置
    pub y: f32,           // y位置
    pub speed: f32,       // 移动速度
    pub start_time: f64,  // 开始时间
    pub color: egui::Color32, // 弹幕颜色
    pub id: String,       // 弹幕唯一标识符
}

// 用户颜色数据结构
#[derive(Clone)]
pub struct UserColor {
    pub username: String,
    pub color: egui::Color32, // 用户选择的画笔颜色
    pub timestamp: u64,       // 颜色更新时间戳
}

// 全局共享状态
pub static mut RECEIVED: Option<Arc<Mutex<HashMap<String, MouseState>>>> = None;
pub static mut RECEIVED_IMAGES: Option<Arc<Mutex<HashMap<String, ImageData>>>> = None;
pub static mut RECEIVED_IMAGE_QUEUES: Option<Arc<Mutex<HashMap<String, ImageQueue>>>> = None;
pub static mut RECEIVED_VIDEOS: Option<Arc<Mutex<HashMap<String, VideoData>>>> = None;
pub static mut RECEIVED_STROKES: Option<Arc<Mutex<Vec<DrawStroke>>>> = None;
pub static mut RECEIVED_ERASES: Option<Arc<Mutex<Vec<EraseOperation>>>> = None;
pub static mut RECEIVED_IMAGE_DELETES: Option<Arc<Mutex<Vec<ImageDeleteOperation>>>> = None;
pub static mut RECEIVED_IMAGE_QUEUE_DELETES: Option<Arc<Mutex<Vec<ImageQueueDeleteOperation>>>> = None;
pub static mut RECEIVED_VIDEO_DELETES: Option<Arc<Mutex<Vec<VideoDeleteOperation>>>> = None;
pub static mut RECEIVED_CHAT_MESSAGES: Option<Arc<Mutex<Vec<ChatMessage>>>> = None;
pub static mut RECEIVED_DANMAKU_MESSAGES: Option<Arc<Mutex<Vec<DanmakuMessage>>>> = None;
pub static mut RECEIVED_USER_COLORS: Option<Arc<Mutex<HashMap<String, UserColor>>>> = None;
pub static mut DANMAKU_ENABLED: Option<Arc<Mutex<bool>>> = None;