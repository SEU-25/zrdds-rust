#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test;
    use std::sync::{Arc, Mutex};
    use crate::dioxus_structs::*;

    #[test]
    fn test_return_code_conversion() {
        use crate::core::return_code::*;
        
        assert_eq!(ReturnCode::Ok as i32, 0);
        assert_eq!(ReturnCode::Error as i32, 1);
    }

    #[test]
    fn test_chat_message_creation() {
        let chat_message = ChatMessage {
            username: "test_user".to_string(),
            message: "Hello World".to_string(),
            timestamp: "1234567890".to_string(),
            color: egui::Color32::from_rgb(255, 0, 0),
            is_private: false,
            target_user: None,
        };
        
        assert_eq!(chat_message.username, "test_user");
        assert_eq!(chat_message.message, "Hello World");
        assert_eq!(chat_message.timestamp, "1234567890");
        assert_eq!(chat_message.color, egui::Color32::from_rgb(255, 0, 0));
        assert_eq!(chat_message.is_private, false);
        assert_eq!(chat_message.target_user, None);
    }

    #[test]
    fn test_draw_stroke_validation() {
        let stroke = DrawStroke {
            username: "artist".to_string(),
            color: egui::Color32::RED,
            start_x: 10.0,
            start_y: 20.0,
            end_x: 30.0,
            end_y: 40.0,
            stroke_width: 2.0,
            is_dashed: false,
            dash_length: 5.0,
            gap_length: 3.0,
            timestamp: 1234567890,
        };
        
        assert!(stroke.stroke_width > 0.0);
        assert!(stroke.start_x >= 0.0);
        assert!(stroke.start_y >= 0.0);
    }

    #[test]
    fn test_mouse_state_creation() {
        let mouse_state = MouseState {
            username: "test_user".to_string(),
            color: egui::Color32::from_rgb(255, 0, 0),
            x: 100.0,
            y: 200.0,
        };
        
        assert_eq!(mouse_state.username, "test_user");
        assert_eq!(mouse_state.color, egui::Color32::from_rgb(255, 0, 0));
        assert_eq!(mouse_state.x, 100.0);
        assert_eq!(mouse_state.y, 200.0);
    }

    #[test]
    fn test_erase_operation_validation() {
        let erase_op = EraseOperation {
            username: "eraser".to_string(),
            x: 50.0,
            y: 75.0,
            radius: 10.0,
            timestamp: 1234567890,
        };
        
        assert!(erase_op.radius > 0.0);
        assert!(erase_op.x >= 0.0);
        assert!(erase_op.y >= 0.0);
    }
}