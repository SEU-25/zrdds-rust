use dioxus::prelude::*;
use std::collections::HashMap;
use crate::dioxus_structs::*;
use std::time::{SystemTime, UNIX_EPOCH};
use base64::{Engine as _, engine::general_purpose};
use rfd::FileDialog;
use std::fs;
use image::GenericImageView;

// 主应用组件
#[component]
pub fn DioxusApp() -> Element {
    // 应用状态
    let app_state = use_signal(|| DioxusAppState::default());
    let mut chat_messages = use_signal(|| Vec::<DioxusChatMessage>::new());
    let mut danmaku_messages = use_signal(|| Vec::<DioxusDanmakuMessage>::new());
    let mouse_positions = use_signal(|| HashMap::<String, DioxusMouseState>::new());
    let images = use_signal(|| HashMap::<String, DioxusImageData>::new());
    let videos = use_signal(|| HashMap::<String, DioxusVideoData>::new());
    let strokes = use_signal(|| Vec::<DioxusDrawStroke>::new());
    let mut local_strokes = use_signal(|| Vec::<DioxusDrawStroke>::new());
    let mut is_drawing = use_signal(|| false);
    let mut last_mouse_pos = use_signal(|| (0.0f32, 0.0f32));
    
    // 初始化DDS（如果需要）
    use_effect(move || {
        // 这里可以添加DDS初始化逻辑
        // init_dds_if_needed();
    });
    
    // 定期更新数据（模拟从DDS接收数据）
    use_future(move || async move {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_millis(16)).await;
            // 更新弹幕位置
            update_danmaku_positions(&mut danmaku_messages);
            
            // 这里可以添加从DDS接收数据的逻辑
            // update_from_dds(&mut chat_messages, &mut mouse_positions, &mut images, &mut videos, &mut strokes);
        }
    });
    
    // 防止未使用的变量警告
    let _unused_vars = (
        &mouse_positions, &images, &videos, &strokes, &local_strokes, 
        &is_drawing, &last_mouse_pos
    );
    
    rsx! {
        div {
            style: "display: flex; height: 100vh; font-family: Arial, sans-serif;",
            
            // 主要内容区域
            div {
                style: "flex: 1; display: flex;",
                
                // 中央面板
                CentralPanel { app_state }
                
                // 画布区域（保留原有功能）
                div {
                    style: "display: none;", // 暂时隐藏原有画布
                    Canvas {
                        app_state: app_state,
                        mouse_positions: mouse_positions,
                        images: images,
                        videos: videos,
                        strokes: strokes,
                        local_strokes: local_strokes,
                        danmaku_messages: danmaku_messages,
                        is_drawing: is_drawing,
                        last_mouse_pos: last_mouse_pos,
                        on_mouse_move: move |(x, y)| {
                            handle_mouse_move(x, y, &app_state, &mut is_drawing, &mut last_mouse_pos, &mut local_strokes);
                        },
                        on_mouse_down: move |(x, y)| {
                            handle_mouse_down(x, y, &app_state, &mut is_drawing, &mut last_mouse_pos);
                        },
                        on_mouse_up: move |_| {
                            handle_mouse_up(&mut is_drawing);
                        }
                    }
                }
            }
            
            // 右侧聊天面板
            ChatPanel {
                chat_messages: chat_messages,
                app_state: app_state,
                on_send_message: move |message| {
                    handle_send_message(message, &mut chat_messages, &mut danmaku_messages, &app_state);
                }
            }
        }
        
        // 弹幕层
        DanmakuOverlay {
            danmaku_messages: danmaku_messages
        }
    }
}

