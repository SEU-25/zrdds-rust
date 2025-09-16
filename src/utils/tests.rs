#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tempfile::NamedTempFile;
    use std::io::Write;
    use base64::Engine as _;
    use crate::utils::{color_from_json, line_intersects_circle, load_video_as_base64, load_image_as_base64};

    #[test]
    fn test_color_from_json_rgba() {
        let color_json = json!([255, 128, 64, 200]);
        let color = color_from_json(&color_json);
        assert_eq!(color, egui::Color32::from_rgba_unmultiplied(255, 128, 64, 200));
    }

    #[test]
    fn test_color_from_json_rgb() {
        let color_json = json!([255, 128, 64]);
        let color = color_from_json(&color_json);
        // RGB数组长度小于4，应该返回白色
        assert_eq!(color, egui::Color32::WHITE);
    }

    #[test]
    fn test_color_from_json_invalid() {
        let invalid_json = json!("not_an_array");
        let color = color_from_json(&invalid_json);
        assert_eq!(color, egui::Color32::WHITE);
    }

    #[test]
    fn test_color_from_json_empty_array() {
        let empty_json = json!([]);
        let color = color_from_json(&empty_json);
        assert_eq!(color, egui::Color32::WHITE);
    }

    #[test]
    fn test_line_intersects_circle_endpoint_inside() {
        // 线段端点在圆内
        assert!(line_intersects_circle(0.0, 0.0, 10.0, 10.0, 5.0, 5.0, 8.0));
    }

    #[test]
    fn test_line_intersects_circle_no_intersection() {
        // 线段与圆不相交
        assert!(!line_intersects_circle(0.0, 0.0, 1.0, 1.0, 10.0, 10.0, 2.0));
    }

    #[test]
    fn test_line_intersects_circle_tangent() {
        // 线段与圆相切
        assert!(line_intersects_circle(0.0, 5.0, 10.0, 5.0, 5.0, 0.0, 5.0));
    }

    #[test]
    fn test_line_intersects_circle_passes_through() {
        // 线段穿过圆
        assert!(line_intersects_circle(0.0, 5.0, 10.0, 5.0, 5.0, 5.0, 3.0));
    }

    #[test]
    fn test_line_intersects_circle_short_line() {
        // 测试很短的线段（小于1像素）
        assert!(line_intersects_circle(5.0, 5.0, 5.1, 5.1, 5.0, 5.0, 2.0));
    }

    #[test]
    fn test_line_intersects_circle_zero_length() {
        // 零长度线段（点）
        assert!(line_intersects_circle(5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 1.0));
        assert!(!line_intersects_circle(5.0, 5.0, 5.0, 5.0, 10.0, 10.0, 1.0));
    }

    #[test]
    fn test_load_video_as_base64_with_temp_file() {
        // 创建临时文件
        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let test_data = b"fake video data for testing";
        temp_file.write_all(test_data).expect("Failed to write to temp file");
        
        let file_path = temp_file.path().to_str().unwrap();
        let result = load_video_as_base64(file_path);
        
        assert!(result.is_ok());
        let (base64_string, file_name, file_size) = result.unwrap();
        
        assert!(!base64_string.is_empty());
        assert!(!file_name.is_empty());
        assert_eq!(file_size, test_data.len() as u64);
        
        // 验证base64解码后的数据
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(&base64_string)
            .expect("Failed to decode base64");
        assert_eq!(decoded, test_data);
    }

    #[test]
    fn test_load_video_as_base64_nonexistent_file() {
        let result = load_video_as_base64("nonexistent_file.mp4");
        assert!(result.is_err());
    }

    // 注意：load_image_as_base64 测试需要实际的图片文件或mock，
    // 这里我们测试错误情况
    #[test]
    fn test_load_image_as_base64_nonexistent_file() {
        let result = load_image_as_base64("nonexistent_image.png");
        assert!(result.is_err());
    }

    #[test]
    fn test_geometric_calculations() {
        // 测试几何计算的边界情况
        
        // 圆心在原点，半径为1的圆
        let cx = 0.0;
        let cy = 0.0;
        let radius = 1.0;
        
        // 水平线段通过圆心
        assert!(line_intersects_circle(-2.0, 0.0, 2.0, 0.0, cx, cy, radius));
        
        // 垂直线段通过圆心
        assert!(line_intersects_circle(0.0, -2.0, 0.0, 2.0, cx, cy, radius));
        
        // 对角线通过圆心
        assert!(line_intersects_circle(-1.5, -1.5, 1.5, 1.5, cx, cy, radius));
        
        // 线段完全在圆外
        assert!(!line_intersects_circle(2.0, 2.0, 3.0, 3.0, cx, cy, radius));
    }

    #[test]
    fn test_color_edge_cases() {
        // 测试颜色值的边界情况
        
        // 最大值
        let max_color_json = json!([255, 255, 255, 255]);
        let max_color = color_from_json(&max_color_json);
        assert_eq!(max_color, egui::Color32::from_rgba_unmultiplied(255, 255, 255, 255));
        
        // 最小值
        let min_color_json = json!([0, 0, 0, 0]);
        let min_color = color_from_json(&min_color_json);
        assert_eq!(min_color, egui::Color32::from_rgba_unmultiplied(0, 0, 0, 0));
        
        // 包含null值
        let null_color_json = json!([255, null, 128, 64]);
        let null_color = color_from_json(&null_color_json);
        assert_eq!(null_color, egui::Color32::from_rgba_unmultiplied(255, 255, 128, 64));
    }
}