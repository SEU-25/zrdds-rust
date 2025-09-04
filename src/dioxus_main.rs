use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};

mod dioxus_structs;
mod dioxus_app;



use dioxus_app::DioxusApp;

fn main() {
    // 初始化日志
    env_logger::init();
    
    // 配置窗口
    let config = Config::new()
        .with_window(
            WindowBuilder::new()
                .with_title("ZRDDS Dioxus 应用")
                .with_inner_size(dioxus_desktop::LogicalSize::new(1200, 800))
                .with_resizable(true)
                .with_min_inner_size(dioxus_desktop::LogicalSize::new(800, 600))
        )
        .with_custom_head(r#"
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
        "#.to_string());
    
    // 启动应用
    LaunchBuilder::new()
        .with_cfg(config)
        .launch(DioxusApp);
}