// 工具栏组件
#[component]
fn ToolBar(
    app_state: Signal<DioxusAppState>,
    on_image_upload: EventHandler<Vec<u8>>,
    on_video_upload: EventHandler<(Vec<u8>, String)>
) -> Element {
    let state = app_state.read();
    
    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 10px; padding: 10px; border-bottom: 1px solid #ccc; background-color: #f5f5f5;",
            
            // 颜色选择器
            div {
                style: "display: flex; align-items: center; gap: 5px;",
                label { "颜色: " }
                input {
                    r#type: "color",
                    value: format!("#{:02x}{:02x}{:02x}", state.current_color.r, state.current_color.g, state.current_color.b),
                    onchange: move |evt| {
                        if let Ok(color) = parse_hex_color(&evt.value()) {
                            app_state.write().current_color = color;
                        }
                    }
                }
            }
            
            // 模式切换按钮
            div {
                style: "display: flex; gap: 5px;",
                button {
                    style: format!("padding: 5px 10px; {}", 
                        if state.draw_mode == DrawMode::Mouse { "background-color: #007bff; color: white;" } else { "background-color: #e9ecef;" }
                    ),
                    onclick: move |_| {
                        app_state.write().draw_mode = DrawMode::Mouse;
                    },
                    "鼠标模式"
                }
                button {
                    style: format!("padding: 5px 10px; {}", 
                        if state.draw_mode == DrawMode::Draw { "background-color: #007bff; color: white;" } else { "background-color: #e9ecef;" }
                    ),
                    onclick: move |_| {
                        app_state.write().draw_mode = DrawMode::Draw;
                    },
                    "画笔模式"
                }
                button {
                    style: format!("padding: 5px 10px; {}", 
                        if state.draw_mode == DrawMode::Erase { "background-color: #007bff; color: white;" } else { "background-color: #e9ecef;" }
                    ),
                    onclick: move |_| {
                        app_state.write().draw_mode = DrawMode::Erase;
                    },
                    "擦除模式"
                }
            }
            
            // 笔刷大小
            div {
                style: "display: flex; align-items: center; gap: 5px;",
                label { "笔刷大小: " }
                input {
                    r#type: "range",
                    min: "1",
                    max: "20",
                    value: "{state.stroke_width}",
                    onchange: move |evt| {
                        if let Ok(width) = evt.value().parse::<f32>() {
                            app_state.write().stroke_width = width;
                        }
                    }
                }
                span { "{state.stroke_width}" }
            }
            
            // 文件上传按钮
            div {
                style: "display: flex; gap: 5px;",
                button {
                    style: "padding: 5px 10px; background-color: #28a745; color: white; border: none; border-radius: 3px;",
                    onclick: move |_| {
                        spawn(async move {
                            if let Some(file) = FileDialog::new()
                                .add_filter("图片", &["png", "jpg", "jpeg", "gif", "bmp"])
                                .pick_file() {
                                if let Ok(data) = fs::read(&file) {
                                    on_image_upload.call(data);
                                }
                            }
                        });
                    },
                    "上传图片"
                }
                button {
                    style: "padding: 5px 10px; background-color: #17a2b8; color: white; border: none; border-radius: 3px;",
                    onclick: move |_| {
                        spawn(async move {
                            if let Some(file) = FileDialog::new()
                                .add_filter("视频", &["mp4", "avi", "mov", "wmv", "flv"])
                                .pick_file() {
                                if let Ok(data) = fs::read(&file) {
                                    let file_name = file.file_name().unwrap_or_default().to_string_lossy().to_string();
                                    on_video_upload.call((data, file_name));
                                }
                            }
                        });
                    },
                    "上传视频"
                }
            }
        }
    }
}

