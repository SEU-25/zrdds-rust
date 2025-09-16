#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::sync::{Arc, Mutex};
    use crate::dioxus_structs::*;
    use crate::dds_handlers::{parse_color32_from_json, parse_danmaku_message};

    #[test]
    fn test_parse_color32_from_json_rgb() {
        // 测试RGB颜色解析
        let rgb_json = json!([255, 128, 64]);
        let color = parse_color32_from_json(&rgb_json);
        assert_eq!(color, egui::Color32::from_rgb(255, 128, 64));
    }

    #[test]
    fn test_parse_color32_from_json_rgba() {
        // 测试RGBA颜色解析
        let rgba_json = json!([255, 128, 64, 200]);
        let color = parse_color32_from_json(&rgba_json);
        assert_eq!(color, egui::Color32::from_rgba_unmultiplied(255, 128, 64, 200));
    }

    #[test]
    fn test_parse_color32_from_json_empty() {
        // 测试空数组
        let empty_json = json!([]);
        let color = parse_color32_from_json(&empty_json);
        assert_eq!(color, egui::Color32::from_rgb(0, 0, 0));
    }

    #[test]
    fn test_parse_color32_from_json_invalid() {
        // 测试无效JSON
        let invalid_json = json!("not_an_array");
        let color = parse_color32_from_json(&invalid_json);
        assert_eq!(color, egui::Color32::from_rgb(0, 0, 0));
    }

    #[test]
    fn test_draw_message_json_structure() {
        let draw_json = json!({
            "type": "Draw",
            "username": "test_user",
            "color": [255, 0, 0],
            "start_x": 10.0,
            "start_y": 20.0,
            "end_x": 30.0,
            "end_y": 40.0,
            "stroke_width": 3.0,
            "is_dashed": true,
            "dash_length": 5.0,
            "gap_length": 2.0,
            "timestamp": 1234567890
        });
        
        // 验证JSON结构
        assert_eq!(draw_json["type"].as_str().unwrap(), "Draw");
        assert_eq!(draw_json["username"].as_str().unwrap(), "test_user");
        assert_eq!(draw_json["stroke_width"].as_f64().unwrap(), 3.0);
        assert_eq!(draw_json["is_dashed"].as_bool().unwrap(), true);
    }

    #[test]
    fn test_user_color_message_json_structure() {
        let color_json = json!({
            "type": "UserColor",
            "username": "test_user",
            "color": {
                "r": 255,
                "g": 128,
                "b": 64,
                "a": 255
            }
        });
        
        assert_eq!(color_json["type"].as_str().unwrap(), "UserColor");
        assert_eq!(color_json["username"].as_str().unwrap(), "test_user");
        assert_eq!(color_json["color"]["r"].as_u64().unwrap(), 255);
        assert_eq!(color_json["color"]["g"].as_u64().unwrap(), 128);
    }

    #[test]
    fn test_chat_message_json_structure() {
        let chat_json = json!({
            "type": "Chat",
            "username": "test_user",
            "message": "Hello World",
            "timestamp": 1234567890,
            "color": [255, 255, 255, 255],
            "is_private": false
        });
        
        assert_eq!(chat_json["type"].as_str().unwrap(), "Chat");
        assert_eq!(chat_json["username"].as_str().unwrap(), "test_user");
        assert_eq!(chat_json["message"].as_str().unwrap(), "Hello World");
        assert_eq!(chat_json["is_private"].as_bool().unwrap(), false);
    }

    #[test]
    fn test_danmaku_message_parsing() {
        let danmaku_json = json!({
            "type": "Danmaku",
            "username": "test_user",
            "message": "弹幕消息",
            "color": [255, 100, 50],
            "x": 100.0,
            "y": 200.0,
            "speed": 50.0,
            "start_time": 1234567890.0
        });
        
        let danmaku_msg = parse_danmaku_message(&danmaku_json);
        assert!(danmaku_msg.is_some());
        
        let msg = danmaku_msg.unwrap();
        assert_eq!(msg.username, "test_user");
        assert_eq!(msg.message, "弹幕消息");
        assert_eq!(msg.x, 100.0);
        assert_eq!(msg.y, 200.0);
        assert_eq!(msg.speed, 50.0);
        assert_eq!(msg.start_time, 1234567890.0);
        assert_eq!(msg.color, egui::Color32::from_rgb(255, 100, 50));
    }

    #[test]
    fn test_danmaku_message_parsing_invalid() {
        let invalid_json = json!({
            "type": "NotDanmaku",
            "username": "test_user",
            "message": "消息"
        });
        
        let danmaku_msg = parse_danmaku_message(&invalid_json);
        assert!(danmaku_msg.is_none());
    }

    #[test]
    fn test_erase_message_json_structure() {
        let erase_json = json!({
            "type": "Erase",
            "username": "test_user",
            "x": 50.0,
            "y": 75.0,
            "radius": 10.0,
            "timestamp": 1234567890
        });
        
        assert_eq!(erase_json["type"].as_str().unwrap(), "Erase");
        assert_eq!(erase_json["x"].as_f64().unwrap(), 50.0);
        assert_eq!(erase_json["y"].as_f64().unwrap(), 75.0);
        assert_eq!(erase_json["radius"].as_f64().unwrap(), 10.0);
    }
}