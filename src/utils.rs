use eframe::egui::Color32;
use serde_json::Value;
use image::{ImageFormat, DynamicImage, GenericImageView};
use base64::{Engine as _, engine::general_purpose};
use std::fs;
use std::path::Path;

// 从JSON值转换为Color32
pub fn color_from_json(value: &Value) -> Color32 {
    if let Some(array) = value.as_array() {
        if array.len() >= 4 {
            return Color32::from_rgba_unmultiplied(
                array[0].as_u64().unwrap_or(255) as u8,
                array[1].as_u64().unwrap_or(255) as u8,
                array[2].as_u64().unwrap_or(255) as u8,
                array[3].as_u64().unwrap_or(255) as u8,
            );
        }
    }
    Color32::WHITE
}

// 检查线段是否与圆相交
pub fn line_intersects_circle(x1: f32, y1: f32, x2: f32, y2: f32, cx: f32, cy: f32, radius: f32) -> bool {
    // 首先检查线段端点是否在圆内
    let dist1_sq = (x1 - cx) * (x1 - cx) + (y1 - cy) * (y1 - cy);
    let dist2_sq = (x2 - cx) * (x2 - cx) + (y2 - cy) * (y2 - cy);
    let radius_sq = radius * radius;
    
    if dist1_sq <= radius_sq || dist2_sq <= radius_sq {
        return true;
    }
    
    // 对于很短的线段（小于1像素），直接检查中点距离
    let line_length_sq = (x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1);
    if line_length_sq < 1.0 {
        let mid_x = (x1 + x2) / 2.0;
        let mid_y = (y1 + y2) / 2.0;
        let mid_dist_sq = (mid_x - cx) * (mid_x - cx) + (mid_y - cy) * (mid_y - cy);
        return mid_dist_sq <= radius_sq;
    }
    
    // 计算线段到圆心的最短距离
    let dx = x2 - x1;
    let dy = y2 - y1;
    let fx = x1 - cx;
    let fy = y1 - cy;
    
    let a = dx * dx + dy * dy;
    let b = 2.0 * (fx * dx + fy * dy);
    let c = (fx * fx + fy * fy) - radius_sq;
    
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return false;
    }
    
    let discriminant = discriminant.sqrt();
    
    let t1 = (-b - discriminant) / (2.0 * a);
    let t2 = (-b + discriminant) / (2.0 * a);
    
    // 检查交点是否在线段上
    (t1 >= 0.0 && t1 <= 1.0) || (t2 >= 0.0 && t2 <= 1.0) || (t1 < 0.0 && t2 > 1.0)
}

// 加载图片并转换为base64
pub fn load_image_as_base64(file_path: &str) -> Result<(String, u32, u32), Box<dyn std::error::Error>> {
    let img = image::open(file_path)?;
    let (width, height) = img.dimensions();
    
    let mut buffer = Vec::new();
    let mut cursor = std::io::Cursor::new(&mut buffer);
    img.write_to(&mut cursor, ImageFormat::Png)?;
    
    let base64_string = general_purpose::STANDARD.encode(&buffer);
    
    Ok((base64_string, width, height))
}

// 加载视频文件并转换为base64
pub fn load_video_as_base64(file_path: &str) -> Result<(String, String, u64), Box<dyn std::error::Error>> {
    let video_data = fs::read(file_path)?;
    let base64_string = general_purpose::STANDARD.encode(&video_data);
    
    let file_name = Path::new(file_path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("video.mp4")
        .to_string();
    
    let file_size = video_data.len() as u64;
    
    Ok((base64_string, file_name, file_size))
}