// 画布组件
#[component]
fn Canvas(
    app_state: Signal<DioxusAppState>,
    mouse_positions: Signal<HashMap<String, DioxusMouseState>>,
    images: Signal<HashMap<String, DioxusImageData>>,
    videos: Signal<HashMap<String, DioxusVideoData>>,
    strokes: Signal<Vec<DioxusDrawStroke>>,
    local_strokes: Signal<Vec<DioxusDrawStroke>>,
    danmaku_messages: Signal<Vec<DioxusDanmakuMessage>>,
    is_drawing: Signal<bool>,
    last_mouse_pos: Signal<(f32, f32)>,
    on_mouse_move: EventHandler<(f32, f32)>,
    on_mouse_down: EventHandler<(f32, f32)>,
    on_mouse_up: EventHandler<()>
) -> Element {
    let state = app_state.read();
    
    rsx! {
        div {
            style: "flex: 1; position: relative; border: 2px solid #333; background-color: white; overflow: hidden;",
            width: "{state.canvas_width}px",
            height: "{state.canvas_height}px",
            
            // SVG画布用于绘制
            svg {
                style: "position: absolute; top: 0; left: 0; pointer-events: none;",
                width: "{state.canvas_width}",
                height: "{state.canvas_height}",
                
                // 渲染远程笔迹
                for stroke in strokes.read().iter() {
                    line {
                        x1: "{stroke.start_x}",
                        y1: "{stroke.start_y}",
                        x2: "{stroke.end_x}",
                        y2: "{stroke.end_y}",
                        stroke: format!("#{:02x}{:02x}{:02x}", stroke.color.r, stroke.color.g, stroke.color.b),
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
                        stroke: format!("#{:02x}{:02x}{:02x}", stroke.color.r, stroke.color.g, stroke.color.b),
                        stroke_width: "{stroke.stroke_width}",
                        stroke_linecap: "round"
                    }
                }
                
                // 渲染其他用户的鼠标位置
                for (username, mouse_state) in mouse_positions.read().iter() {
                    circle {
                        cx: "{mouse_state.x}",
                        cy: "{mouse_state.y}",
                        r: "5",
                        fill: format!("#{:02x}{:02x}{:02x}", mouse_state.color.r, mouse_state.color.g, mouse_state.color.b)
                    }
                    text {
                        x: "{mouse_state.x + 10.0}",
                        y: "{mouse_state.y - 10.0}",
                        font_size: "12",
                        fill: "black",
                        "{username}"
                    }
                }
            }
            
            // 显示图片
            for (_username, image_data) in images.read().iter() {
                div {
                    style: "position: absolute; top: 50%; left: 50%; transform: translate(-50%, -50%);",
                    img {
                        src: "data:image/png;base64,{image_data.base64_data}",
                        style: "max-width: 100%; max-height: 100%; object-fit: contain;",
                        alt: "用户上传的图片"
                    }
                    button {
                        style: "position: absolute; top: 5px; right: 5px; background-color: red; color: white; border: none; border-radius: 50%; width: 20px; height: 20px; cursor: pointer;",
                        onclick: move |_| {
                            // 处理图片删除
                            // handle_image_delete(username.clone());
                        },
                        "×"
                    }
                }
            }
            
            // 显示视频
            for (_username, video_data) in videos.read().iter() {
                div {
                    style: "position: absolute; top: 50%; left: 50%; transform: translate(-50%, -50%);",
                    video {
                        src: "{video_data.video_url}",
                        controls: true,
                        style: "max-width: 100%; max-height: 100%;"
                    }
                    button {
                        style: "position: absolute; top: 5px; right: 5px; background-color: red; color: white; border: none; border-radius: 50%; width: 20px; height: 20px; cursor: pointer;",
                        onclick: move |_| {
                            // 处理视频删除
                            // handle_video_delete(username.clone());
                        },
                        "×"
                    }
                }
            }
            
            // 弹幕层
            for danmaku in danmaku_messages.read().iter() {
                div {
                    style: format!(
                        "position: absolute; left: {}px; top: {}px; color: rgb({},{},{}); font-size: 14px; font-weight: bold; text-shadow: 1px 1px 1px rgba(0,0,0,0.5); pointer-events: none; white-space: nowrap;",
                        danmaku.x, danmaku.y, danmaku.color.r, danmaku.color.g, danmaku.color.b
                    ),
                    "{danmaku.message}"
                }
            }
            
            // 鼠标事件处理层
            div {
                style: "position: absolute; top: 0; left: 0; width: 100%; height: 100%; cursor: crosshair;",
                onmousemove: move |evt| {
                    let rect = evt.element_coordinates();
                    on_mouse_move.call((rect.x as f32, rect.y as f32));
                },
                onmousedown: move |evt| {
                    let rect = evt.element_coordinates();
                    on_mouse_down.call((rect.x as f32, rect.y as f32));
                },
                onmouseup: move |_| {
                    on_mouse_up.call(());
                }
            }
        }
    }
}

