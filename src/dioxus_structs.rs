use std::sync::{Arc, Mutex};
use std::collections::HashMap;

// 颜色结构体，替代egui::Color32
#[derive(Clone, Debug, Copy, PartialEq)]
pub struct Color32 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color32 {
    pub const WHITE: Color32 = Color32 { r: 255, g: 255, b: 255, a: 255 };
    pub const BLACK: Color32 = Color32 { r: 0, g: 0, b: 0, a: 255 };
    pub const RED: Color32 = Color32 { r: 255, g: 0, b: 0, a: 255 };
    pub const GREEN: Color32 = Color32 { r: 0, g: 255, b: 0, a: 255 };
    pub const BLUE: Color32 = Color32 { r: 0, g: 0, b: 255, a: 255 };
    pub const YELLOW: Color32 = Color32 { r: 255, g: 255, b: 0, a: 255 };
    pub const CYAN: Color32 = Color32 { r: 0, g: 255, b: 255, a: 255 };
    pub const MAGENTA: Color32 = Color32 { r: 255, g: 0, b: 255, a: 255 };
    
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }
    
    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    
    pub fn to_hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
    
    pub fn to_css_rgba(&self) -> String {
        format!("rgba({}, {}, {}, {})", self.r, self.g, self.b, self.a as f32 / 255.0)
    }
}

// 消息类型枚举
#[derive(Clone, Debug)]
pub enum DioxusMessageType {
    Mouse,
    Image,
    Draw,
    Erase,
    ImageDelete,
    VideoDelete,
    Chat,
}

// 绘制模式枚举
#[derive(Clone, Debug, PartialEq)]
pub enum DrawMode {
    Mouse,
    Draw,
    Erase,
}

// 鼠标状态
#[derive(Clone, Debug)]
pub struct DioxusMouseState {
    pub username: String,
    pub color: Color32,
    pub x: f32,
    pub y: f32,
}

// 图片数据结构
#[derive(Clone, Debug)]
pub struct DioxusImageData {
    pub username: String,
    pub image_data: Vec<u8>, // PNG图片的原始字节数据
    pub width: u32,
    pub height: u32,
    pub base64_data: String, // Base64编码的图片数据，用于在HTML中显示
}

// 画笔笔迹数据结构
#[derive(Clone, Debug)]
pub struct DioxusDrawStroke {
    pub username: String,
    pub color: Color32,
    pub start_x: f32,
    pub start_y: f32,
    pub end_x: f32,
    pub end_y: f32,
    pub stroke_width: f32,
}

// 擦除操作数据结构
#[derive(Clone, Debug)]
pub struct DioxusEraseOperation {
    pub username: String,
    pub x: f32,
    pub y: f32,
    pub radius: f32, // 擦除半径
}

// 图片删除操作数据结构
#[derive(Clone, Debug)]
pub struct DioxusImageDeleteOperation {
    pub username: String,
    pub image_id: String, // 图片的唯一标识符（使用用户名作为ID）
}

// 视频数据结构
#[derive(Clone, Debug)]
pub struct DioxusVideoData {
    pub username: String,
    pub video_data: Vec<u8>, // 视频文件的原始字节数据
    pub file_name: String,   // 视频文件名
    pub file_size: u64,      // 文件大小
    pub video_url: String,   // 视频文件的临时URL
}

// 视频删除操作数据结构
#[derive(Clone, Debug)]
pub struct DioxusVideoDeleteOperation {
    pub username: String,
    pub video_id: String, // 视频的唯一标识符（使用用户名作为ID）
}

// 聊天消息数据结构
#[derive(Clone, Debug)]
pub struct DioxusChatMessage {
    pub username: String,
    pub message: String,
    pub timestamp: String, // 时间戳
}

// 弹幕数据结构
#[derive(Clone, Debug)]
pub struct DioxusDanmakuMessage {
    pub username: String,
    pub message: String,
    pub x: f32,           // 当前x位置
    pub y: f32,           // y位置
    pub speed: f32,       // 移动速度
    pub start_time: f64,  // 开始时间
    pub color: Color32,   // 弹幕颜色
    pub id: String,       // 弹幕唯一标识符
}

// 应用状态结构
#[derive(Clone, Debug)]
pub struct DioxusAppState {
    pub current_color: Color32,
    pub draw_mode: DrawMode,
    pub chat_input: String,
    pub danmaku_enabled: bool,
    pub canvas_width: f32,
    pub canvas_height: f32,
    pub stroke_width: f32,
    pub erase_radius: f32,
}

impl Default for DioxusAppState {
    fn default() -> Self {
        Self {
            current_color: Color32::BLACK,
            draw_mode: DrawMode::Mouse,
            chat_input: String::new(),
            danmaku_enabled: true,
            canvas_width: 800.0,
            canvas_height: 600.0,
            stroke_width: 2.0,
            erase_radius: 10.0,
        }
    }
}

// 全局共享状态（Dioxus版本）
pub static mut DIOXUS_RECEIVED_MICE: Option<Arc<Mutex<HashMap<String, DioxusMouseState>>>> = None;
pub static mut DIOXUS_RECEIVED_IMAGES: Option<Arc<Mutex<HashMap<String, DioxusImageData>>>> = None;
pub static mut DIOXUS_RECEIVED_VIDEOS: Option<Arc<Mutex<HashMap<String, DioxusVideoData>>>> = None;
pub static mut DIOXUS_RECEIVED_STROKES: Option<Arc<Mutex<Vec<DioxusDrawStroke>>>> = None;
pub static mut DIOXUS_RECEIVED_ERASES: Option<Arc<Mutex<Vec<DioxusEraseOperation>>>> = None;
pub static mut DIOXUS_RECEIVED_IMAGE_DELETES: Option<Arc<Mutex<Vec<DioxusImageDeleteOperation>>>> = None;
pub static mut DIOXUS_RECEIVED_VIDEO_DELETES: Option<Arc<Mutex<Vec<DioxusVideoDeleteOperation>>>> = None;
pub static mut DIOXUS_RECEIVED_CHAT_MESSAGES: Option<Arc<Mutex<Vec<DioxusChatMessage>>>> = None;
pub static mut DIOXUS_DANMAKU_MESSAGES: Option<Arc<Mutex<Vec<DioxusDanmakuMessage>>>> = None;