// 聊天面板组件
#[component]
fn ChatPanel(
    chat_messages: Signal<Vec<DioxusChatMessage>>,
    app_state: Signal<DioxusAppState>,
    on_send_message: EventHandler<String>
) -> Element {
    
    rsx! {
        div {
            style: "width: 300px; display: flex; flex-direction: column; border-left: 1px solid #ccc; background-color: #f9f9f9;",
            
            // 聊天标题和弹幕开关
            div {
                style: "padding: 10px; border-bottom: 1px solid #ccc; background-color: #e9ecef;",
                h3 { style: "margin: 0 0 10px 0;", "聊天室" }
                label {
                    style: "display: flex; align-items: center; gap: 5px;",
                    input {
                        r#type: "checkbox",
                        checked: app_state.read().danmaku_enabled,
                        onchange: move |evt| {
                            app_state.write().danmaku_enabled = evt.checked();
                        }
                    }
                    "启用弹幕"
                }
            }
            
            // 消息列表
            div {
                style: "flex: 1; overflow-y: auto; padding: 10px; max-height: 400px;",
                for message in chat_messages.read().iter() {
                    div {
                        style: "margin-bottom: 8px; padding: 8px; background-color: white; border-radius: 6px; border: 1px solid #ddd; box-shadow: 0 1px 3px rgba(0,0,0,0.1);",
                        div {
                            style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 4px;",
                            span {
                                style: "font-weight: bold; color: #007bff; font-size: 13px;",
                                "{message.username}"
                            }
                            span {
                                style: "font-size: 11px; color: #666;",
                                "{message.timestamp}"
                            }
                        }
                        div {
                            style: "color: #333; line-height: 1.4; font-size: 14px;",
                            "{message.message}"
                        }
                    }
                }
            }
            
            // 输入区域
            div {
                style: "padding: 10px; border-top: 1px solid #ccc;",
                div {
                    style: "display: flex; gap: 5px;",
                    input {
                        style: "flex: 1; padding: 8px; border: 1px solid #ddd; border-radius: 4px; font-size: 14px; outline: none; transition: border-color 0.2s;",
                        r#type: "text",
                        placeholder: "输入消息...",
                        value: app_state.read().chat_input.clone(),
                        oninput: move |evt| {
                            app_state.write().chat_input = evt.value();
                        },
                        onkeypress: move |evt| {
                            if evt.key() == Key::Enter {
                                let message = app_state.read().chat_input.clone();
                                if !message.trim().is_empty() {
                                    on_send_message.call(message);
                                    app_state.write().chat_input.clear();
                                }
                            }
                        }
                    }
                    button {
                        style: "padding: 8px 15px; background-color: #007bff; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 14px; transition: background-color 0.2s; hover:background-color: #0056b3;",
                        onclick: move |_| {
                            let message = app_state.read().chat_input.clone();
                            if !message.trim().is_empty() {
                                on_send_message.call(message);
                                app_state.write().chat_input.clear();
                            }
                        },
                        "发送"
                    }
                }
            }
        }
    }
}

// 辅助函数
fn parse_hex_color(hex: &str) -> Result<Color32, ()> {
    if hex.len() != 7 || !hex.starts_with('#') {
        return Err(());
    }
    
    let r = u8::from_str_radix(&hex[1..3], 16).map_err(|_| ())?;
    let g = u8::from_str_radix(&hex[3..5], 16).map_err(|_| ())?;
    let b = u8::from_str_radix(&hex[5..7], 16).map_err(|_| ())?;
    
    Ok(Color32::from_rgb(r, g, b))
}

fn update_danmaku_positions(danmaku_messages: &mut Signal<Vec<DioxusDanmakuMessage>>) {
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64();
    
    danmaku_messages.write().retain_mut(|danmaku| {
        let _elapsed = current_time - danmaku.start_time;
        danmaku.x -= danmaku.speed;
        
        // 如果弹幕移出屏幕左侧，则删除
        danmaku.x > -200.0
    });
}

fn handle_image_upload(image_data: Vec<u8>, images: &mut Signal<HashMap<String, DioxusImageData>>) {
    // 解码图片获取尺寸
    if let Ok(img) = image::load_from_memory(&image_data) {
        let (width, height) = img.dimensions();
        let base64_data = general_purpose::STANDARD.encode(&image_data);
        
        let image_data = DioxusImageData {
            username: "本地用户".to_string(), // 这里应该从配置或用户输入获取
            image_data: image_data.clone(),
            width,
            height,
            base64_data,
        };
        
        images.write().insert("本地用户".to_string(), image_data);
        
        // 这里应该通过DDS发送图片数据
        // send_image_via_dds(image_data);
    }
}

fn handle_video_upload(video_data: Vec<u8>, file_name: String, videos: &mut Signal<HashMap<String, DioxusVideoData>>) {
    // 创建临时文件URL（在实际应用中，你可能需要将视频保存到临时目录）
    let video_url = format!("data:video/mp4;base64,{}", general_purpose::STANDARD.encode(&video_data));
    
    let video_data_struct = DioxusVideoData {
        username: "本地用户".to_string(),
        video_data: video_data.clone(),
        file_name: file_name.clone(),
        file_size: video_data.len() as u64,
        video_url,
    };
    
    videos.write().insert("本地用户".to_string(), video_data_struct);
    
    // 这里应该通过DDS发送视频数据
    // send_video_via_dds(video_data, file_name);
}

fn handle_mouse_move(
    x: f32, 
    y: f32, 
    app_state: &Signal<DioxusAppState>, 
    is_drawing: &mut Signal<bool>, 
    last_mouse_pos: &mut Signal<(f32, f32)>, 
    local_strokes: &mut Signal<Vec<DioxusDrawStroke>>
) {
    let state = app_state.read();
    let (last_x, last_y) = last_mouse_pos.read().clone();
    
    // 发送鼠标位置（通过DDS）
    // send_mouse_position_via_dds(x, y, state.current_color);
    
    if *is_drawing.read() && state.draw_mode == DrawMode::Draw {
        // 创建新的笔迹
        let stroke = DioxusDrawStroke {
            username: "本地用户".to_string(),
            color: state.current_color,
            start_x: last_x,
            start_y: last_y,
            end_x: x,
            end_y: y,
            stroke_width: state.stroke_width,
        };
        
        local_strokes.write().push(stroke.clone());
        
        // 通过DDS发送笔迹数据
        // send_stroke_via_dds(stroke);
    }
    
    last_mouse_pos.set((x, y));
}

fn handle_mouse_down(
    x: f32, 
    y: f32, 
    app_state: &Signal<DioxusAppState>, 
    is_drawing: &mut Signal<bool>, 
    last_mouse_pos: &mut Signal<(f32, f32)>
) {
    let state = app_state.read();
    
    if state.draw_mode == DrawMode::Draw {
        is_drawing.set(true);
        last_mouse_pos.set((x, y));
    }
}

fn handle_mouse_up(is_drawing: &mut Signal<bool>) {
    is_drawing.set(false);
}

// 中央面板组件
#[component]
fn CentralPanel(app_state: Signal<DioxusAppState>) -> Element {
    rsx! {
        div {
            style: "flex: 1; padding: 20px; background: white; display: flex; flex-direction: column;",
            
            // 工具栏
            div {
                style: "background: #f8f9fa; padding: 15px; border-radius: 8px; margin-bottom: 20px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);",
                
                h3 { 
                    style: "margin: 0 0 15px 0; color: #333;",
                    "绘图工具" 
                }
                
                // 颜色选择器
                div {
                    style: "margin-bottom: 15px; display: flex; align-items: center; gap: 10px;",
                    label { 
                        style: "font-weight: 500; color: #555;",
                        "画笔颜色:" 
                    }
                    input {
                        style: "width: 50px; height: 35px; border: 2px solid #ddd; border-radius: 6px; cursor: pointer;",
                        r#type: "color",
                        value: format!("#{:02x}{:02x}{:02x}", 
                            app_state.read().current_color.r,
                            app_state.read().current_color.g,
                            app_state.read().current_color.b
                        ),
                        onchange: move |evt| {
                            let color_str = evt.value();
                            if let Ok(color) = parse_color(&color_str) {
                                let mut state = app_state.write();
                                state.current_color = color;
                            }
                        }
                    }
                    span {
                        style: "font-size: 12px; color: #666;",

                    div {
                        { format!(
                            "RGB({}, {}, {})", 
                            app_state.read().current_color.r,
                            app_state.read().current_color.g,
                            app_state.read().current_color.b
                        ) }
                    }

                    }
                }
                
                // 模式切换按钮
                div {
                    style: "margin-bottom: 15px;",
                    label { 
                        style: "font-weight: 500; color: #555; display: block; margin-bottom: 8px;",
                        "绘图模式:" 
                    }
                    div {
                        style: "display: flex; gap: 8px;",
                        button {
                            style: if app_state.read().draw_mode == DrawMode::Mouse {
                                "padding: 8px 16px; background: #007bff; color: white; border: none; border-radius: 6px; cursor: pointer; font-size: 14px; transition: all 0.2s; box-shadow: 0 2px 4px rgba(0,123,255,0.3);"
                            } else {
                                "padding: 8px 16px; background: white; color: #333; border: 2px solid #ddd; border-radius: 6px; cursor: pointer; font-size: 14px; transition: all 0.2s;"
                            },
                            onclick: move |_| {
                                let mut state = app_state.write();
                                state.draw_mode = DrawMode::Mouse;
                            },
                            "🖱️ 鼠标"
                        }
                        button {
                            style: if app_state.read().draw_mode == DrawMode::Draw {
                                "padding: 8px 16px; background: #28a745; color: white; border: none; border-radius: 6px; cursor: pointer; font-size: 14px; transition: all 0.2s; box-shadow: 0 2px 4px rgba(40,167,69,0.3);"
                            } else {
                                "padding: 8px 16px; background: white; color: #333; border: 2px solid #ddd; border-radius: 6px; cursor: pointer; font-size: 14px; transition: all 0.2s;"
                            },
                            onclick: move |_| {
                                let mut state = app_state.write();
                                state.draw_mode = DrawMode::Draw;
                            },
                            "✏️ 画笔"
                        }
                        button {
                            style: if app_state.read().draw_mode == DrawMode::Erase {
                                "padding: 8px 16px; background: #dc3545; color: white; border: none; border-radius: 6px; cursor: pointer; font-size: 14px; transition: all 0.2s; box-shadow: 0 2px 4px rgba(220,53,69,0.3);"
                            } else {
                                "padding: 8px 16px; background: white; color: #333; border: 2px solid #ddd; border-radius: 6px; cursor: pointer; font-size: 14px; transition: all 0.2s;"
                            },
                            onclick: move |_| {
                                let mut state = app_state.write();
                                state.draw_mode = DrawMode::Erase;
                            },
                            "🧹 擦除"
                        }
                    }
                }
                
                // 媒体上传按钮
                div {
                    label { 
                        style: "font-weight: 500; color: #555; display: block; margin-bottom: 8px;",
                        "媒体上传:" 
                    }
                    div {
                        style: "display: flex; gap: 8px;",
                        button {
                            style: "padding: 8px 16px; background: #17a2b8; color: white; border: none; border-radius: 6px; cursor: pointer; font-size: 14px; transition: all 0.2s;",
                            onclick: move |_| {
                                upload_image();
                            },
                            "📷 上传图片"
                        }
                        button {
                            style: "padding: 8px 16px; background: #6f42c1; color: white; border: none; border-radius: 6px; cursor: pointer; font-size: 14px; transition: all 0.2s;",
                            onclick: move |_| {
                                upload_video();
                            },
                            "🎥 上传视频"
                        }
                    }
                }
            }
            
            // 画布区域
            div {
                style: "flex: 1; border: 2px solid #ddd; border-radius: 8px; background: #fafafa; display: flex; align-items: center; justify-content: center; color: #666; font-size: 18px; min-height: 400px;",
                "🎨 画布区域 (待实现)"
            }
        }
    }
}

// 辅助函数
fn parse_color(hex: &str) -> Result<Color32, ()> {
    if hex.len() != 7 || !hex.starts_with('#') {
        return Err(());
    }
    
    let r = u8::from_str_radix(&hex[1..3], 16).map_err(|_| ())?;
    let g = u8::from_str_radix(&hex[3..5], 16).map_err(|_| ())?;
    let b = u8::from_str_radix(&hex[5..7], 16).map_err(|_| ())?;
    
    Ok(Color32::from_rgb(r, g, b))
}

fn upload_image() {
    spawn(async move {
        if let Some(file) = FileDialog::new()
            .add_filter("图片", &["png", "jpg", "jpeg", "gif", "bmp"])
            .pick_file() {
            if let Ok(_data) = fs::read(&file) {
                // TODO: 处理图片上传
                println!("图片上传: {:?}", file);
            }
        }
    });
}

fn upload_video() {
    spawn(async move {
        if let Some(file) = FileDialog::new()
            .add_filter("视频", &["mp4", "avi", "mov", "wmv", "flv"])
            .pick_file() {
            if let Ok(_data) = fs::read(&file) {
                // TODO: 处理视频上传
                println!("视频上传: {:?}", file);
            }
        }
    });
}

// 弹幕组件
#[component]
fn DanmakuOverlay(danmaku_messages: Signal<Vec<DioxusDanmakuMessage>>) -> Element {
    static CSS: Asset = asset!("/css/app.css");
    rsx! {
        document::Stylesheet { href: CSS }
        div {
            style: "position: fixed; top: 0; left: 0; width: 100%; height: 100%; \
                    pointer-events: none; z-index: 1000; overflow: hidden;",

            // 直接 for 循环，body 是 RSX
            for message in danmaku_messages.read().iter() {
                div {  // 建议加 key
                    class: "DanmakuOverlayItem",
                    "{ message.message }",
                }
            }
        }

    }
}



fn send_chat_message(
    message: String, 
    chat_messages: &mut Signal<Vec<DioxusChatMessage>>,
    danmaku_messages: &mut Signal<Vec<DioxusDanmakuMessage>>,
    danmaku_enabled: bool
) {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    let formatted_time = format!("{:02}:{:02}", 
        (timestamp / 60) % 60, 
        timestamp % 60
    );
    
    let chat_msg = DioxusChatMessage {
        username: "用户".to_string(),
        message: message.clone(),
        timestamp: formatted_time,
    };
    
    chat_messages.write().push(chat_msg);
    
    // 如果启用弹幕，添加弹幕消息
    if danmaku_enabled {
        add_danmaku_message(message.clone(), danmaku_messages);
    }
    
    // TODO: 通过DDS发送消息
    println!("发送聊天消息: {}", message);
}

fn add_danmaku_message(text: String, danmaku_messages: &mut Signal<Vec<DioxusDanmakuMessage>>) {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64();
    let danmaku_id = format!("danmaku-{}", current_time);
    
    let danmaku_msg = DioxusDanmakuMessage {
        username: "用户".to_string(),
        message: text,
        x: 1200.0, // 从右侧开始
        y: rng.gen_range(50.0..600.0), // 随机Y位置
        color: Color32::from_rgb(
            rng.gen_range(100..255),
            rng.gen_range(100..255), 
            rng.gen_range(100..255)
        ),
        speed: rng.gen_range(80.0..120.0),
        start_time: current_time,
        id: danmaku_id,
    };
    
    danmaku_messages.write().push(danmaku_msg);
}

fn handle_send_message(
    message: String, 
    chat_messages: &mut Signal<Vec<DioxusChatMessage>>, 
    danmaku_messages: &mut Signal<Vec<DioxusDanmakuMessage>>, 
    app_state: &Signal<DioxusAppState>
) {
    let state = app_state.read();
    send_chat_message(message, chat_messages, danmaku_messages, state.danmaku_enabled);
}